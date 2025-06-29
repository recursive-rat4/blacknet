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

#ifndef BLACKNET_CRYPTO_TWISTEDEDWARDSGROUPAFFINE_H
#define BLACKNET_CRYPTO_TWISTEDEDWARDSGROUPAFFINE_H

#include <ostream>
#include <random>

#include "abeliangroup.h"
#include "binaryuniformdistribution.h"

namespace blacknet::crypto {

template<typename BF, typename SR, BF A, BF D>
class TwistedEdwardsGroupAffine {
    BF x;
    BF y;
public:
    typedef BF Base;
    typedef SR Scalar;
    consteval static TwistedEdwardsGroupAffine additive_identity() { return TwistedEdwardsGroupAffine(); }

    consteval TwistedEdwardsGroupAffine() : x(0), y(1) {}
    constexpr TwistedEdwardsGroupAffine(const BF& x, const BF& y) : x{x}, y(y) {}

    constexpr bool operator == (const TwistedEdwardsGroupAffine& other) const = default;

    constexpr TwistedEdwardsGroupAffine operator - () const {
        return TwistedEdwardsGroupAffine(-x, y);
    }

    constexpr TwistedEdwardsGroupAffine operator + (const TwistedEdwardsGroupAffine& other) const {
        BF x1x2 = x * other.x;
        BF y1y2 = y * other.y;
        BF k = D * x1x2 * y1y2;
        BF xr = (x * other.y + y * other.x) / (BF(1) + k);
        BF yr;
        if constexpr (A == BF(-1))
            yr = (y1y2 + x1x2) / (BF(1) - k);
        else
            yr = (y1y2 - A * x1x2) / (BF(1) - k);
        return TwistedEdwardsGroupAffine(xr, yr);
    }

    constexpr TwistedEdwardsGroupAffine douple() const {
        BF xx = x.square();
        BF yy = y.square();
        BF k = D * xx * yy;
        BF xr = (x * y).douple() / (BF(1) + k);
        BF yr;
        if constexpr (A == BF(-1))
            yr = (yy + xx) / (BF(1) - k);
        else
            yr = (yy - A * xx) / (BF(1) - k);
        return TwistedEdwardsGroupAffine(xr, yr);
    }

    constexpr TwistedEdwardsGroupAffine operator * (const Scalar& other) const {
        return abeliangroup::multiply(*this, other);
    }

    constexpr TwistedEdwardsGroupAffine operator - (const TwistedEdwardsGroupAffine& other) const {
        // sub-2025-v
        BF x1x2 = x * other.x;
        BF y1y2 = y * other.y;
        BF k = D * x1x2 * y1y2;
        BF xr = (x * other.y - y * other.x) / (BF(1) - k);
        BF yr;
        if constexpr (A == BF(-1))
            yr = (y1y2 - x1x2) / (BF(1) + k);
        else
            yr = (y1y2 + A * x1x2) / (BF(1) + k);
        return TwistedEdwardsGroupAffine(xr, yr);
    }

    constexpr TwistedEdwardsGroupAffine& operator += (const TwistedEdwardsGroupAffine& other) {
        return *this = *this + other;
    }

    constexpr TwistedEdwardsGroupAffine& operator -= (const TwistedEdwardsGroupAffine& other) {
        return *this = *this - other;
    }

    constexpr TwistedEdwardsGroupAffine& operator *= (const Scalar& other) {
        return *this = *this * other;
    }

    friend std::ostream& operator << (std::ostream& out, const TwistedEdwardsGroupAffine& val)
    {
        return out << '(' << val.x << ", " << val.y << ')';
    }

    template<typename Sponge>
    constexpr static TwistedEdwardsGroupAffine squeeze(Sponge& sponge) {
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
                return TwistedEdwardsGroupAffine(x, y);
            }
        }
    }

    template<std::uniform_random_bit_generator RNG>
    static TwistedEdwardsGroupAffine random(RNG& rng) {
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
                return TwistedEdwardsGroupAffine(x, y);
            }
        }
    }
};

}

#endif
