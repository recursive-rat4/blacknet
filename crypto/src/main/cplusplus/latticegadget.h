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

#include "matrix.h"
#include "vector.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2018/946

namespace lattice_gadget {
namespace {
    template<typename Z, std::size_t radix, std::size_t digits>
    requires(Z::is_integer_ring)
    constexpr void decompose(Z* pieces, const Z& f) {
        auto representative = f.canonical();
        for (std::size_t j = 0; j < digits; ++j) {
            pieces[j] = representative % radix;
            representative /= radix;
        }
    }

    template<typename R, std::size_t radix, std::size_t digits>
    constexpr void decompose(R* pieces, const R& f) {
        for (std::size_t i = 0; i < R::dimension(); ++i) {
            auto representative = f.coefficients[i].canonical();
            for (std::size_t j = 0; j < digits; ++j) {
                pieces[j].coefficients[i] = representative % radix;
                representative /= radix;
            }
        }
    }
}
    template<typename R, std::size_t radix, std::size_t digits>
    constexpr Vector<R> decompose(const R& f) {
        Vector<R> pieces(digits);
        decompose<R, radix, digits>(pieces.elements.data(), f);
        return pieces;
    }

    template<typename R, std::size_t radix, std::size_t digits>
    constexpr Vector<R> decompose(const Vector<R>& f) {
        Vector<R> pieces(f.size() * digits);
        for (std::size_t i = 0; i < f.size(); ++i)
            decompose<R, radix, digits>(pieces.elements.data() + i * digits, f[i]);
        return pieces;
    }

    template<typename R, std::size_t radix, std::size_t digits>
    constexpr Vector<R> vector(const R& r) {
        Vector<R> p(digits);
        p[0] = r;
        typename R::BaseRing t{radix};
        for (std::size_t i = 1; i < digits; ++i) {
            p[i] = r * t;
            t *= radix;
        }
        return p;
    }

    template<typename R, std::size_t radix>
    constexpr Matrix<R> matrix(std::size_t m, std::size_t n) {
        Vector<R> pm(n);
        pm[0] = R::LEFT_MULTIPLICATIVE_IDENTITY();
        for (std::size_t i = 1; i < n; ++i)
            pm[i] = pm[i - 1] * radix;
        return Vector<R>::identity(m).tensor(pm);
    }
};

}

#endif
