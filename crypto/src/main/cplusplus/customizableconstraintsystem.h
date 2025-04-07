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

#include <vector>

#include "matrixsparse.h"
#include "multilinearextension.h"
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
    constexpr CustomizableConstraintSystem(CustomizableConstraintSystem&& other) noexcept
        : rows(other.rows), columns(other.columns), m(std::move(other.m)), s(std::move(other.s)), c(std::move(other.c)) {}

    constexpr bool isSatisfied(const Vector<E>& z) const {
        Vector<E> sigma(rows, E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < c.size(); ++i) {
            Vector<E> circle(rows, c[i]);
            for (std::size_t j : s[i]) {
                circle *= m[j] * z;
            }
            sigma += circle;
        }
        return sigma == Vector<E>(rows, E(0));
    }

    constexpr bool operator == (const CustomizableConstraintSystem&) const = default;

    constexpr std::size_t constraints() const {
        return rows;
    }

    constexpr std::size_t variables() const {
        return columns;
    }

    friend std::ostream& operator << (std::ostream& out, const CustomizableConstraintSystem& val)
    {
        return out << '(' << val.rows << ", " << val.columns << ", " << val.m << ", " << val.s << ", " << val.c << ')';
    }

    template<typename Z = E>
    class Polynomial {
        std::size_t deg;
        std::size_t var;
        std::vector<MultilinearExtension<Z>> mz;
        std::vector<std::vector<std::size_t>> s;
        std::vector<Z> c;
    public:
        constexpr Polynomial(
            std::size_t deg,
            std::size_t var,
            const std::vector<MultilinearExtension<Z>>& mz,
            const std::vector<std::vector<std::size_t>>& s,
            const std::vector<Z>& c
        ) : deg(deg), var(var), mz(mz), s(s), c(c) {}
        constexpr Polynomial(
            std::size_t deg,
            std::size_t var,
            std::vector<MultilinearExtension<Z>>&& mz,
            std::vector<std::vector<std::size_t>>&& s,
            std::vector<Z>&& c
        ) : deg(deg), var(var), mz(std::move(mz)), s(std::move(s)), c(std::move(c)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z sigma(Z::LEFT_ADDITIVE_IDENTITY());
            for (std::size_t i = 0; i < c.size(); ++i) {
                Z circle(c[i]);
                for (std::size_t j : s[i]) {
                    circle *= mz[j](point);
                }
                sigma += circle;
            }
            return sigma;
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> sigma(hypercube.size(), Z::LEFT_ADDITIVE_IDENTITY());
            for (std::size_t i = 0; i < c.size(); ++i) {
                std::vector<Z> circle(hypercube.size(), c[i]);
                for (std::size_t j : s[i]) {
                    mz[j].template bind<e, util::Mul<Z>>(circle);
                }
                util::Add<Z>::call(sigma, circle);
            }
            Fuse::call(hypercube, std::move(sigma));
        }

        constexpr void bind(const Z& e) {
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

        template<typename S>
        constexpr Polynomial<S> homomorph() const {
            std::vector<MultilinearExtension<S>> hmz;
            hmz.reserve(mz.size());
            for (std::size_t i = 0; i < mz.size(); ++i)
                hmz.emplace_back(mz[i].template homomorph<S>());
            std::vector<std::vector<std::size_t>> hs(s);
            std::vector<S> hc;
            hc.reserve(c.size());
            for (std::size_t i = 0; i < c.size(); ++i)
                hc.emplace_back(c[i]);
            return Polynomial<S>(deg, var, std::move(hmz), std::move(hs), std::move(hc));
        }
    };

    constexpr Polynomial<E> polynomial(const Vector<E>& z) const {
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
