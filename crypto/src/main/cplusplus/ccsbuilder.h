/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef BLACKNET_CRYPTO_CCSBUILDER_H
#define BLACKNET_CRYPTO_CCSBUILDER_H

#include <algorithm>
#include <array>
#include <map>
#include <span>
#include <type_traits>
#include <vector>

#include "customizableconstraintsystem.h"
#include "matrix.h"
#include "matrixsparse.h"
#include "r1cs.h"

template<typename E, std::size_t D>
struct CCSBuilder {
    using R = E;

    consteval static std::size_t degree() {
        return D;
    }

    struct Variable;
    struct LinearCombination;
    struct Combination;
    struct Constraint;

    template<typename T>
    struct ConstraintExpression {
        constexpr Constraint operator () () const {
            return static_cast<const T&>(*this)();
        }

        constexpr void operator () (std::span<LinearCombination> combinations) const {
            return static_cast<const T&>(*this)(combinations);
        }

        constexpr void operator () (LinearCombination& lc) const {
            return static_cast<const T&>(*this)(lc);
        }

        consteval static std::size_t degree() {
            return T::degree();
        }
    };

    struct LinearCombination : ConstraintExpression<LinearCombination> {
        std::map<Variable, E> terms;

        decltype(auto) begin() const noexcept {
            return terms.begin();
        }

        decltype(auto) end() const noexcept {
            return terms.end();
        }

        template<class... Args>
        decltype(auto) emplace(Args&&... args) {
            return terms.emplace(std::forward<Args>(args)...);
        }

        constexpr LinearCombination& operator = (const Variable& variable) {
            const E coefficient(1);
            terms = { std::make_pair(variable, coefficient) };
            return *this;
        }

        constexpr LinearCombination& operator *= (const E& e) {
            for (auto& [_, coefficient] : terms)
                coefficient *= e;
            return *this;
        }

        constexpr LinearCombination operator * (const E& e) const {
            LinearCombination t;
            for (const auto& [variable, coefficient] : terms)
                t.emplace(variable, coefficient * e);
            return t;
        }

        constexpr LinearCombination& operator += (const std::pair<Variable, E>& term) {
            const auto& [variable, coefficient] = term;
            if (auto [iterator, inserted] = terms.emplace(variable, coefficient); !inserted)
                iterator->second += coefficient;
            return *this;
        }

        constexpr LinearCombination& operator += (const E& coefficient) {
            const Variable variable(Variable::constant());
            return (*this) += std::make_pair(variable, coefficient);
        }

        constexpr LinearCombination& operator += (const Variable& variable) {
            const E coefficient(1);
            return (*this) += std::make_pair(variable, coefficient);
        }

        constexpr LinearCombination& operator += (const LinearCombination& lc) {
            for (const auto& term : lc)
                (*this) += term;
            return *this;
        }

        constexpr LinearCombination operator + (const LinearCombination& lc) const {
            LinearCombination t(*this);
            t += lc;
            return t;
        }

        constexpr Constraint operator () () const {
            static_assert(false, "Linear combination is not a constraint");
        }

        constexpr void operator () (std::span<LinearCombination> combinations) const {
            (*this)(combinations[0]);
            for (std::size_t i = 1; i < combinations.size(); ++i)
                combinations[i].emplace(Variable::constant(), E(1));
        }

        constexpr void operator () (LinearCombination& lc) const {
            lc += *this;
        }

        consteval static std::size_t degree() {
            return 1;
        }
    };

    struct Combination {
        std::array<LinearCombination, D> lcs;

        constexpr std::size_t size() const noexcept {
            return lcs.size();
        }

        constexpr LinearCombination& operator [] (std::size_t i) {
            return lcs[i];
        }

        constexpr const LinearCombination& operator [] (std::size_t i) const {
            return lcs[i];
        }

        constexpr std::span<LinearCombination, D> span() {
            return { lcs };
        }
    };

    struct Constraint {
        Combination r;
        LinearCombination l;
    };

    struct Constant : ConstraintExpression<Constant> {
        E value;

        constexpr Constant() {}
        constexpr Constant(const E& value) : value(value) {}

        constexpr Constraint operator () () const {
            static_assert(false, "Constant is not a constraint");
        }

        constexpr void operator () (std::span<LinearCombination>) const {
            static_assert(false, "Constant is not a combination");
        }

