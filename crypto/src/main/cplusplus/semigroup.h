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

#ifndef BLACKNET_CRYPTO_SEMIGROUP_H
#define BLACKNET_CRYPTO_SEMIGROUP_H

#include <algorithm>

namespace blacknet::crypto {

namespace semigroup {

template<typename Monoid>
consteval Monoid left_additive_identity() {
    return Monoid::additive_identity();
}

template<typename Monoid>
consteval Monoid right_additive_identity() {
    return Monoid::additive_identity();
}

template<typename Monoid>
consteval Monoid left_multiplicative_identity() {
    return Monoid::multiplicative_identity();
}

template<typename Monoid>
consteval Monoid right_multiplicative_identity() {
    return Monoid::multiplicative_identity();
}

template<typename SG, typename Scalar>
constexpr SG multiply(const SG& e, const Scalar& s) {
    // Double-and-add method
    SG r(left_additive_identity<SG>());
    SG t(e);
    std::ranges::for_each(s.bitsBegin(), s.bitsEnd(), [&](bool bit) {
        if (bit)
            r += t;
        t = t.douple();
    });
    return r;
}

template<typename SG, typename Scalar>
constexpr SG power(const SG& e, const Scalar& s) {
    // Square-and-multiply method
    SG r(left_multiplicative_identity<SG>());
    SG t(e);
    std::ranges::for_each(s.bitsBegin(), s.bitsEnd(), [&](bool bit) {
        if (bit)
            r *= t;
        t = t.square();
    });
    return r;
}

}

}

#endif
