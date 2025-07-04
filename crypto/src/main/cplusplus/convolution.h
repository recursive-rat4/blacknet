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

#ifndef BLACKNET_CRYPTO_CONVOLUTION_H
#define BLACKNET_CRYPTO_CONVOLUTION_H

#include <array>

#include "matrixdense.h"

namespace blacknet::crypto {

namespace convolution {

template<typename Z, std::size_t N>
struct Negacyclic {
    constexpr static void call(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        for (std::size_t k = 0; k < N; ++k) {
            for (std::size_t i = 0; i <= k; ++i) {
                r[k] += a[i] * b[k - i];
            }
            for (std::size_t i = k + 1; i < N; ++i) {
                r[k] -= a[i] * b[k + N - i];
            }
        }
    }
};

template<typename Z, std::size_t N, std::size_t M>
struct Long {
    constexpr static void call(std::array<Z, N+M-1>& r, const std::array<Z, N>& a, const std::array<Z, M>& b) {
        for (std::size_t i = 0; i < N; ++i) {
            for (std::size_t j = 0; j < M; ++j) {
                r[i + j] += a[i] * b[j];
            }
        }
    }
};

template<typename Z, std::size_t N, std::array<Z, N+1> M>
struct Quotient {
private:
    template<Z c>
    constexpr static void fuse(Z& r, const Z& a, const Z& b) {
        if constexpr (c == Z(0))
            r = a;
        else if constexpr (c == Z(1))
            r = a - b;
        else if constexpr (c == Z(2))
            r = a - b.douple();
        else
            r = a - b * c;
    }

