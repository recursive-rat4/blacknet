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

#ifndef BLACKNET_CRYPTO_KYBER_H
#define BLACKNET_CRYPTO_KYBER_H

#include "cyclotomicring.h"
#include "integerring.h"

/*
 * CRYSTALS-Kyber (version 3.02)
 * Roberto Avanzi, Joppe Bos, Léo Ducas, Eike Kiltz, Tancrède Lepoint,
 * Vadim Lyubashevsky, John M. Schanck, Peter Schwabe, Gregor Seiler,
 * Damien Stehlé
 * August 4, 2021
 * https://pq-crystals.org/kyber/data/kyber-specification-round3-20210804.pdf
 */

namespace kyber {
    const int16_t Q = 3329;
    const std::size_t N = 256;

    using Zq = IntegerRing<
        int16_t,
        int32_t,
        Q,
        1353,
        -3327,
        [] (int16_t x) -> int16_t {
            constexpr int16_t M2 = ((1 << 26) + Q / 2) / Q;
            int16_t t((int32_t(x) * int32_t(M2) + (1 << 25)) >> 26);
            return x - t * Q;
        }
    >;

    using Rq = CyclotomicRing<
        Zq,
        N
    >;
}

#endif
