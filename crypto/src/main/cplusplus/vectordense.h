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

#ifndef BLACKNET_CRYPTO_VECTORDENSE_H
#define BLACKNET_CRYPTO_VECTORDENSE_H

#include <algorithm>
#include <concepts>
#include <initializer_list>
#include <ostream>
#include <random>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename E>class MatrixDense;

template<typename E>
class VectorDense {
public:
    using ElementType = E;

    constexpr static VectorDense identity(std::size_t size) { return VectorDense(size, E(1)); }

    std::vector<E> elements;

    consteval VectorDense() = default;
    constexpr VectorDense(std::size_t size) : elements(size) {}
    constexpr VectorDense(std::size_t size, const E& fill) : elements(size, fill) {}
    constexpr VectorDense(std::initializer_list<E> init) : elements(init) {}
    constexpr VectorDense(std::vector<E>&& elements) : elements(std::move(elements)) {}
    constexpr VectorDense(const VectorDense&) = default;
    constexpr VectorDense(VectorDense&&) noexcept = default;
    constexpr ~VectorDense() noexcept = default;

    constexpr VectorDense& operator = (const VectorDense&) = default;
    constexpr VectorDense& operator = (VectorDense&&) noexcept = default;

    constexpr bool operator == (const VectorDense&) const = default;

    constexpr std::size_t size() const noexcept {
        return elements.size();
    }

    constexpr E& operator [] (std::size_t i) {
        return elements[i];
    }

    constexpr const E& operator [] (std::size_t i) const {
        return elements[i];
    }

    constexpr VectorDense& operator += (const VectorDense& other) {
        std::size_t size = elements.size();
        for (std::size_t i = 0; i < size; ++i)
            elements[i] += other.elements[i];
        return *this;
    }

    constexpr VectorDense operator + (const VectorDense& other) const {
        std::size_t size = elements.size();
        VectorDense r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] + other.elements[i];
        return r;
    }

    constexpr VectorDense& operator *= (const VectorDense& other) {
        std::size_t size = elements.size();
        for (std::size_t i = 0; i < size; ++i)
            elements[i] *= other.elements[i];
        return *this;
    }

    constexpr VectorDense operator * (const VectorDense& other) const {
        std::size_t size = elements.size();
        VectorDense r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] * other.elements[i];
        return r;
    }

    constexpr VectorDense operator * (const E& other) const {
        std::size_t size = elements.size();
        VectorDense r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] * other;
        return r;
    }

    friend constexpr VectorDense operator * (const E& lps, const VectorDense& rps) {
        std::size_t size = rps.elements.size();
        VectorDense r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = lps * rps.elements[i];
        return r;
    }

    constexpr VectorDense& operator -= (const VectorDense& other) {
        std::size_t size = elements.size();
        for (std::size_t i = 0; i < size; ++i)
            elements[i] -= other.elements[i];
        return *this;
    }

    constexpr VectorDense operator - (const VectorDense& other) const {
        std::size_t size = elements.size();
        VectorDense r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = elements[i] - other.elements[i];
        return r;
    }

    constexpr VectorDense operator - () const {
        std::size_t size = elements.size();
        VectorDense r(size);
        for (std::size_t i = 0; i < size; ++i)
            r.elements[i] = - elements[i];
        return r;
    }

    constexpr VectorDense operator || (const VectorDense& other) const {
        VectorDense r(size() + other.size());
        for (std::size_t i = 0; i < size(); ++i)
            r.elements[i] = elements[i];
        for (std::size_t i = 0; i < other.size(); ++i)
            r.elements[i + size()] = other.elements[i];
        return r;
    }

    constexpr E dot(const VectorDense& other) const {
        E sigma(E::additive_identity());
        for (std::size_t i = 0; i < elements.size(); ++i)
            sigma += elements[i] * other.elements[i];
        return sigma;
    }

    constexpr MatrixDense<E> tensor(const VectorDense& other) const {
        std::size_t m = elements.size();
        std::size_t n = other.elements.size();
        MatrixDense<E> r(m, n);
        for (std::size_t i = 0; i < m; ++i)
            for (std::size_t j = 0; j < n; ++j)
                r[i, j] = elements[i] * other.elements[j];
        return r;
    }

    template<typename NormType>
    requires(std::same_as<NormType, typename E::NumericType>)
    constexpr bool checkInfinityNorm(const NormType& bound) const {
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

    friend std::ostream& operator << (std::ostream& out, const VectorDense& val)
    {
        fmt::print(out, "{}", val.elements);
        return out;
    }

    template<typename Sponge>
    constexpr static VectorDense squeeze(Sponge& sponge, std::size_t size) {
        VectorDense t(size);
        std::ranges::generate(t.elements, [&] { return E::squeeze(sponge); });
        return t;
    }

    template<typename Sponge, typename DST>
    constexpr static VectorDense squeeze(Sponge& sponge, DST& dst, std::size_t size) {
        VectorDense t(size);
        std::ranges::generate(t.elements, [&] { return E::squeeze(sponge, dst); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG>
    static VectorDense random(RNG& rng, std::size_t size) {
        VectorDense t(size);
        std::ranges::generate(t.elements, [&] { return E::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static VectorDense random(RNG& rng, DST& dst, std::size_t size) {
        VectorDense t(size);
        std::ranges::generate(t.elements, [&] { return E::random(rng, dst); });
        return t;
    }

template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;

    Builder* circuit;
    std::vector<LinearCombination> elements;

    constexpr Circuit(Builder* circuit)
        : circuit(circuit), elements() {}
    constexpr Circuit(Builder* circuit, std::size_t size)
        : circuit(circuit), elements(size) {}
    constexpr Circuit(Builder* circuit, Variable::Type type, std::size_t size)
        : circuit(circuit), elements(size)
    {
        std::ranges::generate(elements, [&]{ return circuit->variable(type); });
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
        auto scope = circuit->scope("Vector::dot");
        LinearCombination sigma;
        for (std::size_t i = 0; i < elements.size(); ++i) {
            auto t = circuit->auxiliary();
            scope(t == elements[i] * other.elements[i]);
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

template<std::size_t Degree>
struct Assigner {
    VectorDense vector;
    std::vector<E>* assigment;

    constexpr Assigner(std::size_t size, const E& fill, std::vector<E>* assigment)
        : vector(size, fill), assigment(assigment) {}
    constexpr Assigner(const VectorDense& vector, std::vector<E>* assigment)
        : vector(vector), assigment(assigment) {}
    constexpr Assigner(VectorDense&& vector, std::vector<E>* assigment)
        : vector(std::move(vector)), assigment(assigment) {}

    constexpr std::size_t size() const noexcept {
        return vector.size();
    }

    constexpr E& operator [] (std::size_t i) {
        return vector[i];
    }

    constexpr const E& operator [] (std::size_t i) const {
        return vector[i];
    }

    constexpr E dot(const Assigner& other) const {
        E sigma(E::additive_identity());
        for (std::size_t i = 0; i < vector.size(); ++i)
            sigma += assigment->emplace_back(
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
