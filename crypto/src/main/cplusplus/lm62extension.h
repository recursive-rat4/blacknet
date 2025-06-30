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

#ifndef BLACKNET_CRYPTO_LM62EXTENSION_H
#define BLACKNET_CRYPTO_LM62EXTENSION_H

#include "bitint.h"
#include "convolution.h"
#include "lm62.h"
#include "numbertheoretictransform.h"
#include "polynomialring.h"

namespace blacknet::crypto {

struct LM62RingDegree2Params {
    using Z = LM62Ring;

    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t N = 2;
    constexpr static const std::array<Z, N + 1> M {
        LM62Ring("2739EB7259CE7E4A"),
        LM62Ring(0),
        LM62Ring(1),
    };
    constexpr static const BitInt<62> INVERSION_R1{"2840000000000021"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::binomial<Z, N>(r.data(), a.data(), b.data(), -M.front());
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<LM62RingDegree2Params> LM62RingDegree2;

struct LM62RingDegree3Params {
    using Z = LM62Ring;

    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t N = 3;
    constexpr static const std::array<Z, N + 1> M {
        LM62Ring(1),
        LM62Ring(1),
        LM62Ring(0),
        LM62Ring(1),
    };
    constexpr static const BitInt<123> INVERSION_R1{"065410000000000A88C0000000000462"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::quotient<Z, N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<LM62RingDegree3Params> LM62RingDegree3;

struct LM62RingDegree4Params {
    using Z = LM62Ring;

    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t N = 4;
    constexpr static const std::array<Z, N + 1> M {
        LM62Ring("2739EB7259CE7E4A"),
        LM62Ring(0),
        LM62Ring(0),
        LM62Ring(0),
        LM62Ring(1),
    };
    constexpr static const BitInt<184> INVERSION_R1{"00FEB7840000000278D640000000020C31800000000090C3"};

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::quotient<Z, N, M>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<LM62RingDegree4Params> LM62RingDegree4;

struct LM62RingDegree64Params {
    using Z = LM62Ring;

    constexpr static const std::size_t cyclotomic_index = 128;

    constexpr static const std::size_t N = 64;

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        convolution::negacyclic<Z, N>(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};

typedef PolynomialRing<LM62RingDegree64Params> LM62RingDegree64;

struct LM62RingDegree64NTTParams {
    using Z = LM62Ring;

    constexpr static const std::size_t cyclotomic_index = 128;

    constexpr static const std::size_t N = 64;

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        NTT<Z, N>::convolute(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>& a) {
        NTT<Z, N>::cooley_tukey(a);
    }
    constexpr static void fromForm(std::array<Z, N>& a) {
        NTT<Z, N>::gentleman_sande(a);
    }
};

typedef PolynomialRing<LM62RingDegree64NTTParams> LM62RingDegree64NTT;

}

#endif
