/*
 * Copyright (c) 2024-2025 Pavel Vasin
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
#include <concepts>
#include <map>
#include <span>
#include <stdexcept>
#include <type_traits>
#include <utility>
#include <vector>

#include "matrixsparse.h"

namespace blacknet::crypto {

template<typename E>class CustomizableConstraintSystem;
template<typename E>class R1CS;

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

        constexpr decltype(auto) begin() const noexcept {
            return terms.begin();
        }

        constexpr decltype(auto) end() const noexcept {
            return terms.end();
        }

        template<class... Args>
        decltype(auto) emplace(Args&&... args) {
            return terms.emplace(std::forward<Args>(args)...);
        }

        constexpr LinearCombination() = default;
        constexpr LinearCombination(const E& coefficient) : terms({std::make_pair(Variable::constant(), coefficient)}) {}
        constexpr LinearCombination(const Variable& variable) : terms({std::make_pair(variable, E(1))}) {}
        constexpr LinearCombination(const LinearCombination&) = default;
        constexpr LinearCombination(LinearCombination&&) noexcept = default;
        constexpr ~LinearCombination() noexcept = default;

        constexpr LinearCombination& operator = (const LinearCombination&) = default;
        constexpr LinearCombination& operator = (LinearCombination&&) noexcept = default;

        constexpr LinearCombination& operator = (const E& coefficient) {
            const Variable variable(Variable::constant());
            terms = { std::make_pair(variable, coefficient) };
            return *this;
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

        friend constexpr LinearCombination operator * (const E& l, const LinearCombination& r) {
            LinearCombination t;
            for (const auto& [variable, coefficient] : r.terms)
                t.emplace(variable, l * coefficient);
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

        friend constexpr LinearCombination operator + (const E& l, const LinearCombination& r) {
            LinearCombination t;
            for (const auto& [variable, coefficient] : r.terms)
                t.emplace(variable, l + coefficient);
            return t;
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

        constexpr LinearCombination& operator -= (const std::pair<Variable, E>& term) {
            const auto& [variable, coefficient] = term;
            if (auto [iterator, inserted] = terms.emplace(variable, -coefficient); !inserted)
                iterator->second -= coefficient;
            return *this;
        }

        constexpr LinearCombination& operator -= (const LinearCombination& lc) {
            for (const auto& term : lc)
                (*this) -= term;
            return *this;
        }

        constexpr LinearCombination operator - (const LinearCombination& lc) const {
            LinearCombination t(*this);
            t -= lc;
            return t;
        }

        constexpr LinearCombination operator - () const {
            LinearCombination t;
            for (const auto& [variable, coefficient] : terms)
                t.emplace(variable, -coefficient);
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

        constexpr Constant() = default;
        constexpr Constant(const E& value) : value(value) {}

        constexpr Constraint operator () () const {
            static_assert(false, "Constant is not a constraint");
        }

        constexpr void operator () (std::span<LinearCombination>) const {
            static_assert(false, "Constant is not a combination");
        }

        constexpr void operator () (LinearCombination& lc) const {
            lc += value;
        }

        consteval static std::size_t degree() {
            return 0;
        }
    };

    struct Variable : ConstraintExpression<Variable> {
        enum Type { Uninitialized, Constant, Input, Auxiliary };
        Type type;
        std::size_t number;

        consteval Variable() noexcept : type(Uninitialized), number(-1) {}
        constexpr Variable(Type type, std::size_t number) : type(type), number(number) {}
        consteval static Variable constant() { return Variable(Constant, 0); }

        constexpr Constraint operator () () const {
            static_assert(false, "Variable is not a constraint");
        }

        constexpr void operator () (std::span<LinearCombination>) const {
            static_assert(false, "Variable is not a combination");
        }

        constexpr void operator () (LinearCombination& lc) const {
            lc += *this;
        }

        consteval static std::size_t degree() {
            return 1;
        }

        constexpr bool operator < (const Variable& other) const {
            if (type < other.type)
                return true;
            else if (other.type < type)
                return false;
            else
                return number < other.number;
        }
    };

    template<typename L, typename R>
    struct AddExpression : ConstraintExpression<AddExpression<L, R>> {
        L l;
        R r;

        constexpr AddExpression() = default;
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
            l(lc);
            r(lc);
        }

        consteval static std::size_t degree() {
            return std::max(L::degree(), R::degree());
        }
    };

    template<typename L, typename R>
    struct MulExpression : ConstraintExpression<MulExpression<L, R>> {
        L l;
        R r;

        constexpr MulExpression() = default;
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

        constexpr EqExpression() = default;
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
    requires(!(std::same_as<L, LinearCombination> && std::same_as<R, LinearCombination>))
    friend constexpr AddExpression<L, R> operator + (const ConstraintExpression<L>& l, const ConstraintExpression<R>& r) {
        return { static_cast<const L&>(l), static_cast<const R&>(r) };
    }

    template<typename L>
    requires(!std::same_as<L, LinearCombination>)
    friend constexpr AddExpression<L, Constant> operator + (const ConstraintExpression<L>& l, const E& r) {
        return { static_cast<const L&>(l), Constant(r) };
    }

    template<typename R>
    requires(!std::same_as<R, LinearCombination>)
    friend constexpr AddExpression<Constant, R> operator + (const E& l, const ConstraintExpression<R>& r) {
        return { Constant(l), static_cast<const R&>(r) };
    }

    template<typename L, typename R>
    friend constexpr MulExpression<L, R> operator * (const ConstraintExpression<L>& l, const ConstraintExpression<R>& r) {
        return { static_cast<const L&>(l), static_cast<const R&>(r) };
    }

    template<typename L>
    requires(!std::same_as<L, LinearCombination>)
    friend constexpr MulExpression<L, Constant> operator * (const ConstraintExpression<L>& l, const E& r) {
        return { static_cast<const L&>(l), Constant(r) };
    }

    template<typename R>
    requires(!std::same_as<R, LinearCombination>)
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

    struct ScopeInfo {
        ScopeInfo* const up;
        std::vector<ScopeInfo> down;
        const char* name;
        std::size_t constraints;
        std::size_t variables;

        constexpr ScopeInfo(ScopeInfo* const up, const char* name)
            : up(up), down(), name(name), constraints(0), variables(0) {}

        void print(std::ostream& out, std::size_t level) const {
            for (std::size_t i = 0; i < level; ++i)
                out << ' ';
            out << "- " << name << ' ' << constraints << 'x' << variables << std::endl;
            for (const auto& scope : down)
                scope.print(out, level + 1);
        }
    };

    class Scope {
        friend CCSBuilder;
        CCSBuilder* builder;

        constexpr Scope(CCSBuilder* builder) : builder(builder) {}
        constexpr Scope(const Scope&) = delete;
        constexpr Scope(Scope&&) = delete;
        constexpr Scope& operator = (const Scope&) = delete;
        constexpr Scope& operator = (Scope&&) = delete;
        public:
        constexpr ~Scope() {
            builder->currentScope = builder->currentScope->up;
        }
    };

    constexpr Scope scope(const char* name) {
        std::vector<ScopeInfo>* level;
        if (currentScope) {
            level = &currentScope->down;
        } else {
            level = &scopes;
        }
        currentScope = &level->emplace_back(currentScope, name);
        return Scope(this);
    }

    std::size_t inputs;
    std::size_t auxiliaries;
    std::vector<Constraint> constraints;
    std::vector<ScopeInfo> scopes;
    ScopeInfo* currentScope;

    consteval CCSBuilder() : inputs(0), auxiliaries(0), constraints(), scopes(), currentScope(nullptr) {}

    [[nodiscard("Circuit variable should be constrained")]]
    constexpr Variable input() {
        if (currentScope) currentScope->variables += 1;
        return { Variable::Type::Input, ++inputs };
    }

    [[nodiscard("Circuit variable should be constrained")]]
    constexpr Variable auxiliary() {
        if (currentScope) currentScope->variables += 1;
        return { Variable::Type::Auxiliary, ++auxiliaries };
    }

    [[nodiscard("Circuit variable should be constrained")]]
    constexpr Variable variable(Variable::Type type) {
        switch (type) {
            case Variable::Type::Constant:
                throw std::runtime_error("New constant variable requested");
            case Variable::Type::Input:
                return input();
            case Variable::Type::Auxiliary:
                return auxiliary();
            case Variable::Type::Uninitialized:
                throw std::runtime_error("New uninitialized variable requested");
        }
        std::unreachable();
    }

    template<typename T>
    constexpr void operator () (const ConstraintExpression<T>& expression) {
        if (currentScope) currentScope->constraints += 1;
        constraints.emplace_back(expression());
    }

    constexpr std::size_t variables() const {
        return 1 + inputs + auxiliaries;
    }

    constexpr R1CS<E> r1cs() const {
        static_assert(degree() <= 2, "High-degree circuits are not supported");
        MatrixSparse<E> a(constraints.size(), variables());
        MatrixSparse<E> b(constraints.size(), variables());
        MatrixSparse<E> c(constraints.size(), variables());
        for (std::size_t i = 0; i < constraints.size(); ++i) {
            put(a, constraints[i].r[0]);
            put(b, constraints[i].r[1]);
            put(c, constraints[i].l);
        }
        return { std::move(a), std::move(b), std::move(c) };
    }

    constexpr CustomizableConstraintSystem<E> ccs() const {
        std::vector<MatrixSparse<E>> ms;
        ms.reserve(degree() + 1);
        std::ranges::generate_n(std::back_inserter(ms), degree() + 1, [&]{ return MatrixSparse<E>(constraints.size(), variables()); });
        for (std::size_t i = 0; i < constraints.size(); ++i) {
            for (std::size_t j = 0; j < constraints[i].r.size(); ++j) {
                put(ms[j], constraints[i].r[j]);
            }
            put(ms.back(), constraints[i].l);
        }
        std::vector<std::vector<std::size_t>> s(2);
        s[0].reserve(degree());
        for (std::size_t i = 0; i < degree(); ++i)
            s[0].push_back(i);
        s[1].reserve(1);
        s[1].push_back(degree());
        return {
            constraints.size(), variables(),
            std::move(ms),
            std::move(s),
            { E(1), E(-1) }
        };
    }

    constexpr void put(MatrixSparse<E>& m, const LinearCombination& lc) const {
        for (const auto& [variable, coefficient] : lc) {
            std::size_t column;
            switch (variable.type) {
                case Variable::Type::Constant:
                    column = 0;
                    break;
                case Variable::Type::Input:
                    column = variable.number;
                    break;
                case Variable::Type::Auxiliary:
                    column = inputs + variable.number;
                    break;
                case Variable::Type::Uninitialized:
                    throw std::runtime_error("Uninitialized variable in circuit");
            }
            m.cIndex.push_back(column);
            m.elements.push_back(coefficient);
        }
        m.rIndex.push_back(m.elements.size());
    }

    void print(std::ostream& out) const {
        out << "Circuit " << constraints.size() << 'x' << variables() << std::endl;
        for (const auto& scope : scopes)
            scope.print(out, 0);
    }
};

template<typename E>
using R1CSBuilder = CCSBuilder<E, 2>;

}

#endif
