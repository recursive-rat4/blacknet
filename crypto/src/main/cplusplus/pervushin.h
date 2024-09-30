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

#ifndef BLACKNET_CRYPTO_PERVUSHIN_H
#define BLACKNET_CRYPTO_PERVUSHIN_H

#include "integerring.h"
#include "polynomialring.h"

// 2⁶¹ - 1
constexpr int64_t PervushinNumber(2305843009213693951);

typedef IntegerRing<
    int64_t,
    __int128_t,
    uint64_t,
    __uint128_t,
    PervushinNumber,
    64,
    -2305843009213693953,
    [] (int64_t x) -> int64_t {
        return (x & PervushinNumber) + (x >> 61);
    },
    [] (int64_t x) -> int64_t {
        return x + ((x >> 63) & PervushinNumber);
    }
> PervushinRing;

typedef ExtensionRing<
    PervushinRing,
    2,
    std::array{
        PervushinRing(1),
        PervushinRing(0),
        PervushinRing(1),
    }
> PervushinRingDegree2;

typedef ExtensionRing<
    PervushinRing,
    3,
    std::array{
        PervushinRing(2),
        PervushinRing(0),
        PervushinRing(1),
        PervushinRing(1),
    }
> PervushinRingDegree3;

typedef ExtensionRing<
    PervushinRing,
    4,
    std::array{
        PervushinRing(1),
        PervushinRing(0),
        PervushinRing(0),
        PervushinRing(1),
        PervushinRing(1),
    }
> PervushinRingDegree4;

#endif
