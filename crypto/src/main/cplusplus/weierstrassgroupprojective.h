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

#ifndef BLACKNET_CRYPTO_WEIERSTRASSGROUPPROJECTIVE_H
#define BLACKNET_CRYPTO_WEIERSTRASSGROUPPROJECTIVE_H

#include <iostream>
#include <boost/random/uniform_int_distribution.hpp>

#include "semigroup.h"

template<typename BF, typename SF, BF A, BF B>
class WeierstrassGroupProjective {
    BF x;
    BF y;
    BF z;
public:
    typedef BF Base;
    typedef SF Scalar;
    consteval static WeierstrassGroupProjective LEFT_ADDITIVE_IDENTITY() { return WeierstrassGroupProjective(); }

    consteval WeierstrassGroupProjective() : x(0), y(0), z(0) {}
    constexpr WeierstrassGroupProjective(const BF& x, const BF& y, const BF& z) : x{x}, y(y), z(z) {}

    constexpr bool operator == (const WeierstrassGroupProjective& other) const {
        bool i1 = z == BF(0);
        bool i2 = other.z == BF(0);
        if (i1 && i2)
            return true;
        else if (i1 || i2)
            return false;
        else
            return (x * other.z == z * other.x) && (y * other.z == z * other.y);
    }

    constexpr WeierstrassGroupProjective operator - () const {
        if (*this != WeierstrassGroupProjective())
            return WeierstrassGroupProjective(x, -y, z);
        else
            return WeierstrassGroupProjective();
    }

    constexpr WeierstrassGroupProjective operator + (const WeierstrassGroupProjective& other) const {
        if (*this == WeierstrassGroupProjective())
            return other;
        if (other == WeierstrassGroupProjective())
            return *this;

        BF u1(other.y * z);
        BF u2(y * other.z);
        BF v1(other.x * z);
        BF v2(x * other.z);

        if (v1 != v2) {
            // add-1998-cmo-2
            BF u(u1 - u2);
            BF uu(u.square());
            BF v(v1 - v2);
            BF vv(v.square());
            BF vvv(v * vv);
            BF w(z * other.z);
            BF r(vv * v2);
            BF a(uu * w - vvv - r - r);
            BF xr(v * a);
            BF yr(u * (r - a) - vvv * u2);
            BF zr(vvv * w);
            return WeierstrassGroupProjective(xr, yr, zr);
        } else if (u1 == u2) {
            return douple();
        } else {
            return WeierstrassGroupProjective();
        }
    }

    constexpr WeierstrassGroupProjective douple() const {
        if (*this == WeierstrassGroupProjective())
            return WeierstrassGroupProjective();
        // dbl-2007-bl
        BF xx(x.square());
        BF w(xx + xx + xx);
        if constexpr (A != BF(0))
            w += A * z.square();
        BF s2(y * z); s2 += s2;
        BF sss8(s2 * s2.square());
        BF r(y * s2);
        BF rr(r.square());
        BF b((x + r).square() - xx - rr);
        BF h(w.square() - b - b);
        BF xr(h * s2);
        BF yr(w * (b - h) - rr - rr);
        BF zr(sss8);
        return WeierstrassGroupProjective(xr, yr, zr);
    }

    constexpr WeierstrassGroupProjective operator * (const SF& other) const {
        return semigroup::multiply(*this, other);
    }

    constexpr WeierstrassGroupProjective operator - (const WeierstrassGroupProjective& other) const {
#if 3
        return *this + -other;
#else
        if (*this == WeierstrassGroupProjective())
            return -other;
        if (other == WeierstrassGroupProjective())
            return *this;

        BF u1(other.y * z);
        BF u2(y * other.z);
        BF v1(other.x * z);
        BF v2(x * other.z);

        if (v1 != v2) {
            // sub-2024-v
            BF u(u1 + u2);
            BF uu(u.square());
            BF v(v1 - v2);
            BF vv(v.square());
            BF vvv(v * vv);
            BF w(z * other.z);
            BF r(vv * v2);
            BF a(uu * w - vvv - r - r);
            BF xr(v * a);
            BF yr(u * (a - r) - vvv * u2);
            BF zr(vvv * w);
            return WeierstrassGroupProjective(xr, yr, zr);
        } else if (-u1 == u2) {
            return douple();
        } else {
            return WeierstrassGroupProjective();
        }
#endif
    }

    constexpr WeierstrassGroupProjective& operator += (const WeierstrassGroupProjective& other) {
        return *this = *this + other;
    }

    constexpr WeierstrassGroupProjective& operator -= (const WeierstrassGroupProjective& other) {
        return *this = *this - other;
    }

    constexpr WeierstrassGroupProjective& operator *= (const SF& other) {
        return *this = *this * other;
    }

    constexpr WeierstrassGroupProjective scale() const {
        if (*this != WeierstrassGroupProjective()) {
            BF a(z.invert());
            return WeierstrassGroupProjective(x * a, y * a, BF(1));
        } else {
            return WeierstrassGroupProjective();
        }
    }

    friend std::ostream& operator << (std::ostream& out, const WeierstrassGroupProjective& val)
    {
        if (val != WeierstrassGroupProjective())
            out << '(' << val.x << ", " << val.y << ", " << val.z << ')';
        else
            out << "Infinity";
        return out;
    }

    template<typename RNG>
    static WeierstrassGroupProjective random(RNG& rng) {
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
                return WeierstrassGroupProjective(x, y, BF(1));
            }
        }
    }
};

#endif
