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

#ifndef BLACKNET_CRYPTO_POLYNOMIALRINGNTT_H
#define BLACKNET_CRYPTO_POLYNOMIALRINGNTT_H

#include <algorithm>
#include <array>
#include <bit>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

#include "semigroup.h"
#include "polynomialring.h"

namespace blacknet::crypto {

template<typename Params>
struct PolynomialRingNTT {
private:
    using Z = Params::Z;
    constexpr static const std::size_t N = Params::N;
    constexpr static const std::size_t inertia = N / Z::twiddles();
public:
    using Isomorphism = PolynomialRing<typename Params::Isomorphism>;

    consteval static PolynomialRingNTT additive_identity() {
        PolynomialRingNTT t;
        std::ranges::fill(t.spectrum, Z::additive_identity());
        return t;
    }
    consteval static PolynomialRingNTT multiplicative_identity() {
        return Z::multiplicative_identity();
    }

    using BaseRing = Z;
    using NumericType = Z::NumericType;

    std::array<Z, N> spectrum;

    consteval PolynomialRingNTT() noexcept = default;
    constexpr PolynomialRingNTT(const Isomorphism& e) : spectrum(e.coefficients) {
        Params::NTT::cooley_tukey(spectrum);
    }
    constexpr PolynomialRingNTT(const Z& e) {
        for (std::size_t i = 0; i < N; ++i) {
            if (i % inertia == 0)
                spectrum[i] = e;
            else
                spectrum[i] = Z::additive_identity();
        }
    }
    constexpr PolynomialRingNTT(std::initializer_list<Z> init) {
        std::ranges::copy(init, spectrum.begin());
        std::fill_n(spectrum.begin() + init.size(), N - init.size(), Z(0));
        Params::NTT::cooley_tukey(spectrum);
    }

    constexpr bool operator == (const PolynomialRingNTT&) const = default;

    consteval static std::size_t size() noexcept {
        return dimension();
    }

    consteval static std::size_t dimension() noexcept {
        return Params::N;
    }

    constexpr Z& operator [] (std::size_t i) {
        return spectrum[i];
    }

    constexpr const Z& operator [] (std::size_t i) const {
        return spectrum[i];
    }

    constexpr PolynomialRingNTT& operator += (const PolynomialRingNTT& other) {
        for (std::size_t i = 0; i < N; ++i)
            spectrum[i] += other.spectrum[i];
        return *this;
    }

    constexpr PolynomialRingNTT operator + (const PolynomialRingNTT& other) const {
        PolynomialRingNTT t;
        for (std::size_t i = 0; i < N; ++i)
            t.spectrum[i] = spectrum[i] + other.spectrum[i];
        return t;
    }

    constexpr PolynomialRingNTT& operator *= (const PolynomialRingNTT& other) {
        if constexpr (inertia == 1) {
            for (std::size_t i = 0; i < N; ++i)
                spectrum[i] *= other.spectrum[i];
            return *this;
        } else {
            return *this = *this * other;
        }
    }

    constexpr PolynomialRingNTT operator * (const PolynomialRingNTT& other) const {
        PolynomialRingNTT t(PolynomialRingNTT::additive_identity());
        Params::convolute(t.spectrum, this->spectrum, other.spectrum);
        return t;
    }

    constexpr PolynomialRingNTT& operator *= (const Z& other) {
        for (std::size_t i = 0; i < N; ++i)
            spectrum[i] *= other;
        return *this;
    }

    constexpr PolynomialRingNTT operator * (const Z& other) const {
        PolynomialRingNTT t;
        for (std::size_t i = 0; i < N; ++i)
            t.spectrum[i] = spectrum[i] * other;
        return t;
    }

    friend constexpr PolynomialRingNTT operator * (const Z& lps, const PolynomialRingNTT& rps) {
        PolynomialRingNTT t;
        for (std::size_t i = 0; i < N; ++i)
            t.spectrum[i] = lps * rps.spectrum[i];
        return t;
    }

    constexpr PolynomialRingNTT& operator -= (const PolynomialRingNTT& other) {
        for (std::size_t i = 0; i < N; ++i)
            spectrum[i] -= other.spectrum[i];
        return *this;
    }

    constexpr PolynomialRingNTT operator - (const PolynomialRingNTT& other) const {
        PolynomialRingNTT t;
        for (std::size_t i = 0; i < N; ++i)
            t.spectrum[i] = spectrum[i] - other.spectrum[i];
        return t;
    }

    constexpr PolynomialRingNTT operator - () const {
        PolynomialRingNTT t;
        for (std::size_t i = 0; i < N; ++i)
            t.spectrum[i] = - spectrum[i];
        return t;
    }

    constexpr PolynomialRingNTT douple() const {
        if constexpr (Z::characteristic() != 2) {
            PolynomialRingNTT t;
            for (std::size_t i = 0; i < N; ++i)
                t.spectrum[i] = spectrum[i].douple();
            return t;
        } else {
            return additive_identity();
        }
    }

    constexpr PolynomialRingNTT square() const {
        if constexpr (inertia == 1) {
            PolynomialRingNTT t;
            for (std::size_t i = 0; i < N; ++i)
                t.spectrum[i] = spectrum[i].square();
            return t;
        } else {
            return *this * *this;
        }
    }

    constexpr bool checkInfinityNorm(const NumericType& bound) const {
        return isomorph().checkInfinityNorm(bound);
    }

    constexpr double euclideanNorm() const {
        return isomorph().euclideanNorm();
    }

