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

#ifndef BLACKNET_CRYPTO_SOLINAS62FIELD_H
#define BLACKNET_CRYPTO_SOLINAS62FIELD_H

#include "bitint.h"
#include "convolution.h"
#include "polynomialring.h"
#include "solinas62.h"

namespace blacknet::crypto {

struct Solinas62RingDegree2Params {
    using Z = Solinas62Ring;

    constexpr static const std::size_t N = 2;
    constexpr static const std::array<Z, N + 1> M {
        Solinas62Ring("3f017d539af5221c"),
        Solinas62Ring(0),
        Solinas62Ring(1),
    };
    constexpr static const BitInt<62> INVERSION_R1{"3ffffffffffffee1"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::binomial<Z, N>(r.data(), a.data(), b.data(), -M.front());
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<Solinas62RingDegree2Params> Solinas62RingDegree2;

struct Solinas62RingDegree3Params {
    using Z = Solinas62Ring;

    constexpr static const std::size_t N = 3;
    constexpr static const std::array<Z, N + 1> M {
        Solinas62Ring(2),
        Solinas62Ring(0),
        Solinas62Ring(1),
        Solinas62Ring(1),
    };
    constexpr static const BitInt<124> INVERSION_R1{"0fffffffffffff70c0000000000140a2"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::quotient<Z, N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<Solinas62RingDegree3Params> Solinas62RingDegree3;

struct Solinas62RingDegree4Params {
    using Z = Solinas62Ring;

    constexpr static const std::size_t N = 4;
    constexpr static const std::array<Z, N + 1> M {
        Solinas62Ring("3f017d539af5221c"),
        Solinas62Ring(0),
        Solinas62Ring(0),
        Solinas62Ring(0),
        Solinas62Ring(1),
    };
    constexpr static const BitInt<188> INVERSION_R1{"03ffffffffffffca400000000000f0c17ffffffffe988943"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::quotient<Z, N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<Solinas62RingDegree4Params> Solinas62RingDegree4;

}

#endif
