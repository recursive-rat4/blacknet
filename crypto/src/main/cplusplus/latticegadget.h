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

#ifndef BLACKNET_CRYPTO_LATTICEGADGET_H
#define BLACKNET_CRYPTO_LATTICEGADGET_H

#include <concepts>
#include <stdexcept>

#include "logicgate.h"
#include "matrixdense.h"
#include "vector.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2018/946

template<typename R>
struct LatticeGadget {
    using NumericType = R::NumericType;
private:
    template<typename T = R>
    requires(T::is_integer_ring)
    constexpr static void decompose(
        NumericType radix, std::size_t digits,
        T* pieces, const T& f
    ) {
        auto representative = f.canonical();
        for (std::size_t j = 0; j < digits; ++j) {
            pieces[j] = representative % radix;
            representative /= radix;
        }
    }

    template<typename T = R>
    constexpr static void decompose(
        NumericType radix, std::size_t digits,
        T* pieces, const T& f
    ) {
        for (std::size_t i = 0; i < R::dimension(); ++i) {
            auto representative = f.coefficients[i].canonical();
            for (std::size_t j = 0; j < digits; ++j) {
                pieces[j].coefficients[i] = representative % radix;
                representative /= radix;
            }
        }
    }
public:
    constexpr static Vector<R> decompose(
        NumericType radix, std::size_t digits,
        const R& f
    ) {
        Vector<R> pieces(digits);
        decompose(radix, digits, pieces.elements.data(), f);
        return pieces;
    }

    constexpr static Vector<R> decompose(
        NumericType radix, std::size_t digits,
        const Vector<R>& f
    ) {
        Vector<R> pieces(f.size() * digits);
        for (std::size_t i = 0; i < f.size(); ++i)
            decompose(radix, digits, pieces.elements.data() + i * digits, f[i]);
        return pieces;
    }

    constexpr static Vector<R> vector(
        NumericType radix, std::size_t digits,
        const R& r
    ) {
        Vector<R> p(digits);
        p[0] = r;
        typename R::BaseRing t{radix};
        for (std::size_t i = 1; i < digits; ++i) {
            p[i] = r * t;
            t *= radix;
        }
        return p;
    }

    constexpr static MatrixDense<R> matrix(
        NumericType radix,
        std::size_t m, std::size_t n
    ) {
        Vector<R> pm(n);
        pm[0] = R::multiplicative_identity();
        for (std::size_t i = 1; i < n; ++i)
            pm[i] = pm[i - 1] * radix;
        return Vector<R>::identity(m).tensor(pm);
    }

template<typename Builder>
requires(std::same_as<R, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using LogicGate = LogicGate<R>::template Circuit<Builder>;
    using Vector = Vector<R>::template Circuit<Builder>;

    Builder& circuit;

    constexpr Circuit(Builder& circuit) : circuit(circuit) {}

    constexpr Vector decompose(
        NumericType radix, std::size_t digits,
        const LinearCombination& f
    ) {
        if (radix != 2) throw std::runtime_error("Not implemented");
        auto scope = circuit.scope("LatticeGadget::decompose");
        R p = R::multiplicative_identity();
        LinearCombination composed;
        Vector pieces(circuit, digits);
        for (auto& piece : pieces) {
            LinearCombination digit = circuit.auxiliary();
            piece = digit;
            composed += digit * p;
            p *= radix;
        }
        LogicGate(circuit).RangeCheck(pieces);
        circuit(f == composed);
        return pieces;
    }
};

template<std::size_t Degree>
struct Assigner {
    std::vector<R>& assigment;

    constexpr Assigner(std::vector<R>& assigment)
        : assigment(assigment) {}

    constexpr Vector<R> decompose(
        NumericType radix, std::size_t digits,
        const R& f
    ) {
        Vector<R> pieces(digits);
        decompose(radix, digits, pieces.elements.data(), f);
        return pieces;
    }
private:
    template<typename T = R>
    requires(T::is_integer_ring)
    constexpr void decompose(
        NumericType radix, std::size_t digits,
        T* pieces, const T& f
    ) {
        auto representative = f.canonical();
        for (std::size_t j = 0; j < digits; ++j) {
            pieces[j] = assigment.emplace_back(
                representative % radix
            );
            representative /= radix;
        }
    }
};

};

}

#endif
