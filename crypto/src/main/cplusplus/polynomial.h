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

#ifndef BLACKNET_CRYPTO_POLYNOMIAL_H
#define BLACKNET_CRYPTO_POLYNOMIAL_H

#include <ostream>
#include <type_traits>
#include <vector>

#include "util.h"

template<typename R, template<typename> typename P>
class Polynomial {
    std::vector<P<R>> polynomials;
public:
    constexpr Polynomial(std::size_t capacity) {
        polynomials.reserve(capacity);
    }
    constexpr Polynomial(std::vector<P<R>>&& polynomials) : polynomials(std::move(polynomials)) {}
    constexpr Polynomial(const Polynomial& other) = delete;
    constexpr Polynomial(Polynomial&& other) noexcept
        : polynomials(std::move(other.polynomials)) {}

    constexpr Polynomial& operator = (Polynomial&& other) noexcept {
        polynomials = std::move(other.polynomials);
        return *this;
    }

    template<typename Fuse1, typename Fuse0 = Fuse1>
    constexpr void apply(R& r, const std::vector<R>& point) const {
        if constexpr (std::is_same_v<Fuse1, Fuse0>) {
            for (std::size_t i = 0; i < polynomials.size(); ++i)
                Fuse1::call(r, polynomials[i](point));
        } else {
            Fuse0::call(r, polynomials[0](point));
            for (std::size_t i = 1; i < polynomials.size(); ++i)
                Fuse1::call(r, polynomials[i](point));
        }
    }

    constexpr Polynomial& operator () (P<R>&& other) {
        polynomials.emplace_back(std::move(other));
        return *this;
    }

    template<R e, typename Fuse1, typename Fuse0 = Fuse1>
    constexpr void bind(std::vector<R>& hypercube) const {
        if constexpr (std::is_same_v<Fuse1, Fuse0>) {
            for (std::size_t i = 0; i < polynomials.size(); ++i)
                polynomials[i].template bind<e, Fuse1>(hypercube);
        } else {
            polynomials[0].template bind<e, Fuse0>(hypercube);
            for (std::size_t i = 1; i < polynomials.size(); ++i)
                polynomials[i].template bind<e, Fuse1>(hypercube);
        }
    }

    constexpr void bind(const R& e) {
        for (auto& i : polynomials)
            i.bind(e);
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
        return out << val.polynomials;
    }
};

#endif