        constexpr void operator () (LinearCombination&) const {
            static_assert(false, "Constant is not a linear combination");
        }

        consteval static std::size_t degree() {
            return 0;
        }
    };

    struct Variable : ConstraintExpression<Variable> {
        enum Type { Uninitialized, Constant, Input, Auxiliary };
        Type type;
        std::size_t number;

        consteval Variable() : type(Uninitialized), number(-1) {}
        constexpr Variable(Type type, std::size_t number) : type(type), number(number) {}
        consteval static Variable constant() { return Variable(Constant, 0); }

        constexpr Constraint operator () () const {
            static_assert(false, "Variable is not a constraint");
        }

        constexpr void operator () (std::span<LinearCombination>) const {
            static_assert(false, "Variable is not a combination");
        }

        constexpr void operator () (LinearCombination&) const {
            static_assert(false, "Variable is not a linear combination");
        }

        consteval static std::size_t degree() {
            return 1;
        }

        constexpr bool operator < (const Variable& other) const {
            if (type < other.type)
                return true;
            return number < other.number;
        }
    };

    template<typename L, typename R>
    struct AddExpression : ConstraintExpression<AddExpression<L, R>> {
        L l;
        R r;

        constexpr AddExpression() {}
        constexpr AddExpression(const L& l, const R& r) : l(l), r(r) {}

        constexpr Constraint operator () () const {
            static_assert(false, "Addition is not a constraint");
        }

        constexpr void operator () (std::span<LinearCombination> combinations) const {
            static_assert(L::degree() <= 1 && R::degree() <= 1, "Can't add non-linear expressions");
            (*this)(combinations[0]);
            for (std::size_t i = 1; i < combinations.size(); ++i)
                combinations[i].emplace(Variable::constant(), E(1));
        }

        constexpr void operator () (LinearCombination& lc) const {
            static_assert(L::degree() <= 1 && R::degree() <= 1, "Can't add non-linear expressions");
            if constexpr (std::is_same_v<L, Constant>) {
                lc += l.value;
            } else if constexpr (std::is_same_v<L, Variable>) {
                lc += l;
            } else {
                l(lc);
            }
            if constexpr (std::is_same_v<R, Constant>) {
                lc += r.value;
            } else if constexpr (std::is_same_v<R, Variable>) {
                lc += r;
            } else {
                r(lc);
            }
        }

        consteval static std::size_t degree() {
            return std::max(L::degree(), R::degree());
        }
    };

    template<typename L, typename R>
    struct MulExpression : ConstraintExpression<MulExpression<L, R>> {
        L l;
        R r;

        constexpr MulExpression() {}
        constexpr MulExpression(const L& l, const R& r) : l(l), r(r) {}

        constexpr Constraint operator () () const {
            static_assert(false, "Multiplication is not a constraint");
        }

        template<std::size_t N>
        requires(N != std::dynamic_extent)
        constexpr void operator () (std::span<LinearCombination, N> combinations) const {
            static_assert(degree() <= combinations.size(), "Can't mul high-degree expressions");
            if constexpr (std::is_same_v<L, Constant> || std::is_same_v<R, Constant>) {
                (*this)(combinations[0]);
                for (std::size_t i = 1; i < combinations.size(); ++i)
                    combinations[i].emplace(Variable::constant(), E(1));
            } else if constexpr (std::is_same_v<L, Variable> && std::is_same_v<R, Variable>) {
                combinations[0].emplace(l, E(1));
                combinations[1].emplace(r, E(1));
                for (std::size_t i = 2; i < combinations.size(); ++i)
                    combinations[i].emplace(Variable::constant(), E(1));
            } else if constexpr (std::is_same_v<L, Variable>) {
                combinations[0].emplace(l, E(1));
                r(combinations.template subspan<1, R::degree()>());
                for (std::size_t i = degree(); i < combinations.size(); ++i)
                    combinations[i].emplace(Variable::constant(), E(1));
            } else if constexpr (std::is_same_v<R, Variable>) {
                l(combinations.template subspan<0, L::degree()>());
                combinations[L::degree()].emplace(r, E(1));
                for (std::size_t i = degree(); i < combinations.size(); ++i)
                    combinations[i].emplace(Variable::constant(), E(1));
            } else {
                l(combinations.template subspan<0, L::degree()>());
                r(combinations.template subspan<L::degree(), R::degree()>());
                for (std::size_t i = degree(); i < combinations.size(); ++i)
                    combinations[i].emplace(Variable::constant(), E(1));
            }
        }

