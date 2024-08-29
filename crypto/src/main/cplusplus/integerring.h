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

#include <charconv>
#include <iostream>
#include <boost/random/uniform_int_distribution.hpp>

template<
    typename I,
    typename L,
    typename UI,
    typename UL,
    I M,
    I R2,
    I RN,
    I(*REDUCE)(I),
    I(*FREEZE)(I)
>
class IntegerRing {
    constexpr IntegerRing(I n, int) : n(n) {}
public:
    typedef I value_type;

    typedef IntegerRing Scalar;
    consteval static IntegerRing LEFT_ADDITIVE_IDENTITY() { return IntegerRing(0); }
    consteval static IntegerRing LEFT_MULTIPLICATIVE_IDENTITY() { return IntegerRing(1); }

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
        return FREEZE(REDUCE(n)) == FREEZE(REDUCE(other.n));
    }

    constexpr IntegerRing& operator += (const IntegerRing& other) {
        n += other.n;
        n = REDUCE(n);
        return *this;
    }

    constexpr IntegerRing operator + (const IntegerRing& other) const {
        I t(n + other.n);
        t = REDUCE(t);
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
        n = REDUCE(n);
        return *this;
    }

    constexpr IntegerRing operator - (const IntegerRing& other) const {
        I t(n - other.n);
        t = REDUCE(t);
        return IntegerRing(t, 0);
    }

    constexpr IntegerRing douple() const {
        I t(n << 1);
        t = REDUCE(t);
        return IntegerRing(t, 0);
    }

    constexpr IntegerRing square() const {
        L tt(L(n) * L(n));
        I t(reduce(tt));
        return IntegerRing(t, 0);
    }

    constexpr bool checkInfiniteNorm(I bound) const {
        I nn(fromForm(n));
        I t(nn >> (sizeof(I) * 8 - 1));
        t = nn - (t & nn << 1);
        if (t < bound)
            return true;
        return false;
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

    template<typename DRG>
    constexpr static IntegerRing squeeze(DRG& drg) {
        return drg.squeeze();
    }

    template<typename RNG>
    static IntegerRing random(RNG& rng) {
        boost::random::uniform_int_distribution<I> ud(-(M - 1) / 2, (M - 1) / 2);
        return IntegerRing(ud(rng));
    }
private:
    template<typename MRI = I, typename MRL = L>
    constexpr static MRI reduce(MRL x) {
        // Partial Montgomery reduction
        MRI t(x * MRI(RN));
        return (x - MRL(t) * MRL(M)) >> sizeof(MRI) * 8;
    }
    template<typename MRI = I, typename MRL = L>
    constexpr static MRI toForm(MRI n) {
        return reduce<MRI, MRL>(MRL(n) * MRL(R2));
    }
    template<typename MRI = I, typename MRL = L>
    constexpr static MRI fromForm(MRI n) {
        return reduce<MRI, MRL>(MRL(n));
    }
};

#endif
