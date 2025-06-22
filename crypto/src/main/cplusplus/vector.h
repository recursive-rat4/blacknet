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

#ifndef BLACKNET_CRYPTO_VECTOR_H
#define BLACKNET_CRYPTO_VECTOR_H

#include <algorithm>
#include <initializer_list>
#include <ostream>
#include <random>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename E>class Matrix;

template<typename E>
class Vector {
public:
    using ElementType = E;

    constexpr static Vector identity(std::size_t size) { return Vector(size, E(1)); }

    std::vector<E> elements;

    consteval Vector() = default;
    constexpr Vector(std::size_t size) : elements(size) {}
    constexpr Vector(std::size_t size, const E& fill) : elements(size, fill) {}
    constexpr Vector(std::initializer_list<E> init) : elements(init) {}
    constexpr Vector(std::vector<E>&& elements) : elements(std::move(elements)) {}
    constexpr Vector(const Vector&) = default;
    constexpr Vector(Vector&&) noexcept = default;
    constexpr ~Vector() noexcept = default;

    constexpr Vector& operator = (const Vector&) = default;
    constexpr Vector& operator = (Vector&&) noexcept = default;

    constexpr bool operator == (const Vector&) const = default;

    constexpr std::size_t size() const noexcept {
        return elements.size();
    }

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

    friend constexpr Vector operator * (const E& lps, const Vector& rps) {
        std::size_t size = rps.elements.size();
        Vector r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = lps * rps.elements[i];
        return r;
    }

    constexpr Vector& operator -= (const Vector& other) {
        std::size_t size = elements.size();
        for (std::size_t i = 0; i < size; ++i)
            elements[i] -= other.elements[i];
        return *this;
    }

    constexpr Vector operator - (const Vector& other) const {
        std::size_t size = elements.size();
        Vector r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] - other.elements[i];
        return r;
    }

    constexpr Vector operator - () const {
        std::size_t size = elements.size();
        Vector r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = - elements[i];
        return r;
    }

    constexpr Vector operator || (const Vector& other) const {
        Vector r(size() + other.size());
        for (std::size_t i = 0; i < size(); ++i)
            r.elements[i] = elements[i];
        for (std::size_t i = 0; i < other.size(); ++i)
            r.elements[i + size()] = other.elements[i];
        return r;
    }

    constexpr E dot(const Vector& other) const {
        E sigma(E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < elements.size(); ++i)
            sigma += elements[i] * other.elements[i];
        return sigma;
    }

    constexpr Matrix<E> tensor(const Vector& other) const {
        std::size_t m = elements.size();
        std::size_t n = other.elements.size();
        Matrix<E> r(m, n);
        for (std::size_t i = 0; i < m; ++i)
            for (std::size_t j = 0; j < n; ++j)
                r[i, j] = elements[i] * other.elements[j];
        return r;
    }

    constexpr bool checkInfinityNorm(const E::NumericType& bound) const {
        return std::ranges::all_of(elements, [&bound](const E& e) {
            return e.checkInfinityNorm(bound);
        });
    }

    constexpr double euclideanNorm() const {
        double t = 0;
        for (std::size_t i = 0; i < elements.size(); ++i) {
            double e = elements[i].euclideanNorm();
            t += e * e;
        }
        return std::sqrt(t);
    }

    constexpr decltype(auto) begin() noexcept {
        return elements.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return elements.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return elements.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return elements.end();
    }

    friend std::ostream& operator << (std::ostream& out, const Vector& val)
    {
        fmt::print(out, "{}", val.elements);
        return out;
    }

    template<typename Sponge>
    constexpr static Vector squeeze(Sponge& sponge, std::size_t size) {
        Vector t(size);
        std::ranges::generate(t.elements, [&] { return E::squeeze(sponge); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG>
    static Vector random(RNG& rng, std::size_t size) {
        Vector t(size);
        std::ranges::generate(t.elements, [&] { return E::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static Vector random(RNG& rng, DST& dst, std::size_t size) {
        Vector t(size);
        std::ranges::generate(t.elements, [&] { return E::random(rng, dst); });
        return t;
    }

template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;

    Builder& circuit;
    std::vector<LinearCombination> elements;

    constexpr Circuit(Builder& circuit)
        : circuit(circuit), elements() {}
    constexpr Circuit(Builder& circuit, std::size_t size)
        : circuit(circuit), elements(size) {}
    constexpr Circuit(Builder& circuit, Variable::Type type, std::size_t size)
        : circuit(circuit), elements(size)
    {
        std::ranges::generate(elements, [&]{ return circuit.variable(type); });
    }

    constexpr std::size_t size() const noexcept {
        return elements.size();
    }

    constexpr LinearCombination& operator [] (std::size_t i) {
        return elements[i];
    }

    constexpr const LinearCombination& operator [] (std::size_t i) const {
        return elements[i];
    }

    constexpr LinearCombination dot(const Circuit& other) const {
        LinearCombination sigma;
        for (std::size_t i = 0; i < elements.size(); ++i) {
            auto t = circuit.auxiliary();
            circuit(t == elements[i] * other.elements[i]);
            sigma += t;
        }
        return sigma;
    }

    constexpr decltype(auto) begin() noexcept {
        return elements.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return elements.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return elements.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return elements.end();
    }
};

struct Tracer {
    Vector vector;
    std::vector<E>& trace;

    constexpr std::size_t size() const noexcept {
        return vector.size();
    }

    constexpr E& operator [] (std::size_t i) {
        return vector[i];
    }

    constexpr const E& operator [] (std::size_t i) const {
        return vector[i];
    }

    constexpr E dot(const Tracer& other) const {
        E sigma(E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < vector.size(); ++i)
            sigma += trace.emplace_back(
                vector[i] * other[i]
            );
        return sigma;
    }

    constexpr decltype(auto) begin() noexcept {
        return vector.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return vector.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return vector.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return vector.end();
    }
};
};

}

#endif
