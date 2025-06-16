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

#ifndef BLACKNET_CRYPTO_WEIERSTRASSGROUPAFFINE_H
#define BLACKNET_CRYPTO_WEIERSTRASSGROUPAFFINE_H

#include <ostream>
#include <random>

#include "abeliangroup.h"
#include "binaryuniformdistribution.h"

namespace blacknet::crypto {

template<typename BF, typename SF, BF A, BF B>
class WeierstrassGroupAffine {
    BF x;
    BF y;
public:
    typedef BF Base;
    typedef SF Scalar;
    consteval static WeierstrassGroupAffine LEFT_ADDITIVE_IDENTITY() { return WeierstrassGroupAffine(); }

    consteval WeierstrassGroupAffine() = default;
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

    constexpr WeierstrassGroupAffine operator * (const SF& other) const {
        return abeliangroup::multiply(*this, other);
    }

    constexpr WeierstrassGroupAffine operator - (const WeierstrassGroupAffine& other) const {
#ifdef BLACKNET_OPTIMIZE
        return *this + -other;
#else
        if (*this == WeierstrassGroupAffine())
            return -other;
        if (other == WeierstrassGroupAffine())
            return *this;

        if (x != other.x) {
            // sub-2024-v
            BF k((other.y + y) / (other.x - x));
            BF xr(k.square() - x - other.x);
            BF yr(k * (xr - x) - y);
            return WeierstrassGroupAffine(xr, yr);
        } else if (y == -other.y) {
            return douple();
        } else {
            return WeierstrassGroupAffine();
        }
#endif
    }

    constexpr WeierstrassGroupAffine& operator += (const WeierstrassGroupAffine& other) {
        return *this = *this + other;
    }

    constexpr WeierstrassGroupAffine& operator -= (const WeierstrassGroupAffine& other) {
        return *this = *this - other;
    }

    constexpr WeierstrassGroupAffine& operator *= (const SF& other) {
        return *this = *this * other;
    }

    friend std::ostream& operator << (std::ostream& out, const WeierstrassGroupAffine& val)
    {
        if (val != WeierstrassGroupAffine())
            out << '(' << val.x << ", " << val.y << ')';
        else
            out << "Infinity";
        return out;
    }

    template<typename Sponge>
    constexpr static WeierstrassGroupAffine squeeze(Sponge& sponge) {
        BF ySign;
        while ((ySign = BF::squeeze(sponge).isQuadraticResidue()) == BF(0))
            continue; // Euler's criterion
        while (true) {
            BF x(BF::squeeze(sponge));
            BF yy(x * x.square());
            if constexpr (A != BF(0))
                yy += A * x;
            if constexpr (B != BF(0))
                yy += B;
            if (auto maybeY = yy.sqrt()) {
                BF& y = *maybeY;
                if (ySign != BF(1))
                    y = -y;
                return WeierstrassGroupAffine(x, y);
            }
        }
    }

    template<std::uniform_random_bit_generator RNG>
    static WeierstrassGroupAffine random(RNG& rng) {
        BinaryUniformDistribution<uint8_t, RNG> bud;
        bool ySign = bud(rng);
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

}

#endif
