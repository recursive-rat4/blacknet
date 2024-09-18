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

#ifndef BLACKNET_CRYPTO_EQEXTENSION_H
#define BLACKNET_CRYPTO_EQEXTENSION_H

#include <vector>
#include <boost/io/ostream_joiner.hpp>

template<typename E>
class EqExtension {
    std::vector<E> coefficients;
public:
    constexpr EqExtension(const std::vector<E>& coefficients) : coefficients(coefficients) {}

    constexpr std::vector<E> operator () () const {
        std::vector<E> r(1 << coefficients.size(), E::LEFT_ADDITIVE_IDENTITY());
        r[0] = E::LEFT_MULTIPLICATIVE_IDENTITY();
        for (std::size_t i = coefficients.size(), j = 1; i --> 0; j <<= 1) {
            for (std::size_t k = 0, l = j; k < j && l < j << 1; ++k, ++l) {
                r[l] = r[k] * coefficients[i];
                r[k] -= r[l];
            }
        }
        return r;
    }

    constexpr E operator () (const std::vector<E>& point) const {
        E pi(E::LEFT_MULTIPLICATIVE_IDENTITY());
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            pi *= coefficients[i] * point[i] + (E(1) - coefficients[i]) * (E(1) - point[i]);
        return pi;
    }

    consteval std::size_t degree() const {
        return 1;
    }

    constexpr std::size_t variables() const {
        return coefficients.size();
    }

    friend std::ostream& operator << (std::ostream& out, const EqExtension& val)
    {
        out << '[';
        std::copy(val.coefficients.begin(), val.coefficients.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }
};

#endif
