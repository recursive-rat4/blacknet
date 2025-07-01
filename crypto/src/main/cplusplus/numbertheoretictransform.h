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

#ifndef BLACKNET_CRYPTO_NUMBERTHEORETICTRANSFORM_H
#define BLACKNET_CRYPTO_NUMBERTHEORETICTRANSFORM_H

#include <array>

#include "convolution.h"

namespace blacknet::crypto {

// https://arxiv.org/abs/2211.13546

template<typename Z, std::size_t N>
struct NTT {
    constexpr static const std::size_t inertia = N / Z::twiddles();

    constexpr static void cooley_tukey(std::array<Z, N>& a) {
        std::size_t i = 0, j = 0;
        for (std::size_t k = N / 2; k >= inertia; k >>= 1) {
            for (std::size_t l = 0; l < N; l = i + k) {
                const Z zeta(Z::twiddle(++j));
                for (i = l; i < l + k; ++i) {
                    Z t(a[i + k] * zeta);
                    a[i + k] = a[i] - t;
                    a[i] += t;
                }
            }
        }
    }

    constexpr static void gentleman_sande(std::array<Z, N>& a) {
        std::size_t i = 0, j = Z::twiddles();
        for (std::size_t k = inertia; k <= N / 2; k <<= 1) {
            for (std::size_t l = 0; l < N; l = i + k) {
                const Z zeta(-Z::twiddle(--j));
                for (i = l; i < l + k; ++i) {
                    Z t(a[i]);
                    a[i] += a[i + k];
                    a[i + k] = t - a[i + k];
                    a[i + k] *= zeta;
                }
            }
        }

        for (i = 0; i < N; ++i) {
            a[i] *= Z::inverse_twiddles();
        }
    }

    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        if constexpr (inertia == 1) {
            for (std::size_t i = 0; i < N; ++i) {
                r[i] = a[i] * b[i];
            }
        } else if constexpr (inertia == 4) {
            constexpr std::size_t k = inertia * 2;
            constexpr std::size_t l = N / k;
            for (std::size_t i = 0; i < l; ++i) {
                Convolution<Z>::template binomial<inertia>(
                    r.data() + i * k,
                    a.data() + i * k,
                    b.data() + i * k,
                    Z::twiddle(l + i)
                );
                Convolution<Z>::template binomial<inertia>(
                    r.data() + i * k + inertia,
                    a.data() + i * k + inertia,
                    b.data() + i * k + inertia,
                    -Z::twiddle(l + i)
                );
            }
        } else {
            static_assert(false, "Not implemented");
        }
    }

template<typename Builder>
requires(std::same_as<Z, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using Convolution = Convolution<Z>::template Circuit<Builder>;

    Builder& circuit;
    Convolution convolution;

    constexpr Circuit(Builder& circuit)
        : circuit(circuit), convolution(circuit) {}

    constexpr void cooley_tukey(std::array<LinearCombination, N>& a) const {
        std::size_t i = 0, j = 0;
        for (std::size_t k = N / 2; k >= inertia; k >>= 1) {
            for (std::size_t l = 0; l < N; l = i + k) {
                const Z zeta(Z::twiddle(++j));
                for (i = l; i < l + k; ++i) {
                    LinearCombination t(a[i + k] * zeta);
                    a[i + k] = a[i] - t;
                    a[i] += t;
                }
            }
        }
    }

    constexpr void gentleman_sande(std::array<LinearCombination, N>& a) const {
        std::size_t i = 0, j = Z::twiddles();
        for (std::size_t k = inertia; k <= N / 2; k <<= 1) {
            for (std::size_t l = 0; l < N; l = i + k) {
                const Z zeta(-Z::twiddle(--j));
                for (i = l; i < l + k; ++i) {
                    LinearCombination t(a[i]);
                    a[i] += a[i + k];
                    a[i + k] = t - a[i + k];
                    a[i + k] *= zeta;
                }
            }
        }

        for (i = 0; i < N; ++i) {
            a[i] *= Z::inverse_twiddles();
        }
    }

    constexpr void convolute(
        std::array<LinearCombination, N>& r,
        const std::array<LinearCombination, N>& a,
        const std::array<LinearCombination, N>& b
    ) {
        if constexpr (inertia == 1) {
            for (std::size_t i = 0; i < N; ++i) {
                auto t = circuit.auxiliary();
                circuit(t == a[i] * b[i]);
                r[i] = t;
            }
        } else if constexpr (inertia == 4) {
            constexpr std::size_t k = inertia * 2;
            constexpr std::size_t l = N / k;
            for (std::size_t i = 0; i < l; ++i) {
                convolution.template binomial<inertia>(
                    r.data() + i * k,
                    a.data() + i * k,
                    b.data() + i * k,
                    Z::twiddle(l + i)
                );
                convolution.template binomial<inertia>(
                    r.data() + i * k + inertia,
                    a.data() + i * k + inertia,
                    b.data() + i * k + inertia,
                    -Z::twiddle(l + i)
                );
            }
        } else {
            static_assert(false, "Not implemented");
        }
    }
};

struct Tracer {
    using Convolution = Convolution<Z>::Tracer;

    Convolution convolution;
    std::vector<Z>& trace;

    constexpr Tracer(std::vector<Z>& trace)
        : convolution(trace), trace(trace) {}

    constexpr void cooley_tukey(std::array<Z, N>& a) const {
        return NTT::cooley_tukey(a);
    }

    constexpr void gentleman_sande(std::array<Z, N>& a) const {
        return NTT::gentleman_sande(a);
    }

    constexpr void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        if constexpr (inertia == 1) {
            for (std::size_t i = 0; i < N; ++i) {
                r[i] = trace.emplace_back(
                    a[i] * b[i]
                );
            }
        } else if constexpr (inertia == 4) {
            constexpr std::size_t k = inertia * 2;
            constexpr std::size_t l = N / k;
            for (std::size_t i = 0; i < l; ++i) {
                convolution.template binomial<inertia>(
                    r.data() + i * k,
                    a.data() + i * k,
                    b.data() + i * k,
                    Z::twiddle(l + i)
                );
                convolution.template binomial<inertia>(
                    r.data() + i * k + inertia,
                    a.data() + i * k + inertia,
                    b.data() + i * k + inertia,
                    -Z::twiddle(l + i)
                );
            }
        } else {
            static_assert(false, "Not implemented");
        }
    }

};

};

}

#endif
