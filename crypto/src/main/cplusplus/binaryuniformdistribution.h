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

#ifndef BLACKNET_CRYPTO_BINARYUNIFORMDISTRIBUTION_H
#define BLACKNET_CRYPTO_BINARYUNIFORMDISTRIBUTION_H

#include <bit>
#include <cstddef>
#include <random>

#include "latticegadget.h"
#include "logicgate.h"
#include "vectordense.h"

namespace blacknet::crypto {

template<
    typename T,
    std::uniform_random_bit_generator RNG
>
class BinaryUniformDistributionRNG {
    using NumericType = RNG::result_type;

    consteval static std::size_t useful_bits() {
        return sizeof(NumericType) * 8;
    }

    NumericType cache;
    std::size_t have_bits;
public:
    using result_type = T;

    constexpr BinaryUniformDistributionRNG() noexcept {
        reset();
    }

    constexpr void reset() noexcept {
        have_bits = 0;
    }

    constexpr result_type operator () (RNG& rng) {
        if (have_bits == 0) {
            cache = rng();
            have_bits = useful_bits();
        }
        result_type result = cache & 1;
        cache >>= 1;
        --have_bits;
        return result;
    }
};

template<
    typename Sponge
>
class BinaryUniformDistributionSponge {
    using Z = Sponge::Z;
    static_assert(Z::is_integer_ring, "Not implemented");

    using NumericType = Z::NumericType;

    consteval static std::size_t useful_bits() {
        if constexpr (std::has_single_bit(Z::modulus()))
            return Z::bits();
        else
            return Z::bits() - 1;
    }

    NumericType cache;
    std::size_t have_bits;
public:
    using result_type = Z;

    constexpr BinaryUniformDistributionSponge() noexcept {
        reset();
    }

    constexpr void reset() noexcept {
        have_bits = 0;
    }

    constexpr result_type operator () (Sponge& sponge) {
        if (have_bits == 0) {
            cache = sponge.squeeze().canonical();
            have_bits = useful_bits();
        }
        result_type result = cache & 1;
        cache >>= 1;
        --have_bits;
        return result;
    }

template<typename Builder>
requires(std::same_as<Z, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using LogicGate = LogicGate<Z>::template Circuit<Builder>;
    using SpongeCircuit = Sponge::template Circuit<Builder>;
    using VectorDense = VectorDense<Z>::template Circuit<Builder>;

    Builder* circuit;
    VectorDense cache;
    std::size_t have_bits;

    constexpr Circuit(Builder* circuit)
        : circuit(circuit), cache(circuit, Z::bits())
    {
        reset();
    }

    constexpr void reset() noexcept {
        have_bits = 0;
    }

    constexpr LinearCombination operator () (SpongeCircuit& sponge) {
        if (have_bits == 0) {
            auto scope = circuit->scope("BinaryUniformDistribution::sample");
            auto squeezed = sponge.squeeze();
            Z p = Z::multiplicative_identity();
            LinearCombination composed;
            for (std::size_t i = 0; i < Z::bits(); ++i) {
                LinearCombination digit = circuit->auxiliary();
                cache[i] = digit;
                composed += digit * p;
                p = p.douple();
            }
            auto m1_gadget = LatticeGadget<Z>::decompose(2, Z::bits(), Z(-1)); //XXX make static?
            LogicGate(circuit).LessOrEqualCheck(cache, m1_gadget);
            scope(squeezed == composed);
            have_bits = useful_bits();
        }
        LinearCombination result = cache[useful_bits() - have_bits];
        --have_bits;
        return result;
    }
};

template<std::size_t Degree>
struct Assigner {
    using LogicGate = LogicGate<Z>::template Assigner<Degree>;
    using SpongeAssigner = Sponge::template Assigner<Degree>;

    VectorDense<Z> cache;
    std::size_t have_bits;
    std::vector<Z>* assigment;

    constexpr Assigner(std::vector<Z>* assigment)
        : cache(Z::bits()), assigment(assigment)
    {
        reset();
    }

    constexpr void reset() noexcept {
        have_bits = 0;
    }

    constexpr result_type operator () (SpongeAssigner& sponge) {
        if (have_bits == 0) {
            auto representative = sponge.squeeze().canonical();
            for (std::size_t j = 0; j < Z::bits(); ++j) {
                assigment->push_back(
                    cache[j] = representative & 1
                );
                representative >>= 1;
            }
            auto m1_gadget = LatticeGadget<Z>::decompose(2, Z::bits(), Z(-1)); //XXX make static?
            LogicGate(assigment).LessOrEqualCheck(cache, m1_gadget);
            have_bits = useful_bits();
        }
        result_type result = cache[useful_bits() - have_bits];
        --have_bits;
        return result;
    }
};

};

}

#endif
