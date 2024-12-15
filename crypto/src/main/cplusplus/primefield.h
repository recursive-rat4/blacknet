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

#include <iostream>
#include <iterator>
#include <optional>

#include "bigint.h"
#include "semigroup.h"

template<typename Params>
class PrimeField {
    constexpr PrimeField(const UInt256& n, int) : n(n) {}
public:
    typedef PrimeField Scalar;
    consteval static PrimeField LEFT_ADDITIVE_IDENTITY() { return PrimeField(0); }
    consteval static PrimeField LEFT_MULTIPLICATIVE_IDENTITY() { return PrimeField(1); }

    UInt256 n;

    consteval PrimeField() : n() {}
    consteval PrimeField(const std::string& hex) : n(Params::toForm(hex)) {}
    constexpr PrimeField(uint8_t n) : n(Params::toForm(n)) {}

    constexpr bool operator == (const PrimeField&) const = default;

    constexpr PrimeField& operator += (const PrimeField& other) {
        n += other.n;
        if (n >= Params::M)
            n -= Params::M;
        return *this;
    }

    constexpr PrimeField operator + (const PrimeField& other) const {
        UInt256 t(n + other.n);
        if (t >= Params::M)
            t -= Params::M;
        return PrimeField(t, 0);
    }

    constexpr PrimeField& operator *= (const PrimeField& other) {
        UInt512 tt(n * other.n);
        n = Params::reduce(tt);
        return *this;
    }

    constexpr PrimeField operator * (const PrimeField& other) const {
        UInt512 tt(n * other.n);
        UInt256 t(Params::reduce(tt));
        return PrimeField(t, 0);
    }

    constexpr PrimeField& operator -= (const PrimeField& other) {
        n -= other.n;
        if (n >= Params::M)
            n += Params::M;
        return *this;
    }

    constexpr PrimeField operator - (const PrimeField& other) const {
        UInt256 t(n - other.n);
        if (t >= Params::M)
            t += Params::M;
        return PrimeField(t, 0);
    }

    constexpr PrimeField& operator /= (const PrimeField& other) noexcept(false) {
        return *this *= other.invert();
    }

    constexpr PrimeField operator / (const PrimeField& other) const noexcept(false) {
        return *this * other.invert();
    }

    constexpr PrimeField operator - () const {
        if (*this != PrimeField(0)) {
            UInt256 t(Params::M - n);
            return PrimeField(t, 0);
        } else {
            return PrimeField(0);
        }
    }

    constexpr PrimeField douple() const {
        UInt256 t(n.douple());
        if (t >= Params::M)
            t -= Params::M;
        return PrimeField(t, 0);
    }

    constexpr PrimeField square() const {
        UInt512 tt(n.square());
        UInt256 t(Params::reduce(tt));
        return PrimeField(t, 0);
    }

    constexpr PrimeField invert() const noexcept(false) {
        if (*this != PrimeField(0)) {
            // Euler's theorem
            return semigroup::power(*this, PHI_MINUS_1);
        } else {
            throw ArithmeticException("Noninvertible field element");
        }
    }

    constexpr std::optional<PrimeField> sqrt() const {
        // Tonelli–Shanks algorithm
        using namespace semigroup;
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
        UInt256 data;
        std::size_t index;
        constexpr BitIterator(const PrimeField& e) : data(Params::fromForm(e.n)), index(0) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = bool;
        constexpr BitIterator& operator = (const BitIterator& other) {
            data = other.data;
            index = other.index;
            return *this;
        }
        constexpr bool operator == (std::default_sentinel_t) const {
            return index == Params::B;
        }
        constexpr bool operator * () const {
            return data[index];
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
    constexpr BitIterator bitsBegin() const noexcept {
        return BitIterator(*this);
    }
    consteval std::default_sentinel_t bitsEnd() const noexcept {
        return std::default_sentinel;
    }

    friend std::ostream& operator << (std::ostream& out, const PrimeField& val)
    {
        return out << Params::fromForm(val.n);
    }

    template<typename DRG>
    constexpr void absorb(DRG& drg) const {
        drg.absorb(*this);
    }

    template<typename DRG>
    constexpr static PrimeField squeeze(DRG& drg) {
        return drg.squeeze();
    }

    template<typename RNG>
    static PrimeField random(RNG& rng) {
        UInt256 t(UInt256::random(rng));
        while (t >= Params::M)
            t = UInt256::random(rng);
        return PrimeField(t, 0);
    }
public:
    constexpr PrimeField isQuadraticResidue() const {
        // Legendre symbol
        return semigroup::power(*this, P_MINUS_1_HALVED);
    }
private:
    constexpr static const PrimeField PHI_MINUS_1 = PrimeField(Params::toForm(Params::M - UInt256(2)), 0);
    constexpr static const PrimeField P_MINUS_1_HALVED = PrimeField(Params::toForm(Params::P_MINUS_1_HALVED), 0);
    constexpr static const PrimeField Q = PrimeField(Params::toForm(Params::Q), 0);
    constexpr static const PrimeField S = PrimeField(Params::toForm(Params::S), 0);
    constexpr static const PrimeField Q_PLUS_1_HALVED = PrimeField(Params::toForm(Params::Q_PLUS_1_HALVED), 0);
};

#endif
