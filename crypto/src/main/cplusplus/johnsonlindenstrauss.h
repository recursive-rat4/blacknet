/*
 * Copyright (c) 2025 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_JOHNSONLINDENSTRAUSS_H
#define BLACKNET_CRYPTO_JOHNSONLINDENSTRAUSS_H

#include <random>
#include <type_traits>

#include "binaryuniformdistribution.h"
#include "matrix.h"
#include "vector.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2021/1397.pdf
// A Modular Johnson–Lindenstrauss Variant

template<typename Z>
requires(Z::is_integer_ring)
struct JohnsonLindenstrauss {
    static_assert(std::is_signed_v<typename Z::NumericType>);

    template<std::uniform_random_bit_generator RNG>
    struct Distribution {
        using result_type = Z::NumericType;

        BinaryUniformDistribution<result_type, RNG> bud;

        constexpr result_type operator () (RNG& rng) {
            if (bud(rng))
                return 0;
            else if (bud(rng))
                return 1;
            else
                return -1;
        }

        constexpr result_type min() const {
            return -1;
        }
        constexpr result_type max() const {
            return 1;
        }
    };

    constexpr static Vector<Z> project(const Matrix<Z>& map, const Vector<Z> point) {
        return map * point;
    }

    template<std::uniform_random_bit_generator RNG>
    constexpr static Matrix<Z> random(RNG& rng, std::size_t n, std::size_t k) {
        Distribution<RNG> dst;
        return Matrix<Z>::random(rng, dst, n, k);
    }
};

}

#endif
