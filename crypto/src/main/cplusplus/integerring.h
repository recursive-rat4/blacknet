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

#include <iostream>
#include <boost/random/uniform_int_distribution.hpp>

template<
    typename I,
    typename L,
    I M,
    I R2,
    I RN,
    I(*REDUCE)(I)
>
class IntegerRing {
    constexpr IntegerRing(I n, int) : n(n) {}
public:
    typedef IntegerRing Scalar;
    consteval static IntegerRing LEFT_ADDITIVE_IDENTITY() { return IntegerRing(0); }
    consteval static IntegerRing LEFT_MULTIPLICATIVE_IDENTITY() { return IntegerRing(1); }

    I n;

    consteval IntegerRing() : n() {}
    constexpr IntegerRing(I n) : n(toForm(n)) {}

    constexpr bool operator == (const IntegerRing& other) const {
        return REDUCE(n) == REDUCE(other.n);
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
        L tt(n * other.n);
        n = reduce(tt);
        return *this;
    }

    constexpr IntegerRing operator * (const IntegerRing& other) const {
        L tt(n * other.n);
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

    template<typename RNG>
    static IntegerRing random(RNG& rng) {
        boost::random::uniform_int_distribution<I> ud(-(M - 1) / 2, (M - 1) / 2);
        return IntegerRing(ud(rng));
    }
private:
    constexpr static I reduce(L x) {
        // Signed Montgomery reduction
        I t(x * RN);
        return (x - L(t) * L(M)) >> sizeof(I) * 8;
    }
    constexpr static I toForm(I n) {
        return reduce(L(n) * L(R2));
    }
    constexpr static I fromForm(I n) {
        return reduce(L(n));
    }
};

#endif
