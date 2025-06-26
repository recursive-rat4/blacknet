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

#ifndef BLACKNET_CRYPTO_TWISTEDEDWARDSGROUPEXTENDED_H
#define BLACKNET_CRYPTO_TWISTEDEDWARDSGROUPEXTENDED_H

#include "blacknet-config.h"

#include <ostream>
#include <random>

#include "abeliangroup.h"
#include "binaryuniformdistribution.h"

namespace blacknet::crypto {

template<typename BF, typename SR, BF A, BF D>
class TwistedEdwardsGroupExtended {
    BF x;
    BF y;
    BF z;
    BF t;
public:
    typedef BF Base;
    typedef SR Scalar;
    consteval static TwistedEdwardsGroupExtended LEFT_ADDITIVE_IDENTITY() { return TwistedEdwardsGroupExtended(); }

    consteval TwistedEdwardsGroupExtended() : x(0), y(1), z(1), t(0) {}
    constexpr TwistedEdwardsGroupExtended(const BF& x, const BF& y) : x{x}, y(y), z(BF(1)), t(x * y) {}
    constexpr TwistedEdwardsGroupExtended(const BF& x, const BF& y, const BF& z, const BF& t) : x{x}, y(y), z(z), t(t) {}

    constexpr bool operator == (const TwistedEdwardsGroupExtended& other) const {
        return (x * other.z == z * other.x) && (y * other.z == z * other.y);
    }

    constexpr TwistedEdwardsGroupExtended operator - () const {
        return TwistedEdwardsGroupExtended(-x, y, z, -t);
    }

    constexpr TwistedEdwardsGroupExtended operator + (const TwistedEdwardsGroupExtended& other) const {
        // add-2008-hwcd-2
        BF x1x2 = x * other.x;
        BF y1y2 = y * other.y;
        BF z1t2 = z * other.t;
        BF z2t1 = t * other.z;
        BF e = z2t1 + z1t2;
        BF f = (x - y) * (other.x + other.y) + y1y2 - x1x2;
        BF g;
        if constexpr (A == BF(-1))
            g = y1y2 - x1x2;
        else
            g = y1y2 + A * x1x2;
        BF h = z2t1 - z1t2;
        BF xr = e * f;
        BF yr = g * h;
        BF zr = f * g;
        BF tr = e * h;
        return TwistedEdwardsGroupExtended(xr, yr, zr, tr);
    }

    constexpr TwistedEdwardsGroupExtended douple() const {
        // dbl-2008-hwcd
        BF xx = x.square();
        BF yy = y.square();
        BF zz2 = z.square().douple();
        BF d;
        if constexpr (A == BF(-1))
            d = -xx;
        else
            d = A * xx;
        BF e = (x + y).square() - xx - yy;
        BF g = d + yy;
        BF f = g - zz2;
        BF h = d - yy;
        BF xr = e * f;
        BF yr = g * h;
        BF zr = f * g;
        BF tr = e * h;
        return TwistedEdwardsGroupExtended(xr, yr, zr, tr);
    }

    constexpr TwistedEdwardsGroupExtended operator * (const Scalar& other) const {
        return abeliangroup::multiply(*this, other);
    }

    constexpr TwistedEdwardsGroupExtended operator - (const TwistedEdwardsGroupExtended& other) const {
#ifdef BLACKNET_OPTIMIZE
        return *this + -other;
#else
        // sub-2025-v
        BF x1x2 = x * other.x;
        BF y1y2 = y * other.y;
        BF z1t2 = z * other.t;
        BF z2t1 = t * other.z;
        BF e = z2t1 - z1t2;
        BF f = (x - y) * (other.y - other.x) + y1y2 + x1x2;
        BF g;
        if constexpr (A == BF(-1))
            g = y1y2 + x1x2;
        else
            g = y1y2 - A * x1x2;
        BF h = z2t1 + z1t2;
        BF xr = e * f;
        BF yr = g * h;
        BF zr = f * g;
        BF tr = e * h;
        return TwistedEdwardsGroupExtended(xr, yr, zr, tr);
#endif
    }

    constexpr TwistedEdwardsGroupExtended& operator += (const TwistedEdwardsGroupExtended& other) {
        return *this = *this + other;
    }

    constexpr TwistedEdwardsGroupExtended& operator -= (const TwistedEdwardsGroupExtended& other) {
        return *this = *this - other;
    }

    constexpr TwistedEdwardsGroupExtended& operator *= (const Scalar& other) {
        return *this = *this * other;
    }

    friend std::ostream& operator << (std::ostream& out, const TwistedEdwardsGroupExtended& val)
    {
        return out << '(' << val.x << ", " << val.y << ", " << val.z << ", " << val.t << ')';
    }

    template<typename Sponge>
    constexpr static TwistedEdwardsGroupExtended squeeze(Sponge& sponge) {
        BinaryUniformDistributionSponge<Sponge> bud;
        bool ySign = bud(sponge) != 0;
        while (true) {
            BF x = BF::squeeze(sponge);
            BF xx = x.square();
            BF n;
            if constexpr (A == BF(-1))
                n = -xx - BF(1);
            else
                n = A * xx - BF(1);
            BF d = D * xx - BF(1);
            BF yy = n / d;
            if (auto maybeY = yy.sqrt()) {
                BF& y = *maybeY;
                if (ySign)
                    y = -y;
                return TwistedEdwardsGroupExtended(x, y);
            }
        }
    }

    template<std::uniform_random_bit_generator RNG>
    static TwistedEdwardsGroupExtended random(RNG& rng) {
        BinaryUniformDistributionRNG<uint8_t, RNG> bud;
        bool ySign = bud(rng);
        while (true) {
            BF x = BF::random(rng);
            BF xx = x.square();
            BF n;
            if constexpr (A == BF(-1))
                n = -xx - BF(1);
            else
                n = A * xx - BF(1);
            BF d = D * xx - BF(1);
            BF yy = n / d;
            if (auto maybeY = yy.sqrt()) {
                BF& y = *maybeY;
                if (ySign)
                    y = -y;
                return TwistedEdwardsGroupExtended(x, y);
            }
        }
    }
};

}

#endif