    constexpr PolynomialRingNTT conjugate() const {
        static_assert(std::has_single_bit(Params::cyclotomic_index));
        if constexpr (inertia == 1) {
            PolynomialRingNTT t(*this);
            for (std::size_t i = 0; i < N / 2; ++i) {
                std::swap(t.spectrum[i], t.spectrum[N - 1 - i]);
            }
            return t;
        } else {
            return isomorph().conjugate();
        }
    }

    constexpr Isomorphism isomorph() const {
        Isomorphism t;
        t.coefficients = spectrum;
        Params::NTT::gentleman_sande(t.coefficients);
        return t;
    }

    constexpr decltype(auto) begin() noexcept {
        return spectrum.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return spectrum.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return spectrum.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return spectrum.end();
    }

    friend std::ostream& operator << (std::ostream& out, const PolynomialRingNTT& val)
    {
        fmt::print(out, "{}", val.spectrum);
        return out;
    }

    consteval static auto characteristic() {
        return Z::characteristic();
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        sponge.absorb(spectrum);
    }

    template<typename Sponge>
    constexpr static PolynomialRingNTT squeeze(Sponge& sponge) {
        PolynomialRingNTT t;
        sponge.squeeze(t.spectrum);
        return t;
    }

    template<std::uniform_random_bit_generator RNG>
    static PolynomialRingNTT random(RNG& rng) {
        PolynomialRingNTT t;
        std::ranges::generate(t.spectrum, [&] { return Z::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static PolynomialRingNTT random(RNG& rng, DST& dst) {
        Isomorphism t;
        std::ranges::generate(t.coefficients, [&] { return Z::random(rng, dst); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static PolynomialRingNTT random(RNG& rng, DST& dst, std::size_t hamming) {
        std::uniform_int_distribution<std::size_t> uid(0, N - 1);
        Isomorphism t;
        std::ranges::fill(t.coefficients, Z(0));
        while (hamming) {
            std::size_t i = uid(rng);
            if (t.coefficients[i] == Z(0)) {
                if ((t.coefficients[i] = dst(rng)) != Z(0))
                    --hamming;
            }
        }
        return t;
    }

template<typename Builder>
requires(std::same_as<Z, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using Convolution = Params::Convolution::template Circuit<Builder>;
    using IsomorphismCircuit = Isomorphism::template Circuit<Builder>;
    using NTT = Params::NTT::template Circuit<Builder>;

    Builder* circuit;
    Convolution convolution;
    NTT ntt;
    std::array<LinearCombination, N> spectrum;

    constexpr Circuit(Builder* circuit)
        : circuit(circuit), convolution(circuit), ntt(circuit), spectrum() {}
    constexpr Circuit(Builder* circuit, Variable::Type type)
        : circuit(circuit), convolution(circuit), ntt(circuit)
    {
        std::ranges::generate(spectrum, [&]{ return circuit->variable(type); });
    }
    constexpr Circuit(const IsomorphismCircuit& e)
        : circuit(e.circuit), convolution(e.circuit), ntt(e.circuit), spectrum(e.coefficients)
    {
        ntt.cooley_tukey(spectrum);
    }

    constexpr LinearCombination& operator [] (std::size_t i) {
        return spectrum[i];
    }

    constexpr const LinearCombination& operator [] (std::size_t i) const {
        return spectrum[i];
    }

    constexpr Circuit& operator += (const Circuit& other) {
        for (std::size_t i = 0; i < N; ++i)
            spectrum[i] += other.spectrum[i];
        return *this;
    }

    constexpr Circuit operator + (const Circuit& other) const {
        Circuit t(circuit);
        for (std::size_t i = 0; i < N; ++i)
            t.spectrum[i] = spectrum[i] + other.spectrum[i];
        return t;
    }

    constexpr Circuit& operator *= (const Circuit& other) {
        return *this = *this * other;
    }

    constexpr Circuit operator * (const Circuit& other) {
        Circuit t(circuit);
        convolution.call(t.spectrum, this->spectrum, other.spectrum);
        return t;
    }
};

template<std::size_t Degree>
struct Assigner {
    using Convolution = Params::Convolution::template Assigner<Degree>;
    using IsomorphismAssigner = Isomorphism::template Assigner<Degree>;

    PolynomialRingNTT polynomial;
    std::vector<Z>* assigment;

    constexpr Assigner(const IsomorphismAssigner& e)
        : polynomial(e.polynomial), assigment(e.assigment) {}
    constexpr Assigner(const PolynomialRingNTT& polynomial, std::vector<Z>* assigment)
        : polynomial(polynomial), assigment(assigment) {}

    constexpr Z& operator [] (std::size_t i) {
        return polynomial[i];
    }

    constexpr const Z& operator [] (std::size_t i) const {
        return polynomial[i];
    }

    constexpr Assigner& operator += (const Assigner& other) {
        polynomial += other.polynomial;
        return *this;
    }

    constexpr Assigner operator + (const Assigner& other) const {
        return { polynomial + other.polynomial, assigment };
    }

    constexpr Assigner& operator *= (const Assigner& other) {
        return *this = *this * other;
    }

    constexpr Assigner operator * (const Assigner& other) const {
        Assigner t(PolynomialRingNTT::additive_identity(), assigment);
        Convolution(assigment).call(t.polynomial.spectrum, polynomial.spectrum, other.polynomial.spectrum);
        return t;
    }

    constexpr decltype(auto) begin() noexcept {
        return polynomial.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return polynomial.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return polynomial.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return polynomial.end();
    }
};

};

}

#endif
