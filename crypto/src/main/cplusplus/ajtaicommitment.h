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

#ifndef BLACKNET_CRYPTO_AJTAICOMMITMENT_H
#define BLACKNET_CRYPTO_AJTAICOMMITMENT_H

#include <type_traits>

#include "matrix.h"
#include "vector.h"

namespace blacknet::crypto {

/*
 * Generating Hard Instances of Lattice Problems (Extended abstract)
 * Miklós Ajtai
 * 1996
 * https://www.cs.sjsu.edu/faculty/pollett/masters/Semesters/Spring21/michaela/files/Ajtai96.pdf
 */

enum class NormP {
    Euclidean = 2,
    Infinity = -1,
};

template<
    typename R,
    NormP norm_p
>
class AjtaiCommitment {
    using NumericType = std::conditional<
        norm_p == NormP::Infinity,
        typename R::NumericType,
        double
    >::type;
    Matrix<R> a;
    NumericType bound;
public:
    constexpr AjtaiCommitment(const Matrix<R>& a, const NumericType& bound)
        : a(a), bound(bound) {}
    constexpr AjtaiCommitment(Matrix<R>&& a, NumericType&& bound)
        : a(std::move(a)), bound(std::move(bound)) {}

    template<typename Sponge>
    constexpr static Matrix<R> setup(Sponge& sponge, std::size_t rows, std::size_t columns) {
        Matrix<R> t(rows, columns);
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                t[i, j] = R::squeeze(sponge);
        return t;
    }

    constexpr Vector<R> commit(const Vector<R>& m) const {
        return a * m;
    }

    constexpr bool open(const Vector<R>& c, const Vector<R>& m) const {
        if constexpr (norm_p == NormP::Infinity) {
            return m.checkInfinityNorm(bound) && c == commit(m);
        } else if constexpr (norm_p == NormP::Euclidean) {
            return m.euclideanNorm() < bound && c == commit(m);
        } else {
            static_assert(false, "Not implemented");
        }
    }
};

}

#endif
