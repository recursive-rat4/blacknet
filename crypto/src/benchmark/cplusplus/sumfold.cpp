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

#include <benchmark/benchmark.h>
#include <boost/random/mersenne_twister.hpp>

#include "hypercube.h"
#include "latticefold.h"
#include "poseidon2solinas62.h"
#include "solinas62.h"
#include "solinas62field.h"
#include "sumcheck.h"
#include "vector.h"

static boost::random::mt19937 rng;

static void BM_LatticeFold_GNorm_SumCheck_ProveEarlyStopping(benchmark::State& state) {
    using Z = Solinas62Ring;
    using F = Solinas62RingDegree3;
    using LatticeFold = LatticeFold<Z>;
    using R = LatticeFold::Rq;
    using S = Poseidon2Solinas62Sponge<{123, 234, 345, 456}>;
    using SumCheck = SumCheck<Z, F, LatticeFold::GNorm, S>;

    Z beta = Z::random(rng);
    std::vector<Z> mu(LatticeFold::k * 2);
    std::ranges::generate(mu, [] { return Z::random(rng); });
    std::vector<Vector<R>> f(LatticeFold::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LatticeFold::GNorm<Z> g(beta, mu, f);

    F sum_morphed = Hypercube<Z>::sum(g);
    LatticeFold::GNorm<F> g_morphed = g.template homomorph<F>();
    SumCheck::ProofEarlyStopped proof;

    for (auto _ : state) {
        proof = SumCheck::proveEarlyStopping(g_morphed, sum_morphed);

        benchmark::DoNotOptimize(g_morphed);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum_morphed);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GNorm_SumCheck_ProveEarlyStopping);

static void BM_LatticeFold_GNorm_SumCheck_VerifyEarlyStopping(benchmark::State& state) {
    using Z = Solinas62Ring;
    using F = Solinas62RingDegree3;
    using LatticeFold = LatticeFold<Z>;
    using R = LatticeFold::Rq;
    using S = Poseidon2Solinas62Sponge<{123, 234, 345, 456}>;
    using SumCheck = SumCheck<Z, F, LatticeFold::GNorm, S>;

    Z beta = Z::random(rng);
    std::vector<Z> mu(LatticeFold::k * 2);
    std::ranges::generate(mu, [] { return Z::random(rng); });
    std::vector<Vector<R>> f(LatticeFold::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LatticeFold::GNorm<Z> g(beta, mu, f);

    F sum_morphed = Hypercube<Z>::sum(g);
    LatticeFold::GNorm<F> g_morphed = g.template homomorph<F>();
    SumCheck::ProofEarlyStopped proof = SumCheck::proveEarlyStopping(g_morphed, sum_morphed);
    bool result;

    for (auto _ : state) {
        result = SumCheck::verifyEarlyStopping(g_morphed, sum_morphed, proof);

        benchmark::DoNotOptimize(g_morphed);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum_morphed);
        benchmark::DoNotOptimize(result);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GNorm_SumCheck_VerifyEarlyStopping);
