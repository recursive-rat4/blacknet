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

#include "pervushin.h"
#include "polynomialring.h"

typedef CyclotomicRing<
    PervushinRing,
    2
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
