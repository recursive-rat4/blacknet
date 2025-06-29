/*
 * Copyright (c) 2025 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_Z2_H
#define BLACKNET_CRYPTO_Z2_H

#include <cstdint>
#include <optional>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>

#include "binaryuniformdistribution.h"

namespace blacknet::crypto {

struct Z2 {
private:
    using I = std::int8_t;
    using UI = std::uint8_t;

    constexpr Z2(I n, int) : n(n) {}
public:
    constexpr static const bool is_integer_ring = true;
    consteval static Z2 additive_identity() { return Z2(0); }
    consteval static Z2 multiplicative_identity() { return Z2(1); }

    using BaseRing = Z2;
    using NumericType = I;

    I n;

    consteval Z2() noexcept = default;
    constexpr Z2(I n) : n(n & 1) {}

    constexpr bool operator == (const Z2& other) const = default;

    constexpr Z2& operator += (const Z2& other) {
        n ^= other.n;
        return *this;
    }

    constexpr Z2 operator + (const Z2& other) const {
        I t = n ^ other.n;
        return Z2(t, 0);
    }

    constexpr Z2& operator *= (const Z2& other) {
        n &= other.n;
        return *this;
    }

    constexpr Z2 operator * (const Z2& other) const {
        I t = n & other.n;
        return Z2(t, 0);
    }

    constexpr Z2& operator -= (const Z2& other) {
        n ^= other.n;
        return *this;
    }

    constexpr Z2 operator - (const Z2& other) const {
        I t = n ^ other.n;
        return Z2(t, 0);
    }

    constexpr Z2 operator - () const {
        return *this;
    }

    constexpr Z2 douple() const {
        return additive_identity();
    }

    constexpr Z2 square() const {
        return *this;
    }

    constexpr std::optional<Z2> invert() const {
        if (*this != Z2(0)) {
            return multiplicative_identity();
        } else {
            return std::nullopt;
        }
    }

    constexpr bool checkInfinityNorm(const NumericType& bound) const {
        return absolute() < bound;
    }

    constexpr double euclideanNorm() const {
        return absolute();
    }

    constexpr I canonical() const {
        return n;
    }

    constexpr I balanced() const {
        return n;
    }

    constexpr I absolute() const {
        return n;
    }

    friend std::ostream& operator << (std::ostream& out, const Z2& val)
    {
        return out << int(val.n);
    }

    consteval static std::size_t bits() {
        return 1;
    }

    consteval static UI characteristic() {
        return 2;
    }

    consteval static UI modulus() {
        return 2;
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        sponge.absorb(*this);
    }

    template<typename Sponge>
    constexpr static Z2 squeeze(Sponge& sponge) {
        return sponge.squeeze();
    }

    template<typename Sponge, typename DST>
    constexpr static Z2 squeeze(Sponge& sponge, DST& dst) {
        return Z2(dst(sponge));
    }

    template<std::uniform_random_bit_generator RNG>
    static Z2 random(RNG& rng) {
        BinaryUniformDistributionRNG<I, RNG> bud;
        return random(rng, bud);
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static Z2 random(RNG& rng, DST& dst) {
        return Z2(dst(rng));
    }
};

}

template<>
struct fmt::formatter<blacknet::crypto::Z2> : ostream_formatter {};

#endif
