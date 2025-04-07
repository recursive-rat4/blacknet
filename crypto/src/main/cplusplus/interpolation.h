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

template<
    typename Z,
    typename S
>
struct Interpolation {
    constexpr static UnivariatePolynomial<S> balanced(const S& z0, const S& p1) {
        return UnivariatePolynomial<S>{z0, p1 - z0};
    }
    constexpr static UnivariatePolynomial<S> balanced(const S& n1, const S& z0, const S& p1) {
        // Undefined behaviour is prohibited in consteval
        static const Z inv2 = Z(2).invert().value();

        S a(p1 * inv2 + n1 * inv2 - z0);
        S b(p1 * inv2 - n1 * inv2);
        S c(z0);
        return UnivariatePolynomial<S>{c, b, a};
    }
    constexpr static UnivariatePolynomial<S> balanced(const S& n2, const S& n1, const S& z0, const S& p1, const S& p2) {
        // Undefined behaviour is prohibited in consteval
        static const Z mul_2_div_3 = Z(2) * Z(3).invert().value();
        static const Z inv4 = Z(4).invert().value();
        static const Z inv6 = Z(6).invert().value();
        static const Z inv12 = Z(12).invert().value();
        static const Z inv24 = Z(24).invert().value();

        S a(z0 * inv4 - p1 * inv6 + p2 * inv24 - n1 * inv6 + n2 * inv24);
        S b(- p1 * inv6 + p2 * inv12 + n1 * inv6 - n2 * inv12);
        S c(- z0 * Z(5) * inv4 + p1 * mul_2_div_3 - p2 * inv24 + n1 * mul_2_div_3 - n2 * inv24);
        S d(p1 * mul_2_div_3 - p2 * inv12 - n1 * mul_2_div_3 + n2 * inv12);
        S e(z0);
        return UnivariatePolynomial<S>{e, d, c, b, a};
    }
    constexpr static UnivariatePolynomial<S> balanced(const S& n2, const S& n1, const S& z0, const S& p1, const S& p2, const S& p3) {
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

        S a(- z0 * inv12 + p1 * inv12 - p2 * inv24 + p3 * inv120 + n1 * inv24 - n2 * inv120);
        S b(z0 * inv4 - p1 * inv6 + p2 * inv24 - n1 * inv6 + n2 * inv24);
        S c(z0 * mul_5_div_12 - p1 * mul_7_div_12 + p2 * mul_7_div_24 - p3 * inv24 - n1 * inv24 - n2 * inv24);
        S d(- z0 * mul_5_div_4 + p1 * mul_2_div_3 - p2 * inv24 + n1 * mul_2_div_3 - n2 * inv24);
        S e(- z0 * inv3 + p1 - p2 * inv4 + p3 * inv30 - n1 * inv2 + n2 * inv20);
        S f(z0);
        return UnivariatePolynomial<S>{f, e, d, c, b, a};
    }
};

}

#endif
