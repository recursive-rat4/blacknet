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

#ifndef BLACKNET_CRYPTO_CHACHA20_H
#define BLACKNET_CRYPTO_CHACHA20_H

#include <cstddef>
#include <algorithm>
#include <array>
#include <bit>
#include <span>

#include "byte.h"

namespace blacknet::crypto {

/*
 * ChaCha, a variant of Salsa20
 * Daniel J. Bernstein
 * https://cr.yp.to/chacha/chacha-20080128.pdf
 */

class ChaCha20 {
    using W = uint32_t;
    constexpr static const std::size_t KEY_SIZE = 32;
    constexpr static const std::size_t IV_SIZE = 12;
    constexpr static const std::size_t BLOCK_SIZE = 64;
    constexpr static const std::size_t L = 16;
    constexpr static const std::size_t ROUNDS = 20;
    constexpr static const std::array<W, 4> sigma {
        0x61707865, 0x3320646E, 0x79622D32, 0x6B206574
    };

    std::array<W, L> input;
public:
    constexpr ChaCha20() = delete;
    constexpr ChaCha20(const ChaCha20&) = delete;
    constexpr ChaCha20(ChaCha20&&) = delete;

    constexpr ChaCha20& operator = (const ChaCha20&) = delete;
    constexpr ChaCha20& operator = (ChaCha20&&) = delete;

    ChaCha20(
        const std::span<const std::byte, KEY_SIZE>& key,
        const std::span<const std::byte, IV_SIZE>& iv
    ) {
        std::ranges::copy(sigma, input.begin());
        for (std::size_t i = 0; i < KEY_SIZE / sizeof(W); ++i)
            input[i + 4] = read(key.data() + i * sizeof(W));
        input[12] = 0;
        for (std::size_t i = 0; i < IV_SIZE / sizeof(W); ++i)
            input[i + 13] = read(iv.data() + i * sizeof(W));
    }

    constexpr void seek(W counter) {
        input[12] = counter;
    }

    void encrypt(const std::span<std::byte>& ct, const std::span<const std::byte>& pt) {
        crypt(ct, pt);
    }

    void decrypt(const std::span<std::byte>& pt, const std::span<const std::byte>& ct) {
        crypt(pt, ct);
    }
private:
    constexpr static void quarter(W& a, W& b, W& c, W& d) {
        a += b; d ^= a; d = std::rotl(d, 16);
        c += d; b ^= c; b = std::rotl(b, 12);
        a += b; d ^= a; d = std::rotl(d,  8);
        c += d; b ^= c; b = std::rotl(b,  7);
    }

    constexpr static void block(std::array<W, L>& output, const std::array<W, L>& input) {
        std::array<W, L> state;
        std::ranges::copy(input, state.begin());
        for (std::size_t i = 0; i < ROUNDS; i += 2) {
            quarter(state[0], state[4], state[ 8], state[12]);
            quarter(state[1], state[5], state[ 9], state[13]);
            quarter(state[2], state[6], state[10], state[14]);
            quarter(state[3], state[7], state[11], state[15]);

            quarter(state[0], state[5], state[10], state[15]);
            quarter(state[1], state[6], state[11], state[12]);
            quarter(state[2], state[7], state[ 8], state[13]);
            quarter(state[3], state[4], state[ 9], state[14]);
        }
        for (std::size_t i = 0; i < L; ++i) {
            output[i] = state[i] + input[i];
        }
    }

    static W read(const std::byte* memory) {
        return byte::read<W, std::endian::little>(memory);
    }

    void crypt(const std::span<std::byte>& y, const std::span<const std::byte>& x) {
        std::size_t offset = 0;
        std::size_t remain = x.size();
        std::array<W, L> state;
        while (remain) {
            block(state, input);
            ++input[12];
            std::size_t process = std::min(remain, BLOCK_SIZE);
            const std::byte* bytes = reinterpret_cast<const std::byte*>(state.data());
            for (std::size_t i = 0; i < process; ++i)
                y[offset + i] = x[offset + i] ^ bytes[i];
            remain -= process;
            offset += process;
        }
    }
};

}

#endif
