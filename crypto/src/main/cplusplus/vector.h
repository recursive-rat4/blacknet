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

#ifndef BLACKNET_CRYPTO_VECTOR_H
#define BLACKNET_CRYPTO_VECTOR_H

#include <initializer_list>
#include <iostream>
#include <vector>
#include <boost/io/ostream_joiner.hpp>

template<typename E>
class Vector {
public:
    std::vector<E> elements;

    constexpr Vector(std::size_t size) : elements(size) {}
    constexpr Vector(std::size_t size, const E& fill) : elements(size, fill) {}
    constexpr Vector(std::initializer_list<E> init) : elements(init) {}
    constexpr Vector(Vector&& other) noexcept : elements(std::move(other.elements)) {}

    constexpr bool operator == (const Vector&) const = default;

    constexpr E& operator [] (std::size_t i) {
        return elements[i];
    }

    constexpr const E& operator [] (std::size_t i) const {
        return elements[i];
    }

    constexpr Vector& operator += (const Vector& other) {
        std::size_t size = elements.size();
        for (std::size_t i = 0; i < size; ++i)
            elements[i] += other.elements[i];
        return *this;
    }

    constexpr Vector operator + (const Vector& other) const {
        std::size_t size = elements.size();
        Vector r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] + other.elements[i];
        return r;
    }

    constexpr Vector& operator *= (const Vector& other) {
        std::size_t size = elements.size();
        for (std::size_t i = 0; i < size; ++i)
            elements[i] *= other.elements[i];
        return *this;
    }

    constexpr Vector operator * (const Vector& other) const {
        std::size_t size = elements.size();
        Vector r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] * other.elements[i];
        return r;
    }

    constexpr Vector operator * (const E& other) const {
        std::size_t size = elements.size();
        Vector r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] * other;
        return r;
    }

    friend std::ostream& operator << (std::ostream& out, const Vector& val)
    {
        out << '[';
        std::copy(val.elements.begin(), val.elements.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }
};

#endif
