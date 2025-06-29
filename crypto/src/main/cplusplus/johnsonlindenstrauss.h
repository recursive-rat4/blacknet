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

#include "binaryuniformdistribution.h"
#include "matrixdense.h"
#include "vector.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2021/1397.pdf
// A Modular Johnson–Lindenstrauss Variant

template<typename Z>
requires(Z::is_integer_ring)
struct JohnsonLindenstrauss {
    template<std::uniform_random_bit_generator RNG>
    struct DistributionRNG {
        using result_type = Z;

        BinaryUniformDistributionRNG<result_type, RNG> bud;

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr result_type operator () (RNG& rng) {
            return bud(rng) + bud(rng) - Z(1);
        }
    };

    constexpr static Vector<Z> project(const MatrixDense<Z>& map, const Vector<Z> point) {
        return map * point;
    }

    template<std::uniform_random_bit_generator RNG>
    constexpr static MatrixDense<Z> random(RNG& rng, std::size_t n, std::size_t k) {
        DistributionRNG<RNG> dst;
        return MatrixDense<Z>::random(rng, dst, n, k);
    }
};

}

#endif
