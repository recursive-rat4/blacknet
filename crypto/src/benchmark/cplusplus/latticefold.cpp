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
#include "lm62.h"
#include "lm62extension.h"
#include "poseidon2lm62.h"
#include "sumcheck.h"
#include "vectordense.h"

using namespace blacknet::crypto;

static FastDRG rng;

using Z = LM62Ring;
using F = LM62RingDegree2;
using R = LM62RingDegree64;
using LF = LatticeFold<Z, F, R, LM62RingDegree64NTT>;
using Duplex = Poseidon2LM62Sponge<{12, 23, 34, 45}>;

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
    std::vector<VectorDense<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return VectorDense<R>::random(rng, 1); });
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
    std::vector<VectorDense<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return VectorDense<R>::random(rng, 1); });
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
    std::vector<VectorDense<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return VectorDense<R>::random(rng, 1); });
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
    std::vector<VectorDense<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return VectorDense<R>::random(rng, 1); });
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
    std::vector<VectorDense<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return VectorDense<R>::random(rng, 1); });
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
    std::vector<VectorDense<R>> f(LF::k * 2);
    std::ranges::generate(f, [] { return VectorDense<R>::random(rng, 1); });
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
