/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_SPONGE_H
#define BLACKNET_CRYPTO_SPONGE_H

#include <algorithm>
#include <array>
#include <concepts>
#include <span>

namespace blacknet::crypto {

// Duplex construction, https://keccak.team/files/CSF-0.1.pdf

enum SpongeMode {
    // Original
    Xor,
    // Generalized
    Add,
    // https://eprint.iacr.org/2008/263
    Overwrite,
};

template<
    typename E,
    std::size_t R,
    std::size_t C,
    std::array<E, C> IV,
    typename F,
    SpongeMode mode
>
requires(R + C == F::width())
struct Sponge {
    using Z = E;

    enum Phase { Absorb, Squeeze };

    Phase phase;
    std::size_t position;
    std::array<E, R+C> state;

    constexpr void reset() {
        phase = Absorb;
        position = 0;
        std::ranges::fill_n(state.begin(), R, E(0));
        std::ranges::copy(IV, state.begin() + R);
    }

    constexpr Sponge() {
        reset();
    }

    constexpr void absorb(const E& e) {
        if (phase == Squeeze) {
            phase = Absorb;
            position = 0;
        } else if (position == R) {
            F::permute(state);
            position = 0;
        }
        if constexpr (mode == SpongeMode::Xor) {
            state[position++] ^= e;
        } else if constexpr (mode == SpongeMode::Add) {
            state[position++] += e;
        } else if constexpr (mode == SpongeMode::Overwrite) {
            state[position++] = e;
        } else {
            static_assert(false, "Not implemented");
        }
    }

    template<std::size_t N>
    constexpr void absorb(const std::array<E, N>& array) {
        for (const E& i : array)
            absorb(i);
    }

    constexpr const E& squeeze() {
        if (phase == Absorb) {
            phase = Squeeze;
            pad(position, state);
            F::permute(state);
            position = 0;
        } else if (position == R) {
            F::permute(state);
            position = 0;
        }
        return state[position++];
    }

    template<std::size_t N>
    constexpr void squeeze(std::array<E, N>& array) {
        for (E& i : array)
            i = squeeze();
    }
private:
    template<typename EE>
    constexpr static void pad(std::size_t position, std::array<EE, R+C>& state) {
        static_assert(Z::characteristic() > 2, "Not implemented");
        // Minimum and non-injective padding, Hirose 2016
        if (position != R) {
            state[position++] = E(1);
            std::ranges::fill_n(state.begin() + position, R - position, E(0));
            position = R;
            state[R+C-1] += E(2);
        } else {
            state[R+C-1] += E(1);
        }
    }
public:
template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using LinearCombination = Builder::LinearCombination;

    Builder& circuit;
    Phase phase;
    std::size_t position;
    std::array<LinearCombination, R+C> state;

    constexpr Circuit(Builder& circuit) : circuit(circuit), phase(Absorb), position(0) {
        std::ranges::fill_n(state.begin(), R, E(0));
        std::ranges::copy(IV, state.begin() + R);
    }

    constexpr void absorb(const LinearCombination& e) {
        if (phase == Squeeze) {
            phase = Absorb;
            position = 0;
        } else if (position == R) {
            F::template Circuit<Builder>::permute(circuit, state);
            position = 0;
        }
        if constexpr (mode == SpongeMode::Add) {
            state[position++] += e;
        } else if constexpr (mode == SpongeMode::Overwrite) {
            state[position++] = e;
        } else {
            static_assert(false, "Not implemented");
        }
    }

    constexpr void absorb(const std::span<const LinearCombination>& span) {
        for (const LinearCombination& i : span)
            absorb(i);
    }

    constexpr LinearCombination squeeze() {
        if (phase == Absorb) {
            phase = Squeeze;
            pad(position, state);
            F::template Circuit<Builder>::permute(circuit, state);
            position = 0;
        } else if (position == R) {
            F::template Circuit<Builder>::permute(circuit, state);
            position = 0;
        }
        return state[position++];
    }

    constexpr void squeeze(const std::span<LinearCombination>& span) {
        for (LinearCombination& i : span)
            i = squeeze();
    }
};

template<std::size_t Degree>
struct Assigner {
    Sponge sponge;
    std::vector<E>& assigment;

    constexpr Assigner(std::vector<E>& assigment) : sponge(), assigment(assigment) {}

    constexpr void absorb(const E& e) {
        if (sponge.phase == Squeeze) {
            sponge.phase = Absorb;
            sponge.position = 0;
        } else if (sponge.position == R) {
            F::template Assigner<Degree>::permute(sponge.state, assigment);
            sponge.position = 0;
        }
        if constexpr (mode == SpongeMode::Add) {
            sponge.state[sponge.position++] += e;
        } else if constexpr (mode == SpongeMode::Overwrite) {
            sponge.state[sponge.position++] = e;
        } else {
            static_assert(false, "Not implemented");
        }
    }

    template<std::size_t N>
    constexpr void absorb(const std::array<E, N>& array) {
        for (const E& i : array)
            absorb(i);
    }

    constexpr E squeeze() {
        if (sponge.phase == Absorb) {
            sponge.phase = Squeeze;
            pad(sponge.position, sponge.state);
            F::template Assigner<Degree>::permute(sponge.state, assigment);
            sponge.position = 0;
        } else if (sponge.position == R) {
            F::template Assigner<Degree>::permute(sponge.state, assigment);
            sponge.position = 0;
        }
        return sponge.state[sponge.position++];
    }

    template<std::size_t N>
    constexpr void squeeze(std::array<E, N>& array) {
        for (E& i : array)
            i = squeeze();
    }
};
};

}

#endif
