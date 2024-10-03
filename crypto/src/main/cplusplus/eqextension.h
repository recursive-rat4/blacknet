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
    E z;
public:
    constexpr EqExtension(const std::vector<E>& coefficients)
        : coefficients(coefficients), z(E::LEFT_MULTIPLICATIVE_IDENTITY()) {}
    constexpr EqExtension(const std::vector<E>& coefficients, const E& z)
        : coefficients(coefficients), z(z) {}
    constexpr EqExtension(std::vector<E>&& coefficients)
        : coefficients(std::move(coefficients)), z(E::LEFT_MULTIPLICATIVE_IDENTITY()) {}
    constexpr EqExtension(std::vector<E>&& coefficients, E&& z)
        : coefficients(std::move(coefficients)), z(std::move(z)) {}

    constexpr std::vector<E> operator () () const {
        std::vector<E> r(1 << coefficients.size(), E::LEFT_ADDITIVE_IDENTITY());
        r[0] = z;
        for (std::size_t i = coefficients.size(), j = 1; i --> 0; j <<= 1) {
            for (std::size_t k = 0, l = j; k < j && l < j << 1; ++k, ++l) {
                r[l] = r[k] * coefficients[i];
                r[k] -= r[l];
            }
        }
        return r;
    }

    constexpr E operator () (const std::vector<E>& point) const {
        E pi(z);
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            pi *= (coefficients[i] * point[i]).douple() - coefficients[i] - point[i] + E(1);
        return pi;
    }

    constexpr EqExtension& operator *= (const E& other) {
        z *= other;
        return *this;
    }

    constexpr EqExtension operator * (const E& other) const {
        return EqExtension(coefficients, z * other);
    }

    template<E e, typename Fuse>
    constexpr void bind(std::vector<E>& r) const {
        E ze;
        if constexpr (e == E(0)) {
            ze = z * (E(1) - coefficients[0]);
        } else if constexpr (e == E(1)) {
            ze = z * coefficients[0];
        } else if constexpr (e == E(2)) {
            ze = z * (coefficients[0].douple() + coefficients[0] - E(1));
        } else if constexpr (e == E(3)) {
            ze = z * (coefficients[0].douple().douple() + coefficients[0] - E(2));
        } else if constexpr (e == E(4)) {
            ze = z * (coefficients[0].douple().douple().douple() - coefficients[0] - E(3));
        } else {
            static_assert(false);
        }
        std::vector<E> re(r.size(), E::LEFT_ADDITIVE_IDENTITY());
        re[0] = ze;
        for (std::size_t i = coefficients.size() - 1, j = 1; i --> 0; j <<= 1) {
            for (std::size_t k = 0, l = j; k < j && l < j << 1; ++k, ++l) {
                re[l] = re[k] * coefficients[i + 1];
                re[k] -= re[l];
            }
        }
        Fuse::call(r, std::move(re));
    }

    constexpr void bind(const E& e) {
        z *= (coefficients[0] * e).douple() - coefficients[0] - e + E(1);
        coefficients.erase(coefficients.begin());
    }

    consteval std::size_t degree() const {
        return 1;
    }

    constexpr std::size_t variables() const {
        return coefficients.size();
    }

    template<typename S>
    constexpr EqExtension<S> homomorph() const {
        std::vector<S> t;
        t.reserve(coefficients.size());
        for (const auto& i : coefficients)
            t.emplace_back(S(i));
        return EqExtension<S>(std::move(t), S(z));
    }

    friend std::ostream& operator << (std::ostream& out, const EqExtension& val)
    {
        out << "([";
        std::copy(val.coefficients.begin(), val.coefficients.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << "], " << val.z << ')';
    }
};

#endif