        constexpr void operator () (LinearCombination& lc) const {
            if constexpr (std::is_same_v<L, Constant>) {
                if constexpr (std::is_same_v<R, Constant>) {
                    static_assert(false, "Not implemented");
                } else if constexpr (std::is_same_v<R, Variable>) {
                    lc += std::make_pair(r, l.value);
                } else if constexpr (R::degree() == 1) {
                    LinearCombination t;
                    r(t);
                    t *= l.value;
                    lc += t;
                } else {
                    static_assert(false, "Can't mul non-linear expressions");
                }
            } else if constexpr (std::is_same_v<R, Constant>) {
                if constexpr (std::is_same_v<L, Constant>) {
                    static_assert(false, "Not implemented");
                } else if constexpr (std::is_same_v<L, Variable>) {
                    lc += std::make_pair(l, r.value);
                } else if constexpr (L::degree() == 1) {
                    LinearCombination t;
                    l(t);
                    t *= r.value;
                    lc += t;
                } else {
                    static_assert(false, "Can't mul non-linear expressions");
                }
            } else {
                static_assert(false, "Can't mul non-constant expressions");
            }
        }

        consteval static std::size_t degree() {
            return L::degree() + R::degree();
        }
    };

    template<typename L, typename R>
    struct EqExpression : ConstraintExpression<EqExpression<L, R>> {
        L l;
        R r;

        constexpr EqExpression() {}
        constexpr EqExpression(const L& l, const R& r) : l(l), r(r) {}

        constexpr Constraint operator () () const {
            static_assert(degree() <= CCSBuilder::degree(), "High-degree constraints are not supported");
            Constraint constraint;
            if constexpr (std::is_same_v<L, Constant>) {
                if constexpr (std::is_same_v<R, Constant>) {
                    static_assert(false, "Not implemented");
                } else if constexpr (std::is_same_v<R, Variable>) {
                    constraint.r[0].emplace(Variable::constant(), l.value);
                    constraint.r[0].emplace(r, E(-1));
                    for (std::size_t i = 1; i < constraint.r.size(); ++i)
                        constraint.r[i].emplace(Variable::constant(), E(1));
                } else {
                    constraint.l.emplace(Variable::constant(), l.value);
                    r(constraint.r.span());
                }
            } else if constexpr (std::is_same_v<L, Variable>) {
                if constexpr (std::is_same_v<R, Constant>) {
                    constraint.r[0].emplace(l, E(-1));
                    constraint.r[0].emplace(Variable::constant(), r.value);
                    for (std::size_t i = 1; i < constraint.r.size(); ++i)
                        constraint.r[i].emplace(Variable::constant(), E(1));
                } else if constexpr (std::is_same_v<R, Variable>) {
                    constraint.r[0].emplace(l, E(1));
                    constraint.r[0].emplace(r, E(-1));
                    for (std::size_t i = 1; i < constraint.r.size(); ++i)
                        constraint.r[i].emplace(Variable::constant(), E(1));
                } else {
                    constraint.l.emplace(l, E(1));
                    r(constraint.r.span());
                }
            } else if constexpr (L::degree() == 1) {
                l(constraint.l);
                r(constraint.r.span());
            } else {
                static_assert(false, "Not implemented");
            }
            return constraint;
        }

        constexpr void operator () (Combination&) const {
            static_assert(false, "Equality is not a combination");
        }

        constexpr void operator () (LinearCombination&) const {
            static_assert(false, "Equality is not a linear combination");
        }

        consteval static std::size_t degree() {
            return std::max(L::degree(), R::degree());
        }
    };

    template<typename L, typename R>
    friend constexpr AddExpression<L, R> operator + (const ConstraintExpression<L>& l, const ConstraintExpression<R>& r) {
        return { static_cast<const L&>(l), static_cast<const R&>(r) };
    }

    template<typename L>
    friend constexpr AddExpression<L, Constant> operator + (const ConstraintExpression<L>& l, const E& r) {
        return { static_cast<const L&>(l), Constant(r) };
    }

    template<typename R>
    friend constexpr AddExpression<Constant, R> operator + (const E& l, const ConstraintExpression<R>& r) {
        return { Constant(l), static_cast<const R&>(r) };
    }

