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

#ifndef BLACKNET_CRYPTO_BINARYUNIFORMDISTRIBUTION_H
#define BLACKNET_CRYPTO_BINARYUNIFORMDISTRIBUTION_H

#include <cstddef>
#include <limits>
#include <random>

namespace blacknet::crypto {

template<
    typename T,
    std::uniform_random_bit_generator RNG
>
class BinaryUniformDistribution {
    using number_type = RNG::result_type;

    consteval static std::size_t useful_bits() {
        return sizeof(number_type) * 8;
    }

    number_type cache;
    std::size_t have_bits;
public:
    using result_type = T;

    constexpr BinaryUniformDistribution() noexcept {
        reset();
    }

    constexpr void reset() noexcept {
        have_bits = 0;
    }

    constexpr result_type operator () (RNG& rng) {
        if (have_bits == 0) {
            cache = rng();
            have_bits = useful_bits();
        }
        result_type result = cache & 1;
        cache >>= 1;
        --have_bits;
        return result;
    }

    constexpr result_type min() const {
        return 0;
    }
    constexpr result_type max() const {
        return 1;
    }
};

}

#endif
