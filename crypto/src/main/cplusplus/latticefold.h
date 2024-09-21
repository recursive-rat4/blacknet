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

#ifndef BLACKNET_CRYPTO_LATTICEFOLD_H
#define BLACKNET_CRYPTO_LATTICEFOLD_H

#include "eqextension.h"
#include "matrix.h"
#include "multilinearextension.h"
#include "polynomialring.h"
#include "vector.h"

/*
 * LatticeFold: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems
 * Dan Boneh, Binyi Chen
 * July 30, 2024
 * https://eprint.iacr.org/2024/257
 */

namespace latticefold {
    constexpr ssize_t b = 2;
    const std::size_t k = 16;
    const std::size_t B = 65536;
    const std::size_t D = 64;
    const std::size_t K = 16;

    template<typename Z>
    using Rq = CyclotomicRing<
        Z,
        D
    >;

    template<typename R>
    constexpr Matrix<R> gadget(std::size_t m, std::size_t n) {
        Vector<R> bpm(n);
        bpm[0] = R(1);
        for (std::size_t i = 1; i < n; ++i)
            bpm[i] = bpm[i - 1] * B;
        return Vector<R>::identity(m).tensor(bpm);
    }

    template<typename Z>
    class G1 {
        EqExtension<Z> eq;
        MultilinearExtension<Z> mle;
    public:
        constexpr G1(const std::vector<Z>& r, const Rq<Z>& f) : eq(r), mle(f) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            return eq(point) * mle(point);
        }

        consteval std::size_t degree() const {
            return eq.degree() + mle.degree();
        }

        constexpr std::size_t variables() const {
            return eq.variables();
        }
    };

    template<typename Z>
    class G2 {
        EqExtension<Z> eq;
        std::array<MultilinearExtension<Z>, b + b - 1> pis;
    public:
        constexpr G2(const std::vector<Z>& beta, const Rq<Z>& f) : eq(beta) {
            std::size_t i = 0;
            for (ssize_t j = - (b - 1); j <= b - 1; ++j) {
                pis[i++] = MultilinearExtension<Z>(f) - Z(j);
            }
        }

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z pi(eq(point));
            for (const auto& i : pis)
                pi *= i(point);
            return pi;
        }

        consteval std::size_t degree() const {
            return eq.degree() + pis.size();
        }

        constexpr std::size_t variables() const {
            return eq.variables();
        }
    };
}

#endif
