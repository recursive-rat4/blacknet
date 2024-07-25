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

#ifndef BLACKNET_CRYPTO_AJTAICOMMITMENT_H
#define BLACKNET_CRYPTO_AJTAICOMMITMENT_H

#include "matrix.h"
#include "vector.h"

/*
 * Generating Hard Instances of Lattice Problems (Extended abstract)
 * Miklós Ajtai
 * 1996
 * https://www.cs.sjsu.edu/faculty/pollett/masters/Semesters/Spring21/michaela/files/Ajtai96.pdf
 */

template<typename R>
class AjtaiCommitment {
    Matrix<R> a;
public:
    constexpr AjtaiCommitment(const Matrix<R>& a) : a(a) {}
    constexpr AjtaiCommitment(Matrix<R>&& a) : a(std::move(a)) {}

    constexpr Vector<R> commit(const Vector<R>& m) const {
        return a * m;
    }

    constexpr bool open(const Vector<R>& c, const Vector<R>& m) const {
        return c == commit(m);
    }
};

#endif
