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

#ifndef BLACKNET_CRYPTO_JOHNSONLINDENSTRAUSS_H
#define BLACKNET_CRYPTO_JOHNSONLINDENSTRAUSS_H

#include <random>

#include "binaryuniformdistribution.h"
#include "matrixdense.h"
#include "vectordense.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2021/1397.pdf
// A Modular Johnson–Lindenstrauss Variant

template<typename Z>
requires(Z::is_integer_ring)
struct JohnsonLindenstrauss {
    template<std::uniform_random_bit_generator RNG>
    struct DistributionRNG {
        using result_type = Z;

        BinaryUniformDistributionRNG<result_type, RNG> bud;

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr result_type operator () (RNG& rng) {
            return bud(rng) + bud(rng) - Z(1);
        }
    };

    template<typename Sponge>
    struct DistributionSponge {
        using result_type = Z;

        BinaryUniformDistributionSponge<Sponge> bud;

        constexpr DistributionSponge() noexcept = default;

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr result_type operator () (Sponge& sponge) {
            return bud(sponge) + bud(sponge) - Z(1);
        }

    template<typename Builder>
    requires(std::same_as<Z, typename Builder::R>)
    struct Circuit {
        using Variable = Builder::Variable;
        using LinearCombination = Builder::LinearCombination;
        using BinaryUniformDistribution = BinaryUniformDistributionSponge<Sponge>::template Circuit<Builder>;
        using SpongeCircuit = Sponge::template Circuit<Builder>;

        Builder& circuit;
        BinaryUniformDistribution bud;

        constexpr Circuit(Builder& circuit) : circuit(circuit), bud(circuit) {}

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr LinearCombination operator () (SpongeCircuit& sponge) {
            return bud(sponge) + bud(sponge) - Z(1);
        }
    };

    template<std::size_t Degree>
    struct Assigner {
        using BinaryUniformDistribution = BinaryUniformDistributionSponge<Sponge>::template Assigner<Degree>;
        using SpongeAssigner = Sponge::template Assigner<Degree>;

        std::vector<Z>& assigment;
        BinaryUniformDistribution bud;

        constexpr Assigner(std::vector<Z>& assigment) : assigment(assigment), bud(assigment) {}

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr result_type operator () (SpongeAssigner& sponge) {
            return bud(sponge) + bud(sponge) - Z(1);
        }
    };

    };

    constexpr static VectorDense<Z> project(const MatrixDense<Z>& map, const VectorDense<Z>& point) {
        return map * point;
    }

    template<std::uniform_random_bit_generator RNG>
    constexpr static MatrixDense<Z> random(RNG& rng, std::size_t n, std::size_t k) {
        DistributionRNG<RNG> dst;
        return MatrixDense<Z>::random(rng, dst, n, k);
    }
};

}

#endif
