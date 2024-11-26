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

#ifndef BLACKNET_CRYPTO_SOLINAS62FIELD_H
#define BLACKNET_CRYPTO_SOLINAS62FIELD_H

#include "polynomialring.h"
#include "solinas62.h"

typedef ExtensionRing<
    Solinas62Ring,
    2,
    std::array{
        Solinas62Ring("3f017d539af5221c"),
        Solinas62Ring(0),
        Solinas62Ring(1),
    }
> Solinas62RingDegree2;

typedef ExtensionRing<
    Solinas62Ring,
    3,
    std::array{
        Solinas62Ring(2),
        Solinas62Ring(0),
        Solinas62Ring(1),
        Solinas62Ring(1),
    }
> Solinas62RingDegree3;

typedef ExtensionRing<
    Solinas62Ring,
    4,
    std::array{
        Solinas62Ring("3f017d539af5221c"),
        Solinas62Ring(0),
        Solinas62Ring(0),
        Solinas62Ring(0),
        Solinas62Ring(1),
    }
> Solinas62RingDegree4;

#endif
