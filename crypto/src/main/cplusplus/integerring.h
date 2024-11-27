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

#ifndef BLACKNET_CRYPTO_INTEGERRING_H
#define BLACKNET_CRYPTO_INTEGERRING_H

#include <cmath>
#include <charconv>
#include <iostream>
#include <optional>
#include <boost/random/uniform_int_distribution.hpp>

#include "semigroup.h"

template<typename Params>
class IntegerRing {
    using I = Params::I;
    using L = Params::L;
    using UI = Params::UI;
    using UL = Params::UL;

    constexpr IntegerRing(I n, int) : n(n) {}
public:
    typedef IntegerRing Scalar;
    consteval static IntegerRing LEFT_ADDITIVE_IDENTITY() { return IntegerRing(0); }
    consteval static IntegerRing LEFT_MULTIPLICATIVE_IDENTITY() { return IntegerRing(1); }

    using NumericType = I;

    I n;

    consteval IntegerRing() : n() {}
    consteval IntegerRing(const std::string& hex) {
        // Undefined behaviour is prohibited in consteval
        UI un;
        std::from_chars(hex.data(), hex.data() + sizeof(UI) * 2, un, 16);
        n = I(toForm<UI, UL>(un));
    }
    constexpr IntegerRing(I n) : n(toForm(n)) {}

    constexpr bool operator == (const IntegerRing& other) const {
        return Params::freeze(Params::reduce(n)) == Params::freeze(Params::reduce(other.n));
    }

    constexpr IntegerRing& operator += (const IntegerRing& other) {
        n += other.n;
        n = Params::reduce(n);
        return *this;
    }

    constexpr IntegerRing operator + (const IntegerRing& other) const {
        I t(n + other.n);
        t = Params::reduce(t);
        return IntegerRing(t, 0);
    }

    constexpr IntegerRing& operator *= (const IntegerRing& other) {
        L tt(L(n) * L(other.n));
        n = reduce(tt);
        return *this;
    }

    constexpr IntegerRing operator * (const IntegerRing& other) const {
        L tt(L(n) * L(other.n));
        I t(reduce(tt));
        return IntegerRing(t, 0);
    }

    constexpr IntegerRing& operator -= (const IntegerRing& other) {
        n -= other.n;
        n = Params::reduce(n);
        return *this;
    }

    constexpr IntegerRing operator - (const IntegerRing& other) const {
        I t(n - other.n);
        t = Params::reduce(t);
        return IntegerRing(t, 0);
    }

    constexpr IntegerRing operator - () const {
        if (*this != IntegerRing(0)) {
            I t(Params::M - n);
            return IntegerRing(t, 0);
        } else {
            return IntegerRing(0);
        }
    }

    constexpr IntegerRing douple() const {
        I t(n << 1);
        t = Params::reduce(t);
        return IntegerRing(t, 0);
    }

    constexpr IntegerRing square() const {
        L tt(L(n) * L(n));
        I t(reduce(tt));
        return IntegerRing(t, 0);
    }

    constexpr std::optional<IntegerRing> invert() const {
        if (*this != IntegerRing(0)) {
            // Euler's theorem
            return semigroup::power(*this, PHI_MINUS_1);
        } else {
            return std::nullopt;
        }
    }

    constexpr bool checkInfiniteNorm(const NumericType& bound) const {
        I nn(fromForm(n));
        I t(nn >> (sizeof(I) * 8 - 1));
        t = nn - (t & nn << 1);
        if (t < bound)
            return true;
        return false;
    }

    constexpr I number() const {
        return fromForm(n);
    }

    class BitIterator {
        friend IntegerRing;
        I data;
        std::size_t index;
        constexpr BitIterator(const IntegerRing& e) : data(Params::freeze(fromForm(e.n))), index(0) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = bool;
        constexpr BitIterator& operator = (const BitIterator& other) {
            data = other.data;
            index = other.index;
            return *this;
        }
        constexpr bool operator == (std::default_sentinel_t) const {
            return index == BITS;
        }
        constexpr bool operator * () const {
            return (data >> index) & 1;
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

    friend std::ostream& operator << (std::ostream& out, const IntegerRing& val)
    {
        return out << fromForm(val.n);
    }

    friend std::istream& operator >> (std::istream& in, IntegerRing& val)
    {
        in >> val.n;
        val.n = toForm(val.n);
        return in;
    }

    consteval static I modulus() {
        return Params::M;
    }

    constexpr static IntegerRing zeta(std::size_t index) {
        return IntegerRing(Params::ZETAS[index], 0);
    }

    consteval static std::size_t zetas() {
        return Params::ZETAS.size();
    }

    template<typename DRG>
    constexpr static IntegerRing squeeze(DRG& drg) {
        return drg.squeeze();
    }

    template<typename RNG>
    static IntegerRing random(RNG& rng) {
        boost::random::uniform_int_distribution<I> ud(-(Params::M - 1) / 2, (Params::M - 1) / 2);
        return random(rng, ud);
    }

    template<typename RNG, typename DST>
    static IntegerRing random(RNG& rng, const DST& dst) {
        return IntegerRing(dst(rng));
    }
private:
    template<typename MRI = I, typename MRL = L>
    constexpr static MRI reduce(MRL x) {
        // Partial Montgomery reduction
        MRI t(x * MRI(Params::RN));
        return (x - MRL(t) * MRL(Params::M)) >> sizeof(MRI) * 8;
    }
    template<typename MRI = I, typename MRL = L>
    constexpr static MRI toForm(MRI n) {
        return reduce<MRI, MRL>(MRL(n) * MRL(Params::R2));
    }
    template<typename MRI = I, typename MRL = L>
    constexpr static MRI fromForm(MRI n) {
        return reduce<MRI, MRL>(MRL(n));
    }

    constexpr static const std::size_t BITS = std::log2(Params::M);
    constexpr static const I PHI_MINUS_1 = Params::M - I(2);
};

#endif
