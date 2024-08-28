/*
 * Copyright (c) 2024 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_POSEIDON2_H
#define BLACKNET_CRYPTO_POSEIDON2_H

#include <array>
#include <vector>

/*
 * Poseidon2: A Faster Version of the Poseidon Hash Function
 * Lorenzo Grassi, Dmitry Khovratovich, Markus Schofnegger
 * February 8, 2024
 * https://eprint.iacr.org/2023/323
 */

namespace poseidon2 {

template<typename F, std::size_t A, std::size_t T, std::size_t R>
class Params {
public:
    std::size_t rfb;
    std::size_t rpe;
    std::array<F, T*R> rc;
    std::array<F, T> m;

    consteval std::size_t a() const { return A; }
    consteval std::size_t t() const { return T; }
    consteval std::size_t r() const { return R; }
};

namespace {

template<typename F, std::size_t A, std::size_t T, std::size_t R>
constexpr void m4(const Params<F, A, T, R>& params, std::array<F, T>& x) {
    (void)params;
    for (std::size_t i = 0; i < T >> 2; ++i) {
        std::size_t j = i << 2;
        std::array<F, 8> t;
        t[0] = x[j] + x[j + 1];
        t[1] = x[j + 2] + x[j + 3];
        t[2] = x[j + 1].douple() + t[1];
        t[3] = x[j + 3].douple() + t[0];
        t[4] = t[1].douple().douple() + t[3];
        t[5] = t[0].douple().douple() + t[2];
        t[6] = t[3] + t[5];
        t[7] = t[2] + t[4];
        x[j] = t[6];
        x[j + 1] = t[5];
        x[j + 2] = t[7];
        x[j + 3] = t[4];
    }
}

template<typename F, std::size_t A, std::size_t T, std::size_t R>
constexpr void external(const Params<F, A, T, R>& params, std::array<F, T>& x) {
    if constexpr (T == 2) {
        F s(x[0] + x[1]);
        x[0] += s;
        x[1] += s;
    } else if constexpr (T == 3) {
        F s(x[0] + x[1] + x[2]);
        x[0] += s;
        x[1] += s;
        x[2] += s;
    } else if constexpr (T == 4) {
        m4(params, x);
    } else if constexpr (T == 8 || T == 12 || T == 16 || T == 20 || T == 24) {
        m4(params, x);
        std::array<F, 4> s;
        for (std::size_t i = 0; i < 4; ++i) {
            s[i] = x[i];
            for (std::size_t j = 1; j < T >> 2; ++j)
                s[i] += x[(j << 2) + i];
        }
        for (std::size_t i = 0; i < T; ++i)
            x[i] += s[i & 3];
    } else {
        static_assert(false);
    }
}

template<typename F, std::size_t A, std::size_t T, std::size_t R>
constexpr void internal(const Params<F, A, T, R>& params, std::array<F, T>& x) {
    if constexpr (T == 2) {
        F s(x[0] + x[1]);
        x[0] += s;
        x[1] = x[1].douple() + s;
    } else if constexpr (T == 3) {
        F s(x[0] + x[1] + x[2]);
        x[0] += s;
        x[1] += s;
        x[2] = x[2].douple() + s;
    } else if constexpr (T == 4 || T == 8 || T == 12 || T == 16 || T == 20 || T == 24) {
        F s(x[0]);
        for (std::size_t i = 1; i < T; ++i)
            s += x[i];
        for (std::size_t i = 0; i < T; ++i)
            x[i] = x[i] * params.m[i] + s;
    } else {
        static_assert(false);
    }
}

template<typename F, std::size_t A, std::size_t T, std::size_t R>
constexpr void rc(const Params<F, A, T, R>& params, std::size_t round, std::array<F, T>& x) {
    for (std::size_t i = 0; i < T; ++i)
        x[i] += params.rc[round * T + i];
}

template<typename F, std::size_t A, std::size_t T, std::size_t R>
constexpr F sbox(const Params<F, A, T, R>& params, const F& x) {
    (void)params;
    if constexpr (A == 3) {
        return x * x.square();
    } else if constexpr (A == 5) {
        return x * x.square().square();
    } else {
        static_assert(false);
    }
}

template<typename F, std::size_t A, std::size_t T, std::size_t R>
constexpr void sbox(const Params<F, A, T, R>& params, std::array<F, T>& x) {
    for (std::size_t i = 0; i < T; ++i)
        x[i] = sbox(params, x[i]);
}

}

template<Params params>
constexpr void permute(auto& x) {
    external(params, x);

    for (std::size_t round = 0; round < params.rfb; ++round) {
        rc(params, round, x);
        sbox(params, x);
        external(params, x);
    }

    for (std::size_t round = params.rfb; round < params.rpe; ++round) {
        x[0] += params.rc[round * params.t()];
        x[0] = sbox(params, x[0]);
        internal(params, x);
    }

    for (std::size_t round = params.rpe; round < params.r(); ++round) {
        rc(params, round, x);
        sbox(params, x);
        external(params, x);
    }
}

}

#endif
