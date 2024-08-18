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

template<typename F>
class Params {
public:
    std::size_t t;
    std::size_t alpha;
    std::size_t rfb;
    std::size_t rpe;
    std::size_t r;
    std::vector<F> rc;
    std::vector<F> m;
};

namespace {

template<typename F>
constexpr void m4(const Params<F>& params, std::vector<F>& x) {
    for (std::size_t i = 0; i < params.t >> 2; ++i) {
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

template<typename F>
constexpr void external(const Params<F>& params, std::vector<F>& x) {
    switch (params.t) {
        case 2: {
            F s(x[0] + x[1]);
            x[0] += s;
            x[1] += s;
            break;
        } case 3: {
            F s(x[0] + x[1] + x[2]);
            x[0] += s;
            x[1] += s;
            x[2] += s;
            break;
        } case 4: {
            m4(params, x);
            break;
        } case 8: case 12: case 16: case 20: case 24: {
            m4(params, x);
            std::array<F, 4> s;
            for (std::size_t i = 0; i < 4; ++i) {
                s[i] = x[i];
                for (std::size_t j = 1; j < params.t >> 2; ++j)
                    s[i] += x[(j << 2) + i];
            }
            for (std::size_t i = 0; i < x.size(); ++i)
                x[i] += s[i & 3];
            break;
        } default: {
            throw [](void*){};
        }
    }
}

template<typename F>
constexpr void internal(const Params<F>& params, std::vector<F>& x) {
    switch (params.t) {
        case 2: {
            F s(x[0] + x[1]);
            x[0] += s;
            x[1] = x[1].douple() + s;
            break;
        } case 3: {
            F s(x[0] + x[1] + x[2]);
            x[0] += s;
            x[1] += s;
            x[2] = x[2].douple() + s;
            break;
        } case 4: case 8: case 12: case 16: case 20: case 24: {
            F s(x[0]);
            for (std::size_t i = 1; i < params.t; ++i)
                s += x[i];
            for (std::size_t i = 0; i < x.size(); ++i)
                x[i] = x[i] * params.m[i] + s;
            break;
        } default: {
            throw [](void*){};
        }
    }
}

template<typename F>
constexpr void rc(const Params<F>& params, std::size_t round, std::vector<F>& x) {
    for (std::size_t i = 0; i < params.t; ++i)
        x[i] += params.rc[round * params.t + i];
}

template<typename F>
constexpr F sbox(const Params<F>& params, const F& x) {
    switch (params.alpha) {
        case 3:
            return x * x.square();
        case 5:
            return x * x.square().square();
        default:
            throw [](void*){};
    }
}

template<typename F>
constexpr void sbox(const Params<F>& params, std::vector<F>& x) {
    for (std::size_t i = 0; i < params.t; ++i)
        x[i] = sbox(params, x[i]);
}

}

template<typename F>
constexpr std::vector<F> permute(const Params<F>& params, const std::vector<F>& x) {
    std::vector<F> state(x);

    external(params, state);

    for (std::size_t round = 0; round < params.rfb; ++round) {
        rc(params, round, state);
        sbox(params, state);
        external(params, state);
    }

    for (std::size_t round = params.rfb; round < params.rpe; ++round) {
        state[0] += params.rc[round * params.t];
        state[0] = sbox(params, state[0]);
        internal(params, state);
    }

    for (std::size_t round = params.rpe; round < params.r; ++round) {
        rc(params, round, state);
        sbox(params, state);
        external(params, state);
    }

    return state;
}

}

#endif
