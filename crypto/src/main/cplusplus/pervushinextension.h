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

#ifndef BLACKNET_CRYPTO_PERVUSHINEXTENSION_H
#define BLACKNET_CRYPTO_PERVUSHINEXTENSION_H

#include "bitint.h"
#include "convolution.h"
#include "pervushin.h"
#include "polynomialring.h"

namespace blacknet::crypto {

struct PervushinRingDegree2Params {
    using Z = PervushinRing;

    constexpr static const std::size_t cyclotomic_index = 4;
    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t N = 2;
    constexpr static const std::array<Z, N + 1> M {
        PervushinRing(1),
        PervushinRing(0),
        PervushinRing(1),
    };
    constexpr static const BitInt<61> INVERSION_R1{"1fffffffffffffff"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        Convolution<Z>::negacyclic<N>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<PervushinRingDegree2Params> PervushinRingDegree2;

struct PervushinRingDegree3Params {
    using Z = PervushinRing;

    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t N = 3;
    constexpr static const std::array<Z, N + 1> M {
        PervushinRing(2),
        PervushinRing(0),
        PervushinRing(1),
        PervushinRing(1),
    };
    constexpr static const BitInt<122> INVERSION_R1{"03ffffffffffffffe000000000000000"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        Convolution<Z>::quotient<N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<PervushinRingDegree3Params> PervushinRingDegree3;

struct PervushinRingDegree4Params {
    using Z = PervushinRing;

    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t N = 4;
    constexpr static const std::array<Z, N + 1> M {
        PervushinRing(1),
        PervushinRing(0),
        PervushinRing(0),
        PervushinRing(1),
        PervushinRing(1),
    };
    constexpr static const BitInt<183> INVERSION_R1{"007ffffffffffffff8000000000000003fffffffffffffff"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        Convolution<Z>::quotient<N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<PervushinRingDegree4Params> PervushinRingDegree4;

}

#endif
