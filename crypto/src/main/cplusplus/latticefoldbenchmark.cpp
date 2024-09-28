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

#include <benchmark/benchmark.h>
#include <boost/random/mersenne_twister.hpp>

#include "latticefold.h"
#include "poseidon2solinas62.h"
#include "solinas62.h"
#include "sumcheck.h"
#include "vector.h"

static boost::random::mt19937 rng;

static void BM_LatticeFold_G2_SumCheck_Prove(benchmark::State& state) {
    using Z = Solinas62Ring;
    using F = Solinas62RingDegree3;
    using R = latticefold::Rq<Z>;
    using S = Poseidon2Solinas62;
    using SumCheck = SumCheck<Z, F, latticefold::G2, S>;

    std::vector<Z> beta(6);
    std::ranges::generate(beta, [] { return Z::random(rng); });
    Vector<R> f{R::random(rng)};
    latticefold::G2<Z> g2(beta, f);

    SumCheck::Proof proof;

    for (auto _ : state) {
        proof = SumCheck::prove(g2);

        benchmark::DoNotOptimize(g2);
        benchmark::DoNotOptimize(proof);
    }
}
BENCHMARK(BM_LatticeFold_G2_SumCheck_Prove);

static void BM_LatticeFold_G2_SumCheck_Verify(benchmark::State& state) {
    using Z = Solinas62Ring;
    using F = Solinas62RingDegree3;
    using R = latticefold::Rq<Z>;
    using S = Poseidon2Solinas62;
    using SumCheck = SumCheck<Z, F, latticefold::G2, S>;

    std::vector<Z> beta(6);
    std::ranges::generate(beta, [] { return Z::random(rng); });
    Vector<R> f{R::random(rng)};
    latticefold::G2<Z> g2(beta, f);

    SumCheck::Proof proof = SumCheck::prove(g2);
    Z sum = proof.claims[0](F(0)).coefficients[0] + proof.claims[0](F(1)).coefficients[0];
    bool result;

    for (auto _ : state) {
        result = SumCheck::verify(g2, sum, proof);

        benchmark::DoNotOptimize(g2);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::DoNotOptimize(result);
    }
}
BENCHMARK(BM_LatticeFold_G2_SumCheck_Verify);
