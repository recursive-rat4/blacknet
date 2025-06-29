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

#ifndef BLACKNET_CRYPTO_CUSTOMIZABLECONSTRAINTSYSTEM_H
#define BLACKNET_CRYPTO_CUSTOMIZABLECONSTRAINTSYSTEM_H

#include <stdexcept>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

#include "matrixsparse.h"
#include "multilinearextension.h"
#include "point.h"
#include "vector.h"
#include "util.h"

namespace blacknet::crypto {

/*
 * Customizable constraint systems for succinct arguments
 * Srinath Setty, Justin Thaler, Riad Wahby
 * May 3, 2023
 * https://eprint.iacr.org/2023/552
 */

template<typename E>
class CustomizableConstraintSystem {
    std::size_t rows;
    std::size_t columns;
    std::vector<MatrixSparse<E>> m;
    std::vector<std::vector<std::size_t>> s;
    std::vector<E> c;
public:
    using ElementType = E;

    constexpr CustomizableConstraintSystem(
        std::size_t rows,
        std::size_t columns,
        std::vector<MatrixSparse<E>>&& m,
        std::vector<std::vector<std::size_t>>&& s,
        std::vector<E>&& c
    ) : rows(rows), columns(columns), m(std::move(m)), s(std::move(s)), c(std::move(c)) {}
    constexpr CustomizableConstraintSystem(CustomizableConstraintSystem&&) noexcept = default;
    constexpr ~CustomizableConstraintSystem() noexcept = default;

    constexpr CustomizableConstraintSystem& operator = (CustomizableConstraintSystem&&) noexcept = default;

    constexpr bool isSatisfied(const Vector<E>& z) const {
        if (variables() != z.size()) {
            throw std::runtime_error(fmt::format("Assigned {} variables instead of {} required", z.size(), variables()));
        }

        Vector<E> sigma(rows, E::additive_identity());
        for (std::size_t i = 0; i < c.size(); ++i) {
            Vector<E> circle(rows, c[i]);
            for (std::size_t j : s[i]) {
                circle *= m[j] * z;
            }
            sigma += circle;
        }
        return sigma == Vector<E>(rows, E::additive_identity());
    }

    constexpr bool operator == (const CustomizableConstraintSystem&) const = default;

    constexpr std::size_t constraints() const {
        return rows;
    }

    constexpr std::size_t variables() const {
        return columns;
    }

    constexpr Vector<E> assigment(E&& constant = E::multiplicative_identity()) const {
        Vector<E> z;
        z.elements.reserve(variables());
        z.elements.emplace_back(constant);
        return z;
    }

    friend std::ostream& operator << (std::ostream& out, const CustomizableConstraintSystem& val)
    {
        fmt::print(out, "({}, {}, {}, {}, {})", val.rows, val.columns, val.m, val.s, val.c);
        return out;
    }

    class Polynomial {
        std::size_t deg;
        std::size_t var;
        std::vector<MultilinearExtension<E>> mz;
        std::vector<std::vector<std::size_t>> s;
        std::vector<E> c;
    public:
        constexpr Polynomial(
            std::size_t deg,
            std::size_t var,
            const std::vector<MultilinearExtension<E>>& mz,
            const std::vector<std::vector<std::size_t>>& s,
            const std::vector<E>& c
        ) : deg(deg), var(var), mz(mz), s(s), c(c) {}
        constexpr Polynomial(
            std::size_t deg,
            std::size_t var,
            std::vector<MultilinearExtension<E>>&& mz,
            std::vector<std::vector<std::size_t>>&& s,
            std::vector<E>&& c
        ) : deg(deg), var(var), mz(std::move(mz)), s(std::move(s)), c(std::move(c)) {}

        constexpr E operator () (const Point<E>& point) const {
            E sigma(E::additive_identity());
            for (std::size_t i = 0; i < c.size(); ++i) {
                E circle(c[i]);
                for (std::size_t j : s[i]) {
                    circle *= mz[j](point);
                }
                sigma += circle;
            }
            return sigma;
        }

        template<E e, typename Fuse>
        constexpr void bind(std::vector<E>& hypercube) const {
            std::vector<E> sigma(hypercube.size(), E::additive_identity());
            for (std::size_t i = 0; i < c.size(); ++i) {
                std::vector<E> circle(hypercube.size(), c[i]);
                for (std::size_t j : s[i]) {
                    mz[j].template bind<e, util::Mul<E>>(circle);
                }
                util::Add<E>::call(sigma, circle);
            }
            Fuse::call(hypercube, std::move(sigma));
        }

        constexpr void bind(const E& e) {
            --var;
            for (std::size_t i = 0; i < mz.size(); ++i)
                mz[i].bind(e);
        }

        constexpr std::size_t degree() const {
            return deg;
        }

        constexpr std::size_t variables() const {
            return var;
        }
    };

    constexpr Polynomial polynomial(const Vector<E>& z) const {
        std::vector<MultilinearExtension<E>> mz;
        mz.reserve(m.size());
        for (std::size_t i = 0; i < m.size(); ++i)
            mz.emplace_back(m[i] * z);
        std::vector<std::vector<std::size_t>> ps(s);
        std::vector<E> pc(c);
        std::size_t deg = std::ranges::max(s, {}, &std::vector<std::size_t>::size).size();
        std::size_t var = mz[0].variables();
        return { deg, var, std::move(mz), std::move(ps), std::move(pc) };
    }
};

}

#endif
