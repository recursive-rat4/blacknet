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

#ifndef BLACKNET_CRYPTO_PRIMEFIELD_H
#define BLACKNET_CRYPTO_PRIMEFIELD_H

#include <iostream>
#include <iterator>
#include <optional>

#include "bitint.h"
#include "bigint.h"
#include "semigroup.h"

template<typename Params>
class PrimeField {
    constexpr PrimeField(const UInt256& n, int) : n(n) {}
public:
    consteval static PrimeField LEFT_ADDITIVE_IDENTITY() { return PrimeField(0); }
    consteval static PrimeField LEFT_MULTIPLICATIVE_IDENTITY() { return PrimeField(1); }

    UInt256 n;

    consteval PrimeField() : n() {}
    consteval PrimeField(const std::string& hex) : n(Params::toForm(hex)) {}
    constexpr PrimeField(int8_t n) {
        if (n >= 0)
            this->n = Params::toForm(n);
        else
            this->n = Params::toForm(Params::M - UInt256(-n));
    }

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
        if (auto maybeInv = other.invert()) {
            return *this *= *maybeInv;
        } else {
            throw ArithmeticException("Noninvertible field element");
        }
    }

    constexpr PrimeField operator / (const PrimeField& other) const noexcept(false) {
        if (auto maybeInv = other.invert()) {
            return *this * *maybeInv;
        } else {
            throw ArithmeticException("Noninvertible field element");
        }
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

    constexpr std::optional<PrimeField> invert() const {
        if (*this != PrimeField(0)) {
            // Euler's theorem
            return semigroup::power(*this, PHI_MINUS_1);
        } else {
            return std::nullopt;
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
            PrimeField c(power(z, Params::Q));
            PrimeField t(power(*this, Params::Q));
            PrimeField r(power(*this, Params::Q_PLUS_1_HALVED));
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

    constexpr UInt256 number() const {
        return Params::fromForm(n);
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
            return index == Params::BITS;
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
        return semigroup::power(*this, Params::P_MINUS_1_HALVED);
    }
private:
    constexpr static const BitInt<Params::BITS> PHI_MINUS_1 = Params::M - UInt256(2);
    constexpr static const PrimeField S = PrimeField(Params::toForm(Params::S), 0);
};

#endif
