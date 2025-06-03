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

#ifndef BLACKNET_CRYPTO_INTEGERRING_H
#define BLACKNET_CRYPTO_INTEGERRING_H

#include <cmath>
#include <charconv>
#include <optional>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>

#include "bitint.h"
#include "semigroup.h"

namespace blacknet::crypto {

template<typename Params>
class IntegerRing {
    using I = Params::I;
    using L = Params::L;
    using UI = Params::UI;
    using UL = Params::UL;

    constexpr IntegerRing(I n, int) : n(n) {}
public:
    consteval static IntegerRing LEFT_ADDITIVE_IDENTITY() { return IntegerRing(0); }
    consteval static IntegerRing LEFT_MULTIPLICATIVE_IDENTITY() { return IntegerRing(1); }

    using BaseRing = IntegerRing;
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
        return freeze(n) == freeze(other.n);
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
        return IntegerRing::LEFT_ADDITIVE_IDENTITY() - *this;
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
        static_assert(Params::is_division_ring, "Not implemented");
        constexpr BitInt<Params::BITS> PHI_MINUS_1 = Params::M - I(2);
        if (*this != IntegerRing(0)) {
            // Euler's theorem
            return semigroup::power(*this, PHI_MINUS_1);
        } else {
            return std::nullopt;
        }
    }

    constexpr bool checkInfinityNorm(const NumericType& bound) const {
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

    friend std::ostream& operator << (std::ostream& out, const IntegerRing& val)
    {
        return out << fromForm(val.n);
    }

    consteval static I modulus() {
        return Params::M;
    }

    constexpr static IntegerRing twiddle(std::size_t index) {
        return IntegerRing(Params::TWIDDLES[index], 0);
    }

    consteval static std::size_t twiddles() {
        return Params::TWIDDLES.size();
    }

    consteval static IntegerRing inverse_twiddles() {
        return IntegerRing(Params::INVERSE_TWIDDLES, 0);
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        sponge.absorb(*this);
    }

    template<typename Sponge>
    constexpr static IntegerRing squeeze(Sponge& sponge) {
        return sponge.squeeze();
    }

    template<std::uniform_random_bit_generator RNG>
    static IntegerRing random(RNG& rng) {
        std::uniform_int_distribution<I> ud(-(Params::M - 1) / 2, (Params::M - 1) / 2);
        return random(rng, ud);
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static IntegerRing random(RNG& rng, DST& dst) {
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
public:
    constexpr static I freeze(I x) {
        if (x >= Params::M)
            return x - Params::M;
        else if (x < 0)
            return x + Params::M;
        else
            return x;
    }
};

}

template<typename Params>
struct fmt::formatter<blacknet::crypto::IntegerRing<Params>> : ostream_formatter {};

#endif
