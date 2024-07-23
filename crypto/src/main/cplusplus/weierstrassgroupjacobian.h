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

#ifndef BLACKNET_CRYPTO_WEIERSTRASSGROUPJACOBIAN_H
#define BLACKNET_CRYPTO_WEIERSTRASSGROUPJACOBIAN_H

#include <iostream>
#include <boost/random/uniform_int_distribution.hpp>

#include "abeliangroup.h"

template<typename BF, typename SF, BF A, BF B>
class WeierstrassGroupJacobian {
    BF x;
    BF y;
    BF z;
public:
    typedef BF Base;
    typedef SF Scalar;
    consteval static WeierstrassGroupJacobian LEFT_ADDITIVE_IDENTITY() { return WeierstrassGroupJacobian(); }

    consteval WeierstrassGroupJacobian() : x(0), y(0), z(0) {}
    constexpr WeierstrassGroupJacobian(const BF& x, const BF& y, const BF& z) : x{x}, y(y), z(z) {}

    constexpr bool operator == (const WeierstrassGroupJacobian& other) const {
        bool i1 = z == BF(0);
        bool i2 = other.z == BF(0);
        if (i1 && i2) {
            return true;
        } else if (i1 || i2) {
            return false;
        } else {
            BF z1z1(z.square());
            BF z1z1z1(z1z1 * z);
            BF z2z2(other.z.square());
            BF z2z2z2(z2z2 * other.z);
            return (x * z2z2 == z1z1 * other.x) && (y * z2z2z2 == z1z1z1 * other.y);
        }
    }

    constexpr WeierstrassGroupJacobian operator - () const {
        if (*this != WeierstrassGroupJacobian())
            return WeierstrassGroupJacobian(x, -y, z);
        else
            return WeierstrassGroupJacobian();
    }

    constexpr WeierstrassGroupJacobian operator + (const WeierstrassGroupJacobian& other) const {
        if (*this == WeierstrassGroupJacobian())
            return other;
        if (other == WeierstrassGroupJacobian())
            return *this;

        BF z1z1(z.square());
        BF z2z2(other.z.square());
        BF u1(x * z2z2);
        BF u2(other.x * z1z1);
        BF v1(y * other.z * z2z2);
        BF v2(other.y * z * z1z1);

        if (u1 != u2) {
            // add-1998-cmo-2
            BF u(u2 - u1);
            BF uu(u.square());
            BF uuu(u * uu);
            BF v(v2 - v1);
            BF vv(v.square());
            BF h(u1 * uu);
            BF xr(vv - uuu - h - h);
            BF yr(v * (h - xr) - v1 * uuu);
            BF zr(z * other.z * u);
            return WeierstrassGroupJacobian(xr, yr, zr);
        } else if (v1 == v2) {
            return douple();
        } else {
            return WeierstrassGroupJacobian();
        }
    }

    constexpr WeierstrassGroupJacobian douple() const {
        if (*this == WeierstrassGroupJacobian())
            return WeierstrassGroupJacobian();
        // dbl-1986-cc
        BF xx(x.square());
        BF yy(y.square());
        BF yz(y * z);
        BF yyyy8(yy.square()); yyyy8 += yyyy8; yyyy8 += yyyy8; yyyy8 += yyyy8;
        BF s(x * yy); s += s; s += s;
        BF m(xx + xx + xx);
        if constexpr (A != BF(0))
            m += A * z.square().square();
        BF t(m.square() - s - s);
        BF& xr = t;
        BF yr(m * (s - t) - yyyy8);
        BF zr(yz + yz);
        return WeierstrassGroupJacobian(xr, yr, zr);
    }

    constexpr WeierstrassGroupJacobian operator * (const SF& other) const {
        return abeliangroup::multiply(*this, other);
    }

    constexpr WeierstrassGroupJacobian operator - (const WeierstrassGroupJacobian& other) const {
#if 3
        return *this + -other;
#else
        if (*this == WeierstrassGroupJacobian())
            return -other;
        if (other == WeierstrassGroupJacobian())
            return *this;

        BF z1z1(z.square());
        BF z2z2(other.z.square());
        BF u1(x * z2z2);
        BF u2(other.x * z1z1);
        BF v1(y * other.z * z2z2);
        BF v2(other.y * z * z1z1);

        if (u1 != u2) {
            // sub-2024-v
            BF u(u2 - u1);
            BF uu(u.square());
            BF uuu(u * uu);
            BF v(v2 + v1);
            BF vv(v.square());
            BF h(u1 * uu);
            BF xr(vv - uuu - h - h);
            BF yr(v * (xr - h) - v1 * uuu);
            BF zr(z * other.z * u);
            return WeierstrassGroupJacobian(xr, yr, zr);
        } else if (v1 == -v2) {
            return douple();
        } else {
            return WeierstrassGroupJacobian();
        }
#endif
    }

    constexpr WeierstrassGroupJacobian& operator += (const WeierstrassGroupJacobian& other) {
        return *this = *this + other;
    }

    constexpr WeierstrassGroupJacobian& operator -= (const WeierstrassGroupJacobian& other) {
        return *this = *this - other;
    }

    constexpr WeierstrassGroupJacobian& operator *= (const SF& other) {
        return *this = *this * other;
    }

    constexpr WeierstrassGroupJacobian scale() const {
        if (*this != WeierstrassGroupJacobian()) {
            BF a(z.invert());
            BF aa(a.square());
            BF aaa(a * aa);
            return WeierstrassGroupJacobian(x * aa, y * aaa, BF(1));
        } else {
            return WeierstrassGroupJacobian();
        }
    }

    friend std::ostream& operator << (std::ostream& out, const WeierstrassGroupJacobian& val)
    {
        if (val != WeierstrassGroupJacobian())
            out << '(' << val.x << ", " << val.y << ", " << val.z << ')';
        else
            out << "Infinity";
        return out;
    }

    template<typename RNG>
    static WeierstrassGroupJacobian random(RNG& rng) {
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
                return WeierstrassGroupJacobian(x, y, BF(1));
            }
        }
    }
};

#endif
