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

#ifndef BLACKNET_CRYPTO_FASTRNG_H
#define BLACKNET_CRYPTO_FASTRNG_H

#include <limits>

#include "chacha.h"
#include "getentropy.h"

namespace blacknet::crypto {

// Implements std::uniform_random_bit_generator
class FastDRG : private ChaCha<8> {
public:
    using seed_type = std::array<std::byte, ChaCha::KEY_SIZE>;
    using result_type = ChaCha::W;
    constexpr static const std::size_t word_count = ChaCha::L;
    constexpr static const seed_type default_seed{};
private:
    constexpr static const std::array<std::byte, ChaCha::IV_SIZE> IV{};
    std::array<result_type, word_count> buffer;
    std::size_t position;
public:
    FastDRG() : ChaCha(default_seed, IV), position(0) {
        ChaCha::keystream(buffer);
    }

    void seed(const seed_type& seed) {
        ChaCha::reset(seed, IV);
        ChaCha::keystream(buffer);
        position = 0;
    }

    result_type operator () () {
        if (position != word_count) {
            return buffer[position++];
        } else {
            position = 1;
            ChaCha::keystream(buffer);
            return buffer[0];
        }
    }

    void discard(std::size_t z) {
        std::size_t pos_z = position + z;
        if (pos_z <= word_count) {
            position = pos_z;
            return;
        }
        static_assert(word_count == 16, "Not implemented");
        std::size_t q = pos_z >> 4;
        std::size_t r = pos_z & 15;
        ChaCha::counter(ChaCha::counter() + q - 1);
        position = r;
        ChaCha::keystream(buffer);
    }

    consteval static result_type min() {
        return std::numeric_limits<result_type>::min();
    }
    consteval static result_type max() {
        return std::numeric_limits<result_type>::max();
    }
};

class FastRNG : public FastDRG {
public:
    FastRNG() : FastDRG() {
        seed_type seed;
        compat::getentropy(seed);
        FastDRG::seed(seed);
    }
};

}

#endif
