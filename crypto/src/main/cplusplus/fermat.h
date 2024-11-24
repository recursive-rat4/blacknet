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

#ifndef BLACKNET_CRYPTO_FERMAT_H
#define BLACKNET_CRYPTO_FERMAT_H

#include "integerring.h"

// 2¹⁶ + 1
constexpr int32_t Fermat4Number(65537);

typedef IntegerRing<
    int32_t,
    int64_t,
    uint32_t,
    uint64_t,
    Fermat4Number,
    1,
    -65535,
    431,
    1024,
    [] (int32_t x) -> int32_t {
        return (x & 0xFFFF) - (x >> 16);
    },
    [] (int32_t x) -> int32_t {
        return x;
    }
> FermatRing;

#endif
