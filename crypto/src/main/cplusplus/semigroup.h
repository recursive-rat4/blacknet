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

#ifndef BLACKNET_CRYPTO_SEMIGROUP_H
#define BLACKNET_CRYPTO_SEMIGROUP_H

#include <algorithm>

template<typename SG>
constexpr SG multiply(const SG& e, const typename SG::Scalar& s) {
    // Double-and-add method
    SG r(SG::LEFT_ADDITIVE_IDENTITY());
    SG t(e);
    std::ranges::for_each(s.bitsBegin(), s.bitsEnd(), [&](bool bit) {
        if (bit)
            r += t;
        t = t.douple();
    });
    return r;
}

template<typename SG>
constexpr SG power(const SG& e, const typename SG::Scalar& s) {
    // Square-and-multiply method
    SG r(SG::LEFT_MULTIPLICATIVE_IDENTITY());
    SG t(e);
    std::ranges::for_each(s.bitsBegin(), s.bitsEnd(), [&](bool bit) {
        if (bit)
            r *= t;
        t = t.square();
    });
    return r;
}

#endif
