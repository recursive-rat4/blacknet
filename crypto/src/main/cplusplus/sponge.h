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

// Sponge construction, https://keccak.team/files/CSF-0.1.pdf

class SpongeException : public std::exception {
    std::string message;
public:
    SpongeException(const std::string& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

template<
    typename E,
    std::size_t R,
    std::size_t C,
    std::array<E, C> IV,
    typename F
>
requires(R + C == F::width())
class Sponge {
public:
    enum Phase { ABSORB, SQUEEZE };

    Phase phase;
    std::size_t position;
    std::array<E, R+C> state;

    constexpr Sponge() : phase(ABSORB), position(0) {
        std::ranges::fill_n(state.begin(), R, E(0));
        std::ranges::copy(IV, state.begin() + R);
    }

    constexpr void absorb(const E& e) {
        if (phase == SQUEEZE) {
            throw SpongeException("Cannot absorb during squeeze");
        } else if (position == R) {
            F::permute(state);
            position = 0;
        }
        // Overwrite mode, https://eprint.iacr.org/2008/263
        state[position++] = e;
    }

    constexpr E squeeze() {
        if (phase == ABSORB) {
            phase = SQUEEZE;
            pad(position, state);
            F::permute(state);
            position = 0;
        } else if (position == R) {
            F::permute(state);
            position = 0;
        }
        return state[position++];
    }
private:
    template<typename EE>
    constexpr static void pad(std::size_t position, std::array<EE, R+C>& state) {
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
template<typename Circuit>
requires(std::same_as<E, typename Circuit::R>)
struct circuit {
    using Variable = Circuit::Variable;
    using LinearCombination = Circuit::LinearCombination;

    constexpr static void init(
        std::array<LinearCombination, R+C>& state
    ) {
        std::ranges::fill_n(state.begin(), R, E(0));
        std::ranges::copy_n(IV.begin(), IV.size(), state.begin() + R);
    }

    template<std::size_t N, std::size_t M>
    constexpr static void fixed(
        Circuit& circuit,
        std::size_t position,
        std::array<LinearCombination, R+C>& state,
        const std::array<LinearCombination, N>& absorb,
        std::array<LinearCombination, M>& squeeze
    ) {
        for (const auto& e : absorb) {
            if (position == R) {
                F::template circuit<Circuit>::permute(circuit, state);
                position = 0;
            }
            state[position++] = e;
        }

        pad(position, state);
        F::template circuit<Circuit>::permute(circuit, state);
        position = 0;

        for (auto& e : squeeze) {
            if (position == R) {
                F::template circuit<Circuit>::permute(circuit, state);
                position = 0;
            }
            e = state[position++];
        }
    }
};

template<std::size_t circuit>
struct trace {
    template<std::size_t N, std::size_t M>
    constexpr static void fixed(
        std::size_t position,
        std::array<E, R+C>& state,
        const std::array<E, N>& absorb,
        std::array<E, M>& squeeze,
        std::vector<E>& trace
    ) {
        for (const auto& e : absorb) {
            if (position == R) {
                F::template trace<circuit>::permute(state, trace);
                position = 0;
            }
            state[position++] = e;
        }

        pad(position, state);
        F::template trace<circuit>::permute(state, trace);
        position = 0;

        for (auto& e : squeeze) {
            if (position == R) {
                F::template trace<circuit>::permute(state, trace);
                position = 0;
            }
            e = state[position++];
        }
    }
};
};

#endif
