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

#ifndef BLACKNET_CRYPTO_UNIVARIATEPOLYNOMIAL_H
#define BLACKNET_CRYPTO_UNIVARIATEPOLYNOMIAL_H

#include <initializer_list>
#include <iostream>
#include <vector>
#include <boost/io/ostream_joiner.hpp>

template<typename E>
class UnivariatePolynomial {
public:
    std::vector<E> coefficients;

    constexpr UnivariatePolynomial(std::size_t size) : coefficients(size) {}
    constexpr UnivariatePolynomial(std::initializer_list<E> init) : coefficients(init) {}
    constexpr UnivariatePolynomial(UnivariatePolynomial&& other) noexcept
        : coefficients(std::move(other.coefficients)) {}

    constexpr bool operator == (const UnivariatePolynomial&) const = default;

    constexpr E operator () (const E& point) const {
        E sigma = coefficients[0];
        E pi = point;
        for (std::size_t i = 1; i < coefficients.size(); ++i) {
            sigma += pi * coefficients[i];
            pi *= point;
        }
        return sigma;
    }

    constexpr std::size_t degree() const {
        return coefficients.size() - 1;
    }

    consteval std::size_t variables() const {
        return 1;
    }

    constexpr static UnivariatePolynomial interpolate(const E& p0, const E& p1) {
        return UnivariatePolynomial{p0, p1 - p0};
    }

    friend std::ostream& operator << (std::ostream& out, const UnivariatePolynomial& val)
    {
        out << '[';
        std::copy(val.coefficients.begin(), val.coefficients.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }
};

#endif
