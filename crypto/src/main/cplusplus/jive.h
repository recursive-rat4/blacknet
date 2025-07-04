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

namespace blacknet::crypto {

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

    template<typename Builder>
    using HashCircuit = std::array<typename Builder::LinearCombination, M>;

template<typename Builder>
requires(std::same_as<F, typename Builder::R>)
struct Circuit {
    using LinearCombination = Builder::LinearCombination;
    using Hash = HashCircuit<Builder>;

    constexpr static Hash compress(
        Builder* circuit,
        const Hash& x0,
        const Hash& x1
    ) {
        auto scope = circuit->scope("Jive::compress");
        std::array<LinearCombination, M * B> state;
        std::ranges::copy(x0, state.begin());
        std::ranges::copy(x1, state.begin() + x0.size());
        P::template Circuit<Builder>::permute(circuit, state);
        Hash hash;
        for (std::size_t i = 0; i < hash.size(); ++i)
            hash[i] = x0[i] + x1[i] + state[i] + state[i + hash.size()];
        return hash;
    }
};

template<std::size_t Degree>
struct Assigner {
    constexpr static Hash compress(
        const Hash& x0,
        const Hash& x1,
        std::vector<F>* assigment
    ) {
        std::array<F, M * B> state;
        std::ranges::copy(x0, state.begin());
        std::ranges::copy(x1, state.begin() + x0.size());
        P::template Assigner<Degree>::permute(state, assigment);
        Hash hash;
        for (std::size_t i = 0; i < hash.size(); ++i)
            hash[i] = x0[i] + x1[i] + state[i] + state[i + hash.size()];
        return hash;
    }
};
};

}

#endif
