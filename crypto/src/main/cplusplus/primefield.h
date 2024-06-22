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

#ifndef BLACKNET_CRYPTO_PRIMEFIELD_H
#define BLACKNET_CRYPTO_PRIMEFIELD_H

#include <exception>
#include <iostream>
#include <iterator>
#include <optional>

#include "bigint.h"
#include "semigroup.h"

class ArithmeticException : public std::exception {
    std::string message;
public:
    ArithmeticException(const std::string& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

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
class PrimeField {
    constexpr PrimeField(const UInt256& n) : n(n) {}
public:
    typedef PrimeField Scalar;
    consteval static PrimeField LEFT_ADDITIVE_IDENTITY() { return PrimeField(0); }
    consteval static PrimeField LEFT_MULTIPLICATIVE_IDENTITY() { return PrimeField(1); }

    UInt256 n;

    consteval PrimeField() : n() {}
    consteval PrimeField(uint8_t n) : n(n) {}

    constexpr bool operator == (const PrimeField&) const = default;

    constexpr PrimeField& operator += (const PrimeField& other) {
        n += other.n;
        if (n >= M)
            n -= M;
        return *this;
    }

    constexpr PrimeField operator + (const PrimeField& other) const {
        UInt256 t(n + other.n);
        if (t >= M)
            t -= M;
        return PrimeField(t);
    }

    constexpr PrimeField& operator *= (const PrimeField& other) {
        UInt512 tt(n * other.n);
        n = reduce(tt);
        return *this;
    }

    constexpr PrimeField operator * (const PrimeField& other) const {
        UInt512 tt(n * other.n);
        UInt256 t(reduce(tt));
        return PrimeField(t);
    }

    constexpr PrimeField& operator -= (const PrimeField& other) {
        n -= other.n;
        if (n >= M)
            n += M;
        return *this;
    }

    constexpr PrimeField operator - (const PrimeField& other) const {
        UInt256 t(n - other.n);
        if (t >= M)
            t += M;
        return PrimeField(t);
    }

    constexpr PrimeField& operator /= (const PrimeField& other) noexcept(false) {
        return *this *= other.invert();
    }

    constexpr PrimeField operator / (const PrimeField& other) const noexcept(false) {
        return *this * other.invert();
    }

    constexpr PrimeField operator - () const {
        if (*this != PrimeField(0)) {
            UInt256 t(M - n);
            return PrimeField(t);
        } else {
            return PrimeField(0);
        }
    }

    constexpr PrimeField square() const {
        UInt512 tt(n.square());
        UInt256 t(reduce(tt));
        return PrimeField(t);
    }

    constexpr PrimeField invert() const noexcept(false) {
        if (*this != PrimeField(0)) {
            // Euler's theorem
            return power(*this, PHI_MINUS_1);
        } else {
            throw ArithmeticException("Noninvertible field element");
        }
    }

    constexpr std::optional<PrimeField> sqrt() const {
        // Tonelli–Shanks algorithm
        auto iqr = isQuadraticResidue();
        if (iqr == PrimeField(1)) {
            PrimeField z(2);
            while (z.isQuadraticResidue() == PrimeField(1))
                z += PrimeField(1);
            PrimeField m(S);
            PrimeField c(power(z, Q));
            PrimeField t(power(*this, Q));
            PrimeField r(power(*this, Q_PLUS_1_HALVED));
            while (true) {
                if (t == PrimeField(0)) {
                    return PrimeField(0);
                } else if (t == PrimeField(1)) {
                    return r;
                } else {
                    PrimeField i(1);
                    while (power(t, power(PrimeField(2), i)) != PrimeField(1))
                        i += PrimeField(1);
                    PrimeField b(power(c, power(PrimeField(2), m - i - PrimeField(1))));
                    m = i;
                    c = b.square();
                    t *= c;
                    r *= b;
                }
            }
        } else if (iqr == PrimeField(0)) {
            return PrimeField(0);
        } else {
            return std::nullopt;
        }
    }

    class BitIterator {
        friend PrimeField;
        const UInt256* data;
        std::size_t index;
        constexpr BitIterator(const PrimeField& e) : data(&e.n), index(0) {}
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

    friend std::ostream& operator << (std::ostream& out, const PrimeField& val)
    {
        return out << val.n;
    }

    friend std::istream& operator >> (std::istream& in, PrimeField& val)
    {
        return in >> val.n;
    }

    template<typename RNG>
    static PrimeField random(RNG& rng) {
        UInt256 t(UInt256::random(rng));
        while (t >= M)
            t = UInt256::random(rng);
        return PrimeField(t);
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

    constexpr PrimeField isQuadraticResidue() const {
        // Legendre symbol
        return power(*this, P_MINUS_1_HALVED);
    }
};

#endif
