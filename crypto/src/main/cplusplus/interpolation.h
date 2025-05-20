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

#ifndef BLACKNET_CRYPTO_INTERPOLATION_H
#define BLACKNET_CRYPTO_INTERPOLATION_H

#include "univariatepolynomial.h"

namespace blacknet::crypto {

template<typename R>
struct Interpolation {
    using Z = R::BaseRing;

    constexpr static UnivariatePolynomial<R> balanced(const R& z0, const R& p1) {
        return UnivariatePolynomial<R>{z0, p1 - z0};
    }
    constexpr static UnivariatePolynomial<R> balanced(const R& n1, const R& z0, const R& p1) {
        // Undefined behaviour is prohibited in consteval
        static const Z inv2 = Z(2).invert().value();

        R a(p1 * inv2 + n1 * inv2 - z0);
        R b(p1 * inv2 - n1 * inv2);
        R c(z0);
        return UnivariatePolynomial<R>{c, b, a};
    }
    constexpr static UnivariatePolynomial<R> balanced(const R& n1, const R& z0, const R& p1, const R& p2) {
        // Undefined behaviour is prohibited in consteval
        static const Z inv2 = Z(2).invert().value();
        static const Z inv3 = Z(3).invert().value();
        static const Z inv6 = Z(6).invert().value();

        R a(z0 * inv2 - p1 * inv2 + p2 * inv6 - n1 * inv6);
        R b(- z0 + p1 * inv2 + n1 * inv2);
        R c(- z0 * inv2 + p1 - p2 * inv6 - n1 * inv3);
        R d(z0);
        return UnivariatePolynomial<R>{d, c, b, a};
    }
    constexpr static UnivariatePolynomial<R> balanced(const R& n2, const R& n1, const R& z0, const R& p1, const R& p2) {
        // Undefined behaviour is prohibited in consteval
        static const Z mul_2_div_3 = Z(2) * Z(3).invert().value();
        static const Z inv4 = Z(4).invert().value();
        static const Z inv6 = Z(6).invert().value();
        static const Z inv12 = Z(12).invert().value();
        static const Z inv24 = Z(24).invert().value();

        R a(z0 * inv4 - p1 * inv6 + p2 * inv24 - n1 * inv6 + n2 * inv24);
        R b(- p1 * inv6 + p2 * inv12 + n1 * inv6 - n2 * inv12);
        R c(- z0 * Z(5) * inv4 + p1 * mul_2_div_3 - p2 * inv24 + n1 * mul_2_div_3 - n2 * inv24);
        R d(p1 * mul_2_div_3 - p2 * inv12 - n1 * mul_2_div_3 + n2 * inv12);
        R e(z0);
        return UnivariatePolynomial<R>{e, d, c, b, a};
    }
    constexpr static UnivariatePolynomial<R> balanced(const R& n2, const R& n1, const R& z0, const R& p1, const R& p2, const R& p3) {
        // Undefined behaviour is prohibited in consteval
        static const Z mul_2_div_3 = Z(2) * Z(3).invert().value();
        static const Z mul_5_div_4 = Z(5) * Z(4).invert().value();
        static const Z mul_5_div_12 = Z(5) * Z(12).invert().value();
        static const Z mul_7_div_12 = Z(7) * Z(12).invert().value();
        static const Z mul_7_div_24 = Z(7) * Z(24).invert().value();
        static const Z inv2 = Z(2).invert().value();
        static const Z inv3 = Z(3).invert().value();
        static const Z inv4 = Z(4).invert().value();
        static const Z inv6 = Z(6).invert().value();
        static const Z inv12 = Z(12).invert().value();
        static const Z inv20 = Z(20).invert().value();
        static const Z inv24 = Z(24).invert().value();
        static const Z inv30 = Z(30).invert().value();
        static const Z inv120 = Z(120).invert().value();

        R a(- z0 * inv12 + p1 * inv12 - p2 * inv24 + p3 * inv120 + n1 * inv24 - n2 * inv120);
        R b(z0 * inv4 - p1 * inv6 + p2 * inv24 - n1 * inv6 + n2 * inv24);
        R c(z0 * mul_5_div_12 - p1 * mul_7_div_12 + p2 * mul_7_div_24 - p3 * inv24 - n1 * inv24 - n2 * inv24);
        R d(- z0 * mul_5_div_4 + p1 * mul_2_div_3 - p2 * inv24 + n1 * mul_2_div_3 - n2 * inv24);
        R e(- z0 * inv3 + p1 - p2 * inv4 + p3 * inv30 - n1 * inv2 + n2 * inv20);
        R f(z0);
        return UnivariatePolynomial<R>{f, e, d, c, b, a};
    }
};

}

#endif
