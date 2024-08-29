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

#ifndef BLACKNET_CRYPTO_PRIMEFIELDBARRETT_H
#define BLACKNET_CRYPTO_PRIMEFIELDBARRETT_H

#include <iostream>
#include <iterator>
#include <optional>

#include "bigint.h"
#include "semigroup.h"

template<
    std::size_t B,
    UInt256 M,
    UInt512 M2,
    UInt256 PHI_MINUS_1,
    UInt256 P_MINUS_1_HALVED,
    UInt256 Q,
    UInt256 S,
    UInt256 Q_PLUS_1_HALVED
>
class PrimeFieldBarrett {
    constexpr PrimeFieldBarrett(const UInt256& n) : n(n) {}
public:
    typedef PrimeFieldBarrett Scalar;
    consteval static PrimeFieldBarrett LEFT_ADDITIVE_IDENTITY() { return PrimeFieldBarrett(0); }
    consteval static PrimeFieldBarrett LEFT_MULTIPLICATIVE_IDENTITY() { return PrimeFieldBarrett(1); }

    UInt256 n;

    consteval PrimeFieldBarrett() : n() {}
    consteval PrimeFieldBarrett(const std::string& hex) : n(hex) {}
    constexpr PrimeFieldBarrett(uint8_t n) : n(n) {}

    constexpr bool operator == (const PrimeFieldBarrett&) const = default;

    constexpr PrimeFieldBarrett& operator += (const PrimeFieldBarrett& other) {
        n += other.n;
        if (n >= M)
            n -= M;
        return *this;
    }

    constexpr PrimeFieldBarrett operator + (const PrimeFieldBarrett& other) const {
        UInt256 t(n + other.n);
        if (t >= M)
            t -= M;
        return PrimeFieldBarrett(t);
    }

    constexpr PrimeFieldBarrett& operator *= (const PrimeFieldBarrett& other) {
        UInt512 tt(n * other.n);
        n = reduce(tt);
        return *this;
    }

    constexpr PrimeFieldBarrett operator * (const PrimeFieldBarrett& other) const {
        UInt512 tt(n * other.n);
        UInt256 t(reduce(tt));
        return PrimeFieldBarrett(t);
    }

    constexpr PrimeFieldBarrett& operator -= (const PrimeFieldBarrett& other) {
        n -= other.n;
        if (n >= M)
            n += M;
        return *this;
    }

    constexpr PrimeFieldBarrett operator - (const PrimeFieldBarrett& other) const {
        UInt256 t(n - other.n);
        if (t >= M)
            t += M;
        return PrimeFieldBarrett(t);
    }

    constexpr PrimeFieldBarrett& operator /= (const PrimeFieldBarrett& other) noexcept(false) {
        return *this *= other.invert();
    }

    constexpr PrimeFieldBarrett operator / (const PrimeFieldBarrett& other) const noexcept(false) {
        return *this * other.invert();
    }

    constexpr PrimeFieldBarrett operator - () const {
        if (*this != PrimeFieldBarrett(0)) {
            UInt256 t(M - n);
            return PrimeFieldBarrett(t);
        } else {
            return PrimeFieldBarrett(0);
        }
    }

    constexpr PrimeFieldBarrett douple() const {
        UInt256 t(n.douple());
        if (t >= M)
            t -= M;
        return PrimeFieldBarrett(t);
    }

    constexpr PrimeFieldBarrett square() const {
        UInt512 tt(n.square());
        UInt256 t(reduce(tt));
        return PrimeFieldBarrett(t);
    }

    constexpr PrimeFieldBarrett invert() const noexcept(false) {
        if (*this != PrimeFieldBarrett(0)) {
            // Euler's theorem
            return semigroup::power(*this, PHI_MINUS_1);
        } else {
            throw ArithmeticException("Noninvertible field element");
        }
    }

    constexpr std::optional<PrimeFieldBarrett> sqrt() const {
        // Tonelli–Shanks algorithm
        using namespace semigroup;
        auto iqr = isQuadraticResidue();
        if (iqr == PrimeFieldBarrett(1)) {
            PrimeFieldBarrett z(2);
            while (z.isQuadraticResidue() == PrimeFieldBarrett(1))
                z += PrimeFieldBarrett(1);
            PrimeFieldBarrett m(S);
            PrimeFieldBarrett c(power(z, Q));
            PrimeFieldBarrett t(power(*this, Q));
            PrimeFieldBarrett r(power(*this, Q_PLUS_1_HALVED));
            while (true) {
                if (t == PrimeFieldBarrett(0)) {
                    return PrimeFieldBarrett(0);
                } else if (t == PrimeFieldBarrett(1)) {
                    return r;
                } else {
                    PrimeFieldBarrett i(1);
                    while (power(t, power(PrimeFieldBarrett(2), i)) != PrimeFieldBarrett(1))
                        i += PrimeFieldBarrett(1);
                    PrimeFieldBarrett b(power(c, power(PrimeFieldBarrett(2), m - i - PrimeFieldBarrett(1))));
                    m = i;
                    c = b.square();
                    t *= c;
                    r *= b;
                }
            }
        } else if (iqr == PrimeFieldBarrett(0)) {
            return PrimeFieldBarrett(0);
        } else {
            return std::nullopt;
        }
    }

    class BitIterator {
        friend PrimeFieldBarrett;
        const UInt256* data;
        std::size_t index;
        constexpr BitIterator(const PrimeFieldBarrett& e) : data(&e.n), index(0) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = bool;
        constexpr BitIterator& operator = (const BitIterator& other) {
            data = other.data;
            index = other.index;
            return *this;
        }
        constexpr bool operator == (std::default_sentinel_t) const {
            return index == B;
        }
        constexpr bool operator * () const {
            return (*data)[index];
        }
        constexpr BitIterator& operator ++ () {
            ++index;
            return *this;
        }
        constexpr BitIterator operator ++ (int) {
            BitIterator old(*this);
            ++*this;
            return old;
        }
    };
    static_assert(std::input_iterator<BitIterator>);
    constexpr BitIterator bitsBegin() const {
        return BitIterator(*this);
    }
    consteval std::default_sentinel_t bitsEnd() const {
        return std::default_sentinel;
    }

    friend std::ostream& operator << (std::ostream& out, const PrimeFieldBarrett& val)
    {
        return out << val.n;
    }

    template<typename DRG>
    constexpr static PrimeFieldBarrett squeeze(DRG& drg) {
        return drg.squeeze();
    }

    template<typename RNG>
    static PrimeFieldBarrett random(RNG& rng) {
        UInt256 t(UInt256::random(rng));
        while (t >= M)
            t = UInt256::random(rng);
        return PrimeFieldBarrett(t);
    }
private:
    constexpr static UInt256 reduce(const UInt512& x) {
        // Barrett reduction
        UInt1024 ttt(x * M2);
        UInt256 t1{ttt.limbs[11], ttt.limbs[10], ttt.limbs[9], ttt.limbs[8]};
        UInt512 tt(x - t1 * M);
        UInt256 t2{tt.limbs[3], tt.limbs[2], tt.limbs[1], tt.limbs[0]};
        if (t2 >= M)
            t2 -= M;
        return t2;
    }
public:
    constexpr PrimeFieldBarrett isQuadraticResidue() const {
        // Legendre symbol
        return semigroup::power(*this, P_MINUS_1_HALVED);
    }
private:
};

#endif
