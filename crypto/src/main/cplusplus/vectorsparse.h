/*
 * Copyright (c) 2025 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_VECTORSPARSE_H
#define BLACKNET_CRYPTO_VECTORSPARSE_H

#include <ostream>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename E>class MatrixDense;
template<typename E>class VectorDense;

template<typename E>
struct VectorSparse {
    using ElementType = E;

    std::size_t dSize;
    std::vector<std::size_t> eIndex;
    std::vector<E> sElements;

    constexpr VectorSparse(std::size_t size) : dSize(size) {}
    constexpr VectorSparse(const VectorDense<E>& dense) : dSize(dense.size()) {
        for (std::size_t i = 0; i < dSize; ++i) {
            if (dense[i] != E(0)) {
                eIndex.push_back(i);
                sElements.push_back(dense[i]);
            }
        }
    }
    constexpr VectorSparse(
        std::size_t dSize,
        std::vector<std::size_t>&& eIndex,
        std::vector<E>&& sElements
    ) : dSize(dSize), eIndex(std::move(eIndex)), sElements(std::move(sElements)) {}
    constexpr VectorSparse(const VectorSparse&) = default;
    constexpr VectorSparse(VectorSparse&&) noexcept = default;
    constexpr ~VectorSparse() noexcept = default;

    constexpr VectorSparse& operator = (const VectorSparse&) = default;
    constexpr VectorSparse& operator = (VectorSparse&&) noexcept = default;

    constexpr bool operator == (const VectorSparse&) const = default;

    constexpr std::size_t size() const {
        return dSize;
    }

    friend constexpr VectorDense<E> operator * (const MatrixDense<E>& lps, const VectorSparse<E>& rps) {
        VectorDense<E> r(lps.rows, E::additive_identity());
        std::size_t rps_nnz = rps.eIndex.size();
        for (std::size_t i = 0; i < lps.rows; ++i) {
            for (std::size_t j = 0; j < rps_nnz; ++j) {
                std::size_t column = rps.eIndex[j];
                r[i] += lps[i, column] * rps.sElements[j];
            }
        }
        return r;
    }

    template<typename NormType>
    requires(std::same_as<NormType, typename E::NumericType>)
    constexpr bool checkInfinityNorm(const NormType& bound) const {
        return std::ranges::all_of(sElements, [&bound](const E& e) {
            return e.checkInfinityNorm(bound);
        });
    }

    constexpr double euclideanNorm() const {
        double t = 0;
        for (std::size_t i = 0; i < sElements.size(); ++i) {
            double e = sElements[i].euclideanNorm();
            t += e * e;
        }
        return std::sqrt(t);
    }

    constexpr VectorDense<E> dense() const {
        VectorDense<E> r(dSize, E(0));
        for (std::size_t i = 0; i < sElements.size(); ++i) {
            r[eIndex[i]] = sElements[i];
        }
        return r;
    }

    friend std::ostream& operator << (std::ostream& out, const VectorSparse& val)
    {
        fmt::print(out, "({}, {})", val.eIndex, val.sElements);
        return out;
    }
};

}

template<typename E>
struct fmt::formatter<blacknet::crypto::VectorSparse<E>> : ostream_formatter {};

#endif
