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

#ifndef BLACKNET_CRYPTO_TERNARYUNIFORMDISTRIBUTION_H
#define BLACKNET_CRYPTO_TERNARYUNIFORMDISTRIBUTION_H

#include <cstddef>
#include <cstdint>
#include <random>

namespace blacknet::crypto {

template<
    typename Z,
    std::uniform_random_bit_generator RNG
>
requires(Z::characteristic() >= 3)
class TernaryUniformDistribution {
    static_assert(Z::is_integer_ring, "Not implemented");

    using NumericType = RNG::result_type;

    consteval static std::size_t useful_bits() {
        return sizeof(NumericType) * 8;
    }

    NumericType cache;
    std::size_t have_bits;
public:
    using result_type = Z;

    constexpr TernaryUniformDistribution() noexcept {
        reset();
    }

    constexpr void reset() noexcept {
        have_bits = 0;
    }

    constexpr result_type operator () (RNG& rng) {
        for (;;) {
            if (have_bits == 0) {
                cache = rng();
                have_bits = useful_bits();
            } else {
                [[assume(have_bits >= 2)]];
            }
            std::int8_t sample = cache & 3;
            cache >>= 2;
            have_bits -= 2;
            if (sample != 3) {
                return sample - 1;
            }
        }
    }
};

}

#endif
