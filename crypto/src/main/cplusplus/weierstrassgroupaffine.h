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

#ifndef BLACKNET_CRYPTO_WEIERSTRASSGROUPAFFINE_H
#define BLACKNET_CRYPTO_WEIERSTRASSGROUPAFFINE_H

#include <iostream>
#include <boost/random/uniform_int_distribution.hpp>

#include "semigroup.h"

template<typename BF, typename SF, BF A, BF B>
class WeierstrassGroupAffine {
    BF x;
    BF y;
public:
    typedef BF Base;
    typedef SF Scalar;
    consteval static WeierstrassGroupAffine LEFT_ADDITIVE_IDENTITY() { return WeierstrassGroupAffine(); }

    consteval WeierstrassGroupAffine() : x(0), y(0) {}
    constexpr WeierstrassGroupAffine(const BF& x, const BF& y) : x{x}, y(y) {}

    constexpr bool operator == (const WeierstrassGroupAffine& other) const {
        return x == other.x && y == other.y;
    }

    constexpr WeierstrassGroupAffine operator - () const {
        if (*this != WeierstrassGroupAffine())
            return WeierstrassGroupAffine(x, -y);
        else
            return WeierstrassGroupAffine();
    }

    constexpr WeierstrassGroupAffine operator + (const WeierstrassGroupAffine& other) const {
        if (*this == WeierstrassGroupAffine())
            return other;
        if (other == WeierstrassGroupAffine())
            return *this;

        if (x != other.x) {
            BF k((other.y - y) / (other.x - x));
            BF xr(k.square() - x - other.x);
            BF yr(k * (x - xr) - y);
            return WeierstrassGroupAffine(xr, yr);
        } else if (y == other.y) {
            return douple();
        } else {
            return WeierstrassGroupAffine();
        }
    }

    constexpr WeierstrassGroupAffine douple() const {
        if (*this == WeierstrassGroupAffine())
            return WeierstrassGroupAffine();

        BF xx(x.square());
        BF k(xx + xx + xx);
        if constexpr (A != BF(0))
            k += A;
        k /= y + y;
        BF xr(k.square() - x - x);
        BF yr(k * (x - xr) - y);
        return WeierstrassGroupAffine(xr, yr);
    }

    constexpr WeierstrassGroupAffine& operator += (const WeierstrassGroupAffine& other) {
        return *this = *this + other;
    }

    constexpr WeierstrassGroupAffine operator * (const SF& other) const {
        return multiply(*this, other);
    }

    friend std::ostream& operator << (std::ostream& out, const WeierstrassGroupAffine& val)
    {
        if (val != WeierstrassGroupAffine())
            out << '(' << val.x << ", " << val.y << ')';
        else
            out << "Infinity";
        return out;
    }

    template<typename RNG>
    static WeierstrassGroupAffine random(RNG& rng) {
        boost::random::uniform_int_distribution<uint8_t> ud(0, 1);
        bool ySign = ud(rng);
        while (true) {
            BF x(BF::random(rng));
            BF yy(x * x.square());
            if constexpr (A != BF(0))
                yy += A * x;
            if constexpr (B != BF(0))
                yy += B;
            if (auto maybeY = yy.sqrt()) {
                BF& y = *maybeY;
                if (ySign)
                    y = -y;
                return WeierstrassGroupAffine(x, y);
            }
        }
    }
};

#endif
