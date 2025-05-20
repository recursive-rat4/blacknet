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

#include "fastrng.h"
#include "hypercube.h"
#include "latticefold.h"
#include "poseidon2solinas62.h"
#include "solinas62.h"
#include "solinas62field.h"
#include "sumcheck.h"
#include "vector.h"

using namespace blacknet::crypto;

static FastDRG rng;

static void BM_LatticeFold_GNorm_SumCheck_ProveEarlyStopping(benchmark::State& state) {
    using Z = Solinas62Ring;
    using F = Solinas62RingDegree2;
    using LatticeFold = LatticeFold<Z, F>;
    using R = LatticeFold::Rq;
    using S = Poseidon2Solinas62Sponge<{123, 234, 345, 456}>;
    using SumCheck = SumCheck<F, LatticeFold::GNorm, S>;

    F beta = F::random(rng);
    std::vector<F> mu(LatticeFold::k * 2);
    std::ranges::generate(mu, [] { return F::random(rng); });
    std::vector<Vector<R>> f(LatticeFold::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LatticeFold::GNorm g(beta, mu, f);

    F sum = Hypercube<F>::sum(g);
    SumCheck::ProofEarlyStopped proof;

    for (auto _ : state) {
        proof = SumCheck::proveEarlyStopping(g, sum);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GNorm_SumCheck_ProveEarlyStopping);

static void BM_LatticeFold_GNorm_SumCheck_VerifyEarlyStopping(benchmark::State& state) {
    using Z = Solinas62Ring;
    using F = Solinas62RingDegree2;
    using LatticeFold = LatticeFold<Z, F>;
    using R = LatticeFold::Rq;
    using S = Poseidon2Solinas62Sponge<{123, 234, 345, 456}>;
    using SumCheck = SumCheck<F, LatticeFold::GNorm, S>;

    F beta = F::random(rng);
    std::vector<F> mu(LatticeFold::k * 2);
    std::ranges::generate(mu, [] { return F::random(rng); });
    std::vector<Vector<R>> f(LatticeFold::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LatticeFold::GNorm g(beta, mu, f);

    F sum = Hypercube<F>::sum(g);
    SumCheck::ProofEarlyStopped proof = SumCheck::proveEarlyStopping(g, sum);
    bool result;

    for (auto _ : state) {
        result = SumCheck::verifyEarlyStopping(g, sum, proof);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::DoNotOptimize(result);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GNorm_SumCheck_VerifyEarlyStopping);
