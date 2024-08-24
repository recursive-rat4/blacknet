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

#ifndef BLACKNET_CRYPTO_SOLINAS62_H
#define BLACKNET_CRYPTO_SOLINAS62_H

#include "integerring.h"

// 2⁶² - 2⁸ - 2⁵ + 1
constexpr int64_t Solinas62Prime(0x3ffffffffffffee1);

typedef IntegerRing<
    int64_t,
    __int128_t,
    uint64_t,
    __uint128_t,
    Solinas62Prime,
    1317904,
    -3454747365720865503,
    [] (int64_t x) -> int64_t {
        int32_t t((x + (1l << 61)) >> 62);
        return x - t * Solinas62Prime;
    },
    [] (int64_t x) -> int64_t {
        return x + ((x >> 63) & Solinas62Prime);
    }
> Solinas62Ring;

#endif
