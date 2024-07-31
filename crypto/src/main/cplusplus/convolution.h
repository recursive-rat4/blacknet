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
}

#endif
