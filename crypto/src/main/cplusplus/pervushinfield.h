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

#ifndef BLACKNET_CRYPTO_PERVUSHINFIELD_H
#define BLACKNET_CRYPTO_PERVUSHINFIELD_H

#include "bitint.h"
#include "convolution.h"
#include "pervushin.h"
#include "polynomialring.h"

namespace blacknet::crypto {

struct PervushinRingDegree2Params {
    using Z = PervushinRing;

    constexpr static const std::size_t N = 2;
    constexpr static const std::array<Z, N + 1> M {
        PervushinRing(1),
        PervushinRing(0),
        PervushinRing(1),
    };
    constexpr static const BitInt<122> PSY_MINUS_1{"03ffffffffffffffbfffffffffffffff"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::negacyclic<Z, N>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<PervushinRingDegree2Params> PervushinRingDegree2;

struct PervushinRingDegree3Params {
    using Z = PervushinRing;

    constexpr static const std::size_t N = 3;
    constexpr static const std::array<Z, N + 1> M {
        PervushinRing(2),
        PervushinRing(0),
        PervushinRing(1),
        PervushinRing(1),
    };
    constexpr static const BitInt<183> PSY_MINUS_1{"007ffffffffffffff4000000000000005ffffffffffffffd"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::quotient<Z, N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<PervushinRingDegree3Params> PervushinRingDegree3;

struct PervushinRingDegree4Params {
    using Z = PervushinRing;

    constexpr static const std::size_t N = 4;
    constexpr static const std::array<Z, N + 1> M {
        PervushinRing(1),
        PervushinRing(0),
        PervushinRing(0),
        PervushinRing(1),
        PervushinRing(1),
    };
    constexpr static const BitInt<244> PSY_MINUS_1{"000ffffffffffffffe0000000000000017ffffffffffffff7fffffffffffffff"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::quotient<Z, N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<PervushinRingDegree4Params> PervushinRingDegree4;

}

#endif
