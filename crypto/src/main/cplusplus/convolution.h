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

namespace convolution {
    template<typename CR>
    constexpr CR negacyclic(const CR& a, const CR& b) {
        CR t(CR::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t k = 0; k < CR::DEGREE(); ++k) {
            for (std::size_t i = 0; i <= k; ++i) {
                t.coefficients[k] += a.coefficients[i] * b.coefficients[k - i];
            }
            for (std::size_t i = k + 1; i < CR::DEGREE(); ++i) {
                t.coefficients[k] -= a.coefficients[i] * b.coefficients[k + CR::DEGREE() - i];
            }
        }
        return t;
    }
}

#endif
