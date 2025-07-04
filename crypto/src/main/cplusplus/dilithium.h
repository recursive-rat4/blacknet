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

#ifndef BLACKNET_CRYPTO_DILITHIUM_H
#define BLACKNET_CRYPTO_DILITHIUM_H

#include "dilithiumring.h"
#include "numbertheoretictransform.h"
#include "polynomialring.h"

namespace blacknet::crypto {

/*
 * CRYSTALS-Dilithium (Version 3.1)
 * Shi Bai, Léo Ducas, Eike Kiltz, Tancrède Lepoint, Vadim Lyubashevsky,
 * Peter Schwabe, Gregor Seiler, Damien Stehlé
 * February 8, 2021
 * https://pq-crystals.org/dilithium/data/dilithium-specification-round3-20210208.pdf
 */

namespace dilithium {
    // Dilithium3
    const std::size_t K = 6;
    const std::size_t L = 5;

    using Zq = DilithiumRing;

    struct CyclotomicRingParams {
        using Z = DilithiumRing;

        constexpr static const std::size_t cyclotomic_index = 512;

        constexpr static const std::size_t N = 256;

        using Convolution = NTT<Z, N>::Convolution;
        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            Convolution::call(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>& a) {
            NTT<Z, N>::cooley_tukey(a);
        }
        constexpr static void fromForm(std::array<Z, N>& a) {
            NTT<Z, N>::gentleman_sande(a);
        }

        static_assert(N == Z::twiddles());
    };

    using Rq = PolynomialRing<CyclotomicRingParams>;
}

}

#endif
