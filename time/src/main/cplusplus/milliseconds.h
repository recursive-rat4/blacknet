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

#ifndef BLACKNET_TIME_MILLISECONDS_H
#define BLACKNET_TIME_MILLISECONDS_H

#include <cstdint>
#include <chrono>
#include <compare>
#include <limits>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>

namespace blacknet::time {

// A timestamp or a time interval measured in milliseconds. The value may be negative.
class Milliseconds {
public:
    using NumericType = std::int64_t;
private:
    NumericType n;
public:
    consteval Milliseconds() : n() {}
    constexpr Milliseconds(NumericType n) : n(n) {}

    constexpr bool operator == (const Milliseconds&) const = default;
    constexpr std::strong_ordering operator <=> (const Milliseconds&) const = default;

    constexpr Milliseconds& operator += (const Milliseconds& other) {
        n += other.n;
        return *this;
    }

    constexpr Milliseconds operator + (const Milliseconds& other) const {
        return n + other.n;
    }

    constexpr Milliseconds& operator *= (const NumericType& other) {
        n *= other;
        return *this;
    }

    constexpr Milliseconds operator * (const NumericType& other) const {
        return n * other;
    }

    constexpr Milliseconds& operator -= (const Milliseconds& other) {
        n -= other.n;
        return *this;
    }

    constexpr Milliseconds operator - (const Milliseconds& other) const {
        return n - other.n;
    }

    constexpr NumericType operator / (const Milliseconds& other) const {
        return n / other.n;
    }

    constexpr Milliseconds& operator /= (const NumericType& other) {
        n /= other;
        return *this;
    }

    constexpr Milliseconds operator / (const NumericType& other) const {
        return n / other;
    }

    constexpr Milliseconds& operator %= (const Milliseconds& other) {
        n %= other.n;
        return *this;
    }

    constexpr Milliseconds operator % (const Milliseconds& other) const {
        return n % other.n;
    }

    constexpr Milliseconds& operator %= (const NumericType& other) {
        n %= other;
        return *this;
    }

    constexpr Milliseconds operator % (const NumericType& other) const {
        return n % other;
    }

    constexpr Milliseconds operator + () const {
        return +n;
    }

    constexpr Milliseconds operator - () const {
        return -n;
    }

    constexpr NumericType number() const {
        return n;
    }

    consteval static Milliseconds zero() {
        return 0;
    }

    consteval static Milliseconds min() {
        return std::numeric_limits<NumericType>::min();
    }

    consteval static Milliseconds max() {
        return std::numeric_limits<NumericType>::max();
    }

    friend std::ostream& operator << (std::ostream& out, const Milliseconds& val)
    {
        return out << val.n;
    }

    template<std::uniform_random_bit_generator RNG>
    static Milliseconds random(RNG& rng) {
        std::uniform_int_distribution<NumericType> ud;
        return random(rng, ud);
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static Milliseconds random(RNG& rng, DST& dst) {
        return dst(rng);
    }
};

consteval Milliseconds operator ""_seconds(const char* str, std::size_t size) {
    typename Milliseconds::NumericType value;
    std::from_chars(str, str + size, value);
    return Milliseconds(value * 1000);
}

consteval Milliseconds operator ""_minutes(const char* str, std::size_t size) {
    typename Milliseconds::NumericType value;
    std::from_chars(str, str + size, value);
    return Milliseconds(value * 60000);
}

consteval Milliseconds operator ""_hours(const char* str, std::size_t size) {
    typename Milliseconds::NumericType value;
    std::from_chars(str, str + size, value);
    return Milliseconds(value * 3600000);
}

consteval Milliseconds operator ""_days(const char* str, std::size_t size) {
    typename Milliseconds::NumericType value;
    std::from_chars(str, str + size, value);
    return Milliseconds(value * 86400000);
}

}

template<>
struct fmt::formatter<blacknet::time::Milliseconds> : ostream_formatter {};

#endif
