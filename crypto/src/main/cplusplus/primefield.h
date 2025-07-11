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

#include <iterator>
#include <optional>
#include <ostream>
#include <random>

#include "bitint.h"
#include "bigint.h"
#include "semigroup.h"

namespace blacknet::crypto {

template<typename Params>
class PrimeField {
    constexpr PrimeField(const UInt256& n, int) : n(n) {}
public:
    constexpr static const bool is_integer_ring = true;
    consteval static PrimeField additive_identity() { return PrimeField(0); }
    consteval static PrimeField multiplicative_identity() { return PrimeField(1); }

    using BaseRing = PrimeField;
    using NumericType = UInt256;

    UInt256 n;

    consteval PrimeField() noexcept = default;
    consteval PrimeField(const std::string& hex) : n(Params::toForm(hex)) {}
    constexpr PrimeField(int8_t n) {
        if (n >= 0)
            this->n = Params::toForm(n);
        else
            this->n = Params::toForm(Params::M - UInt256(-n));
    }
    constexpr PrimeField(const UInt256& n) : n(Params::toForm(n)) {}

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
        if constexpr (Params::has_sparse_modulus) {
            constexpr static const BitInt<Params::BITS> PHI_MINUS_1 = Params::M - UInt256(2);
            if (*this != PrimeField(0)) {
                // Euler's theorem
                return semigroup::power(*this, PHI_MINUS_1);
            } else {
                return std::nullopt;
            }
        } else {
            // Extended Binary GCD (classic algorithm)
            // https://eprint.iacr.org/2020/972
            constexpr PrimeField TWO_INVERTED = PrimeField(Params::toForm(Params::TWO_INVERTED), 0);
            UInt256 a(canonical());
            UInt256 b(modulus());
            PrimeField c(1);
            PrimeField d(0);
            while (a != UInt256(0)) {
                if (a.isEven()) {
                    a = a.halve();
                    c *= TWO_INVERTED;
                } else {
                    if (a < b) {
                        std::swap(a, b);
                        std::swap(c, d);
                    }
                    a -= b;
                    a = a.halve();
                    c -= d;
                    c *= TWO_INVERTED;
                }
            }
            if (b != 1)
                return std::nullopt;
            return d;
        }
    }

    constexpr std::optional<PrimeField> sqrt() const {
        // Tonelli–Shanks algorithm
        using namespace semigroup;
        constexpr PrimeField S = PrimeField(Params::toForm(Params::S), 0);
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

    constexpr UInt256 canonical() const {
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
        consteval BitIterator() noexcept = default;
        constexpr BitIterator(const BitIterator&) = default;
        constexpr BitIterator& operator = (const BitIterator&) = default;
        constexpr bool operator == (const BitIterator& other) const {
            return index == other.index;
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
    static_assert(std::forward_iterator<BitIterator>);
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

    consteval static std::size_t bits() {
        return Params::BITS;
    }

    consteval static UInt256 characteristic() {
        return Params::M;
    }

    consteval static UInt256 modulus() {
        return Params::M;
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        sponge.absorb(*this);
    }

    template<typename Sponge>
    constexpr static PrimeField squeeze(Sponge& sponge) {
        return sponge.squeeze();
    }

    template<std::uniform_random_bit_generator RNG>
    static PrimeField random(RNG& rng) {
        UInt256 t(UInt256::random(rng));
        while (t >= Params::M)
            t = UInt256::random(rng);
        return PrimeField(t, 0);
    }
private:
    constexpr PrimeField isQuadraticResidue() const {
        // Legendre symbol
        return semigroup::power(*this, Params::P_MINUS_1_HALVED);
    }
};

}

#endif
