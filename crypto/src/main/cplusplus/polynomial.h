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

#ifndef BLACKNET_CRYPTO_POLYNOMIAL_H
#define BLACKNET_CRYPTO_POLYNOMIAL_H

#include <iostream>
#include <vector>
#include <boost/io/ostream_joiner.hpp>

template<typename R, template<typename> typename P>
class Polynomial {
    std::vector<P<R>> polynomials;
public:
    constexpr Polynomial(std::size_t capacity) {
        polynomials.reserve(capacity);
    }
    constexpr Polynomial(std::vector<P<R>>&& polynomials) : polynomials(std::move(polynomials)) {}
    constexpr Polynomial(Polynomial&& other) noexcept
        : polynomials(std::move(other.polynomials)) {}

    constexpr Polynomial& operator = (Polynomial&& other) {
        polynomials = std::move(other.polynomials);
        return *this;
    }

    constexpr void sigma(std::vector<R>& r) const {
        for (const auto& i : polynomials)
            i.sigma(r);
    }

    constexpr void sigma(R& r, const std::vector<R>& point) const {
        for (const auto& i : polynomials)
            r += i(point);
    }

    constexpr void pi(std::vector<R>& r) const {
        for (const auto& i : polynomials)
            i.pi(r);
    }

    constexpr void pi(R& r, const std::vector<R>& point) const {
        for (const auto& i : polynomials)
            r *= i(point);
    }

    constexpr Polynomial& operator () (P<R>&& other) {
        polynomials.emplace_back(std::move(other));
        return *this;
    }

    template<R e>
    constexpr Polynomial bind() const {
        std::vector<P<R>> t;
        t.reserve(polynomials.size());
        for (const auto& i : polynomials)
            t.emplace_back(i.template bind<e>());
        return Polynomial(std::move(t));
    }

    constexpr Polynomial bind(const R& e) const {
        std::vector<P<R>> t;
        t.reserve(polynomials.size());
        for (const auto& i : polynomials)
            t.emplace_back(i.bind(e));
        return Polynomial(std::move(t));
    }

    constexpr std::size_t variables() const {
        return polynomials[0].variables();
    }

    template<typename S>
    constexpr Polynomial<S, P> homomorph() const {
        std::vector<P<S>> t;
        t.reserve(polynomials.size());
        for (const auto& i : polynomials)
            t.emplace_back(i.template homomorph<S>());
        return Polynomial<S, P>(std::move(t));
    }

    friend std::ostream& operator << (std::ostream& out, const Polynomial& val)
    {
        out << '[';
        std::copy(val.polynomials.begin(), val.polynomials.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }
};

#endif
