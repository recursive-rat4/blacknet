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

#ifndef BLACKNET_CRYPTO_JIVE_H
#define BLACKNET_CRYPTO_JIVE_H

#include <algorithm>
#include <array>
#include <concepts>

// Jive Compression Mode, https://eprint.iacr.org/2022/840

template<
    typename F,
    std::size_t M,
    typename P,
    std::size_t B
>
requires(M * B == P::width())
struct Jive {
    static_assert(B == 2, "Not implemented");

    using Hash = std::array<F, M>;

    constexpr static Hash compress(const Hash& x0, const Hash& x1) {
        std::array<F, M * B> state;
        std::ranges::copy(x0, state.begin());
        std::ranges::copy(x1, state.begin() + x0.size());
        P::permute(state);
        Hash hash;
        for (std::size_t i = 0; i < hash.size(); ++i)
            hash[i] = x0[i] + x1[i] + state[i] + state[i + hash.size()];
        return hash;
    }

template<typename Circuit>
requires(std::same_as<F, typename Circuit::R>)
struct circuit {
    using LinearCombination = Circuit::LinearCombination;

    constexpr static void compress(
        Circuit& circuit,
        const std::array<LinearCombination, M>& x0,
        const std::array<LinearCombination, M>& x1,
        std::array<LinearCombination, M>& hash
    ) {
        std::array<LinearCombination, M * B> state;
        std::ranges::copy(x0, state.begin());
        std::ranges::copy(x1, state.begin() + x0.size());
        P::template circuit<Circuit>::permute(circuit, state);
        for (std::size_t i = 0; i < hash.size(); ++i)
            hash[i] = x0[i] + x1[i] + state[i] + state[i + hash.size()];
    }
};

template<std::size_t circuit>
struct trace {
    constexpr static void compress(
        const Hash& x0,
        const Hash& x1,
        Hash& hash,
        std::vector<F>& trace
    ) {
        std::array<F, M * B> state;
        std::ranges::copy(x0, state.begin());
        std::ranges::copy(x1, state.begin() + x0.size());
        P::template trace<circuit>::permute(state, trace);
        for (std::size_t i = 0; i < hash.size(); ++i)
            hash[i] = x0[i] + x1[i] + state[i] + state[i + hash.size()];
    }
};
};

#endif
