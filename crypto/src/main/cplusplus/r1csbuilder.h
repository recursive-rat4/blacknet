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

#ifndef BLACKNET_CRYPTO_R1CSBUILDER_H
#define BLACKNET_CRYPTO_R1CSBUILDER_H

#include <map>
#include <type_traits>
#include <vector>

#include "matrix.h"
#include "matrixsparse.h"
#include "r1cs.h"

template<typename E>
struct R1CSBuilder {
    struct Variable;
    struct LinearCombination;
    struct QuadraticCombination;
    struct Constraint;

    template<typename T>
    struct ConstraintExpression {
        constexpr Constraint operator () () const {
            return static_cast<const T&>(*this)();
        }

        constexpr void operator () (QuadraticCombination& qc) const {
            return static_cast<const T&>(*this)(qc);
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

        decltype(auto) begin() const {
            return terms.begin();
        }

        decltype(auto) end() const {
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

        constexpr void operator () (QuadraticCombination& qc) const {
            (*this)(qc.a);
            qc.b.emplace(Variable::constant(), E(1));
        }

        constexpr void operator () (LinearCombination& lc) const {
            lc += *this;
        }

        consteval static std::size_t degree() {
            return 1;
        }
    };

    struct QuadraticCombination {
        LinearCombination a;
        LinearCombination b;
    };

    struct Constraint {
        QuadraticCombination qc;
        LinearCombination lc;
    };

    struct Constant : ConstraintExpression<Constant> {
        E value;

        constexpr Constant() {}
        constexpr Constant(const E& value) : value(value) {}

        constexpr Constraint operator () () const {
            static_assert(false, "Constant is not a constraint");
        }

        constexpr void operator () (QuadraticCombination&) const {
            static_assert(false, "Constant is not a quadratic combination");
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

        constexpr void operator () (QuadraticCombination&) const {
            static_assert(false, "Variable is not a quadratic combination");
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

        constexpr void operator () (QuadraticCombination& qc) const {
            static_assert(L::degree() <= 1 && R::degree() <= 1, "Can't add non-linear expressions");
            (*this)(qc.a);
            qc.b.emplace(Variable::constant(), E(1));
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

        constexpr void operator () (QuadraticCombination& qc) const {
            if constexpr (std::is_same_v<L, Constant> || std::is_same_v<R, Constant>) {
                (*this)(qc.a);
                qc.b.emplace(Variable::constant(), E(1));
            } else if constexpr (std::is_same_v<L, Variable> && std::is_same_v<R, Variable>) {
                qc.a.emplace(l, E(1));
                qc.b.emplace(r, E(1));
            } else if constexpr (std::is_same_v<L, Variable> && R::degree() == 1) {
                qc.a.emplace(l, E(1));
                r(qc.b);
            } else if constexpr (std::is_same_v<R, Variable> && L::degree() == 1) {
                l(qc.a);
                qc.b.emplace(r, E(1));
            } else if constexpr (L::degree() == 1 && R::degree() == 1) {
                l(qc.a);
                r(qc.b);
            } else {
                static_assert(false, "Can't mul non-linear expressions");
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
            static_assert(degree() <= 2, "High-degree constraints are not supported");
            Constraint constraint;
            if constexpr (std::is_same_v<L, Constant>) {
                if constexpr (std::is_same_v<R, Constant>) {
                    static_assert(false, "Not implemented");
                } else if constexpr (std::is_same_v<R, Variable>) {
                    constraint.qc.a.emplace(Variable::constant(), l.value);
                    constraint.qc.a.emplace(r, E(-1));
                    constraint.qc.b.emplace(Variable::constant(), E(1));
                } else {
                    constraint.lc.emplace(Variable::constant(), l.value);
                    r(constraint.qc);
                }
            } else if constexpr (std::is_same_v<L, Variable>) {
                if constexpr (std::is_same_v<R, Constant>) {
                    constraint.qc.a.emplace(l, E(-1));
                    constraint.qc.a.emplace(Variable::constant(), r.value);
                    constraint.qc.b.emplace(Variable::constant(), E(1));
                } else if constexpr (std::is_same_v<R, Variable>) {
                    constraint.qc.a.emplace(l, E(1));
                    constraint.qc.a.emplace(r, E(-1));
                    constraint.qc.b.emplace(Variable::constant(), E(1));
                } else {
                    constraint.lc.emplace(l, E(1));
                    r(constraint.qc);
                }
            } else if constexpr (L::degree() == 1) {
                l(constraint.lc);
                r(constraint.qc);
            } else {
                static_assert(false, "Not implemented");
            }
            return constraint;
        }

        constexpr void operator () (QuadraticCombination&) const {
            static_assert(false, "Equality is not a quadratic combination");
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

    consteval R1CSBuilder() : inputs(0), auxiliaries(0), constraints() {}

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
        std::size_t variables = 1 + inputs + auxiliaries;
        Matrix<E> a(constraints.size(), variables, E(0));
        Matrix<E> b(constraints.size(), variables, E(0));
        Matrix<E> c(constraints.size(), variables, E(0));
        for (std::size_t i = 0; i < constraints.size(); ++i) {
            for (const auto& [variable, coefficient] : constraints[i].qc.a) {
                if (variable.type == Variable::Type::Constant)
                    a[i, 0] = coefficient;
                else if (variable.type == Variable::Type::Input)
                    a[i, variable.number] = coefficient;
                else if (variable.type == Variable::Type::Auxiliary)
                    a[i, inputs + variable.number] = coefficient;
            }
            for (const auto& [variable, coefficient] : constraints[i].qc.b) {
                if (variable.type == Variable::Type::Constant)
                    b[i, 0] = coefficient;
                else if (variable.type == Variable::Type::Input)
                    b[i, variable.number] = coefficient;
                else if (variable.type == Variable::Type::Auxiliary)
                    b[i, inputs + variable.number] = coefficient;
            }
            for (const auto& [variable, coefficient] : constraints[i].lc) {
                if (variable.type == Variable::Type::Constant)
                    c[i, 0] = coefficient;
                else if (variable.type == Variable::Type::Input)
                    c[i, variable.number] = coefficient;
                else if (variable.type == Variable::Type::Auxiliary)
                    c[i, inputs + variable.number] = coefficient;
            }
        }
        return { MatrixSparse(a), MatrixSparse(b), MatrixSparse(c) };
    }
};

#endif
