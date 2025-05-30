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

using Z = Solinas62Ring;
using F = Solinas62RingDegree2;
using LF = LatticeFold<Z, F>;
using R = LF::Rq;
using Duplex = Poseidon2Solinas62Sponge<{123, 234, 345, 456}>;

static void BM_LatticeFold_GEval_SumCheck_Prove(benchmark::State& state) {
    using SumCheck = SumCheck<F, LF::GEval, Duplex>;

    std::vector<F> alpha(LF::k * 2);
    std::ranges::generate(alpha, [] { return F::random(rng); });
    std::vector<std::vector<F>> r(LF::k * 2);
    std::ranges::generate(r, [] {
        std::vector<F> ri(6);
        std::ranges::generate(ri, [] {
            return F::random(rng);
        });
        return ri;
    });
    std::vector<Vector<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LF::GEval g(alpha, r, f);

    F sum = Hypercube<F>::sum(g);
    SumCheck::Proof proof;

    for (auto _ : state) {
        Duplex duplex;
        proof = SumCheck::prove(g, sum, duplex);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GEval_SumCheck_Prove);

static void BM_LatticeFold_GEval_SumCheck_Verify(benchmark::State& state) {
    using SumCheck = SumCheck<F, LF::GEval, Duplex>;

    std::vector<F> alpha(LF::k * 2);
    std::ranges::generate(alpha, [] { return F::random(rng); });
    std::vector<std::vector<F>> r(LF::k * 2);
    std::ranges::generate(r, [] {
        std::vector<F> ri(6);
        std::ranges::generate(ri, [] {
            return F::random(rng);
        });
        return ri;
    });
    std::vector<Vector<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LF::GEval g(alpha, r, f);

    Duplex duplex;
    F sum = Hypercube<F>::sum(g);
    SumCheck::Proof proof = SumCheck::prove(g, sum, duplex);
    bool result;

    for (auto _ : state) {
        Duplex duplex;
        result = SumCheck::verify(g, sum, proof, duplex);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::DoNotOptimize(result);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GEval_SumCheck_Verify);

static void BM_LatticeFold_GNorm_SumCheck_Prove(benchmark::State& state) {
    using SumCheck = SumCheck<F, LF::GNorm, Duplex>;

    F beta = F::random(rng);
    std::vector<F> mu(LF::k * 2);
    std::ranges::generate(mu, [] { return F::random(rng); });
    std::vector<Vector<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LF::GNorm g(beta, mu, f);

    F sum = Hypercube<F>::sum(g);
    SumCheck::Proof proof;

    for (auto _ : state) {
        Duplex duplex;
        proof = SumCheck::prove(g, sum, duplex);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GNorm_SumCheck_Prove);

static void BM_LatticeFold_GNorm_SumCheck_Verify(benchmark::State& state) {
    using SumCheck = SumCheck<F, LF::GNorm, Duplex>;

    F beta = F::random(rng);
    std::vector<F> mu(LF::k * 2);
    std::ranges::generate(mu, [] { return F::random(rng); });
    std::vector<Vector<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LF::GNorm g(beta, mu, f);

    Duplex duplex;
    F sum = Hypercube<F>::sum(g);
    SumCheck::Proof proof = SumCheck::prove(g, sum, duplex);
    bool result;

    for (auto _ : state) {
        Duplex duplex;
        result = SumCheck::verify(g, sum, proof, duplex);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::DoNotOptimize(result);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GNorm_SumCheck_Verify);

static void BM_LatticeFold_GFold_SumCheck_Prove(benchmark::State& state) {
    using SumCheck = SumCheck<F, LF::GFold, Duplex>;

    std::vector<F> alpha(LF::k * 2);
    std::ranges::generate(alpha, [] { return F::random(rng); });
    F beta = F::random(rng);
    std::vector<F> mu(LF::k * 2);
    std::ranges::generate(mu, [] { return F::random(rng); });
    std::vector<std::vector<F>> r(LF::k * 2);
    std::ranges::generate(r, [] {
        std::vector<F> ri(6);
        std::ranges::generate(ri, [] {
            return F::random(rng);
        });
        return ri;
    });
    std::vector<Vector<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LF::GFold g(alpha, beta, mu, r, f);

    F sum = Hypercube<F>::sum(g);
    SumCheck::Proof proof;

    for (auto _ : state) {
        Duplex duplex;
        proof = SumCheck::prove(g, sum, duplex);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GFold_SumCheck_Prove);

static void BM_LatticeFold_GFold_SumCheck_Verify(benchmark::State& state) {
    using SumCheck = SumCheck<F, LF::GFold, Duplex>;

    std::vector<F> alpha(LF::k * 2);
    std::ranges::generate(alpha, [] { return F::random(rng); });
    F beta = F::random(rng);
    std::vector<F> mu(LF::k * 2);
    std::ranges::generate(mu, [] { return F::random(rng); });
    std::vector<std::vector<F>> r(LF::k * 2);
    std::ranges::generate(r, [] {
        std::vector<F> ri(6);
        std::ranges::generate(ri, [] {
            return F::random(rng);
        });
        return ri;
    });
    std::vector<Vector<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return Vector<R>::random(rng, 1); });
    LF::GFold g(alpha, beta, mu, r, f);

    Duplex duplex;
    F sum = Hypercube<F>::sum(g);
    SumCheck::Proof proof = SumCheck::prove(g, sum, duplex);
    bool result;

    for (auto _ : state) {
        Duplex duplex;
        result = SumCheck::verify(g, sum, proof, duplex);

        benchmark::DoNotOptimize(g);
        benchmark::DoNotOptimize(proof);
        benchmark::DoNotOptimize(sum);
        benchmark::DoNotOptimize(result);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_LatticeFold_GFold_SumCheck_Verify);