    template<typename L, typename R>
    friend constexpr MulExpression<L, R> operator * (const ConstraintExpression<L>& l, const ConstraintExpression<R>& r) {
        return { static_cast<const L&>(l), static_cast<const R&>(r) };
    }

    template<typename L>
    friend constexpr MulExpression<L, Constant> operator * (const ConstraintExpression<L>& l, const E& r) {
        return { static_cast<const L&>(l), Constant(r) };
    }

    template<typename R>
    friend constexpr MulExpression<Constant, R> operator * (const E& l, const ConstraintExpression<R>& r) {
        return { Constant(l), static_cast<const R&>(r) };
    }

    template<typename L, typename R>
    friend constexpr EqExpression<L, R> operator == (const ConstraintExpression<L>& l, const ConstraintExpression<R>& r) {
        return { static_cast<const L&>(l), static_cast<const R&>(r) };
    }

    template<typename L>
    friend constexpr EqExpression<L, Constant> operator == (const ConstraintExpression<L>& l, const E& r) {
        return { static_cast<const L&>(l), Constant(r) };
    }

    template<typename R>
    friend constexpr EqExpression<Constant, R> operator == (const E& l, const ConstraintExpression<R>& r) {
        return { Constant(l), static_cast<const R&>(r) };
    }

    std::size_t inputs;
    std::size_t auxiliaries;
    std::vector<Constraint> constraints;

    consteval CCSBuilder() : inputs(0), auxiliaries(0), constraints() {}

    constexpr Variable input() {
        return { Variable::Type::Input, ++inputs };
    }

    constexpr Variable auxiliary() {
        return { Variable::Type::Auxiliary, ++auxiliaries };
    }

    template<typename T>
    constexpr void operator () (const ConstraintExpression<T>& expression) {
        constraints.emplace_back(expression());
    }

    constexpr R1CS<E> r1cs() const {
        static_assert(degree() <= 2, "High-degree circuits are not supported");
        std::size_t variables = 1 + inputs + auxiliaries;
        Matrix<E> a(constraints.size(), variables, E(0));
        Matrix<E> b(constraints.size(), variables, E(0));
        Matrix<E> c(constraints.size(), variables, E(0));
        for (std::size_t i = 0; i < constraints.size(); ++i) {
            put(a, i, constraints[i].r[0]);
            put(b, i, constraints[i].r[1]);
            put(c, i, constraints[i].l);
        }
        return { MatrixSparse(a), MatrixSparse(b), MatrixSparse(c) };
    }

    constexpr CustomizableConstraintSystem<E> ccs() const {
        std::size_t variables = 1 + inputs + auxiliaries;
        std::vector<Matrix<E>> md;
        md.reserve(degree() + 1);
        std::ranges::generate_n(std::back_inserter(md), degree() + 1, [&]{ return Matrix<E>(constraints.size(), variables, E(0)); });
        for (std::size_t i = 0; i < constraints.size(); ++i) {
            for (std::size_t j = 0; j < constraints[i].r.size(); ++j) {
                put(md[j], i, constraints[i].r[j]);
            }
            put(md.back(), i, constraints[i].l);
        }
        std::vector<MatrixSparse<E>> ms;
        ms.reserve(degree() + 1);
        std::ranges::transform(md, std::back_inserter(ms), [](auto&& ix) { return MatrixSparse<E>(ix); });
        std::vector<std::vector<std::size_t>> s(2);
        s[0].reserve(degree());
        for (std::size_t i = 0; i < degree(); ++i)
            s[0].push_back(i);
        s[1].reserve(1);
        s[1].push_back(degree());
        return {
            constraints.size(), variables,
            std::move(ms),
            std::move(s),
            { E(1), E(-1) }
        };
    }

    constexpr void put(Matrix<E>& m, std::size_t row, const LinearCombination& lc) const {
        for (const auto& [variable, coefficient] : lc) {
            if (variable.type == Variable::Type::Constant)
                m[row, 0] = coefficient;
            else if (variable.type == Variable::Type::Input)
                m[row, variable.number] = coefficient;
            else if (variable.type == Variable::Type::Auxiliary)
                m[row, inputs + variable.number] = coefficient;
        }
    }
};

template<typename E>
using R1CSBuilder = CCSBuilder<E, 2>;

#endif
