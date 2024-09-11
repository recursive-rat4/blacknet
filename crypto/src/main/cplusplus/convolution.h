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

#ifndef BLACKNET_CRYPTO_CONVOLUTION_H
#define BLACKNET_CRYPTO_CONVOLUTION_H

#include <array>

namespace convolution {
    template<typename Z, std::size_t N>
    constexpr void negacyclic(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        for (std::size_t k = 0; k < N; ++k) {
            for (std::size_t i = 0; i <= k; ++i) {
                r[k] += a[i] * b[k - i];
            }
            for (std::size_t i = k + 1; i < N; ++i) {
                r[k] -= a[i] * b[k + N - i];
            }
        }
    }

    template<typename Z, std::size_t N, std::size_t M>
    constexpr void lonk(std::array<Z, N+M-1>& r, const std::array<Z, N>& a, const std::array<Z, M>& b) {
        for (std::size_t i = 0; i < N; ++i) {
            for (std::size_t j = 0; j < M; ++j) {
                r[i + j] += a[i] * b[j];
            }
        }
    }

    template<typename Z, std::size_t N, std::array<Z, N+1> M>
    constexpr void quotient(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        static_assert(M[M.size()-1] == Z(1));
        std::array<Z, N + N - 1> t;
        t.fill(Z::LEFT_ADDITIVE_IDENTITY());
        lonk(t, a, b);
        if constexpr (N == 2) {
            r[0] = t[0] - t[2] * M[0];
            r[1] = t[1] - t[2] * M[1];
        } else if constexpr (N == 4) {
            r[2] = t[2] - t[6] * M[0];
            r[3] = t[3] - t[6] * M[1];
            t[4] -= t[6] * M[2];
            t[5] -= t[6] * M[3];

            r[1] = t[1] - t[5] * M[0];
            r[2] -= t[5] * M[1];
            r[3] -= t[5] * M[2];
            t[4] -= t[5] * M[3];

            r[0] = t[0] - t[4] * M[0];
            r[1] -= t[4] * M[1];
            r[2] -= t[4] * M[2];
            r[3] -= t[4] * M[3];
        } else {
            static_assert(false, "Not implemented");
        }
    }
}

#endif
