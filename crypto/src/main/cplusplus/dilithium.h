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

#ifndef BLACKNET_CRYPTO_DILITHIUM_H
#define BLACKNET_CRYPTO_DILITHIUM_H

#include "dilithiumring.h"
#include "integerring.h"
#include "polynomialring.h"

/*
 * CRYSTALS-Dilithium (Version 3.1)
 * Shi Bai, Léo Ducas, Eike Kiltz, Tancrède Lepoint, Vadim Lyubashevsky,
 * Peter Schwabe, Gregor Seiler, Damien Stehlé
 * February 8, 2021
 * https://pq-crystals.org/dilithium/data/dilithium-specification-round3-20210208.pdf
 */

namespace dilithium {
    const int32_t Q = 8380417;
    const std::size_t N = 256;

    // Dilithium3
    const std::size_t K = 6;
    const std::size_t L = 5;

    using Zq = DilithiumRing;

    static_assert(N == Zq::zetas());

    using Rq = CyclotomicRing<
        Zq,
        N
    >;
}

#endif