    template<Z b>
    constexpr static void fuse(Z& r, const Z& a) {
        if constexpr (b == Z(0))
            return;
        else if constexpr (b == Z(1))
            r -= a;
        else if constexpr (b == Z(2))
            r -= a.douple();
        else
            r -= a * b;
    }
public:
    constexpr static void call(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        static_assert(M.back() == Z(1));
        std::array<Z, N + N - 1> t;
        t.fill(Z::additive_identity());
        Long<Z, N, N>::call(t, a, b);
        if constexpr (N == 2) {
            fuse<M[0]>(r[0], t[0], t[2]);
            fuse<M[1]>(r[1], t[1], t[2]);
        } else if constexpr (N == 3) {
            fuse<M[0]>(r[1], t[1], t[4]);
            fuse<M[1]>(r[2], t[2], t[4]);
            fuse<M[2]>(t[3], t[4]);

            fuse<M[0]>(r[0], t[0], t[3]);
            fuse<M[1]>(r[1], t[3]);
            fuse<M[2]>(r[2], t[3]);
        } else if constexpr (N == 4) {
            fuse<M[0]>(r[2], t[2], t[6]);
            fuse<M[1]>(r[3], t[3], t[6]);
            fuse<M[2]>(t[4], t[6]);
            fuse<M[3]>(t[5], t[6]);

            fuse<M[0]>(r[1], t[1], t[5]);
            fuse<M[1]>(r[2], t[5]);
            fuse<M[2]>(r[3], t[5]);
            fuse<M[3]>(t[4], t[5]);

            fuse<M[0]>(r[0], t[0], t[4]);
            fuse<M[1]>(r[1], t[4]);
            fuse<M[2]>(r[2], t[4]);
            fuse<M[3]>(r[3], t[4]);
        } else {
            static_assert(false, "Not implemented");
        }
    }
};

template<typename Z, std::size_t N>
struct Binomial {
    constexpr static void call(Z* r, const Z* a, const Z* b, Z zeta) {
        if constexpr (N == 4) {
            r[0] = a[0] * b[0] + zeta * (a[1] * b[3] + a[2] * b[2] + a[3] * b[1]);
            r[1] = a[0] * b[1] + a[1] * b[0] + zeta * (a[2] * b[3] + a[3] * b[2]);
            r[2] = a[0] * b[2] + a[1] * b[1] + a[2] * b[0] + zeta * (a[3] * b[3]);
            r[3] = a[0] * b[3] + a[1] * b[2] + a[2] * b[1] + a[3] * b[0];
        } else if constexpr (N == 3) {
            r[0] = a[0] * b[0] + zeta * (a[1] * b[2] + a[2] * b[1]);
            r[1] = a[0] * b[1] + a[1] * b[0] + zeta * (a[2] * b[2]);
            r[2] = a[0] * b[2] + a[1] * b[1] + a[2] * b[0];
        } else if constexpr (N == 2) {
            r[0] = a[0] * b[0] + zeta * (a[1] * b[1]);
            r[1] = a[0] * b[1] + a[1] * b[0];
        } else {
            static_assert(false, "Not implemented");
        }
    }

template<typename Builder>
requires(std::same_as<Z, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using MatrixDense = MatrixDense<Z>::template Circuit<Builder>;

    Builder& circuit;

    constexpr Circuit(Builder& circuit) : circuit(circuit) {}

    constexpr void call(
        LinearCombination* r,
        const LinearCombination* a,
        const LinearCombination* b,
        Z zeta
    ) {
        auto scope = circuit.scope("Convolution::binomial");
        //TODO Karatsuba method
        MatrixDense ab(circuit, N, N);
        for (std::size_t i = 0; i < N; ++i) {
            for (std::size_t j = 0; j < N; ++j) {
                auto t = circuit.auxiliary();
                circuit(t == a[i] * b[j]);
                ab[i, j] = t;
            }
        }
        if constexpr (N == 4) {
            r[0] = ab[0, 0] + zeta * (ab[1, 3] + ab[2, 2] + ab[3, 1]);
            r[1] = ab[0, 1] + ab[1, 0] + zeta * (ab[2, 3] + ab[3, 2]);
            r[2] = ab[0, 2] + ab[1, 1] + ab[2, 0] + zeta * (ab[3, 3]);
            r[3] = ab[0, 3] + ab[1, 2] + ab[2, 1] + ab[3, 0];
        } else if constexpr (N == 3) {
            r[0] = ab[0, 0] + zeta * (ab[1, 2] + ab[2, 1]);
            r[1] = ab[0, 1] + ab[1, 0] + zeta * (ab[2, 2]);
            r[2] = ab[0, 2] + ab[1, 1] + ab[2, 0];
        } else if constexpr (N == 2) {
            r[0] = ab[0, 0] + zeta * (ab[1, 1]);
            r[1] = ab[0, 1] + ab[1, 0];
        } else {
            static_assert(false, "Not implemented");
        }
    }
};

template<std::size_t Degree>
struct Assigner {
    using MatrixDense = MatrixDense<Z>::template Assigner<Degree>;

    std::vector<Z>& assigment;

    constexpr void call(Z* r, const Z* a, const Z* b, Z zeta) {
        MatrixDense ab(N, N, assigment);
        for (std::size_t i = 0; i < N; ++i) {
            for (std::size_t j = 0; j < N; ++j) {
                ab[i, j] = assigment.emplace_back(
                    a[i] * b[j]
                );
            }
        }
        if constexpr (N == 4) {
            r[0] = ab[0, 0] + zeta * (ab[1, 3] + ab[2, 2] + ab[3, 1]);
            r[1] = ab[0, 1] + ab[1, 0] + zeta * (ab[2, 3] + ab[3, 2]);
            r[2] = ab[0, 2] + ab[1, 1] + ab[2, 0] + zeta * (ab[3, 3]);
            r[3] = ab[0, 3] + ab[1, 2] + ab[2, 1] + ab[3, 0];
        } else if constexpr (N == 3) {
            r[0] = ab[0, 0] + zeta * (ab[1, 2] + ab[2, 1]);
            r[1] = ab[0, 1] + ab[1, 0] + zeta * (ab[2, 2]);
            r[2] = ab[0, 2] + ab[1, 1] + ab[2, 0];
        } else if constexpr (N == 2) {
            r[0] = ab[0, 0] + zeta * (ab[1, 1]);
            r[1] = ab[0, 1] + ab[1, 0];
        } else {
            static_assert(false, "Not implemented");
        }
    }
};

};

}

}

#endif
