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
#include "pastacurves.h"
#include "pedersencommitment.h"
#include "vectordense.h"

using namespace blacknet::crypto;

static FastDRG rng;

template<typename G>
static void BM_PedersenCommitmentSingle(benchmark::State& state) {
    using Scalar = G::Scalar;

    PedersenCommitment<G> cs({
        G::random(rng),
        G::random(rng),
    });
    Scalar s{ Scalar::random(rng) };
    Scalar t{ Scalar::random(rng) };
    G c;

    for (auto _ : state) {
        c = cs.commit(s, t);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(s);
        benchmark::DoNotOptimize(t);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PedersenCommitmentSingle<VestaGroupAffine>);
BENCHMARK(BM_PedersenCommitmentSingle<VestaGroupJacobian>);
BENCHMARK(BM_PedersenCommitmentSingle<VestaGroupProjective>);

template<typename G>
static void BM_PedersenCommitmentVector(benchmark::State& state) {
    using Scalar = G::Scalar;

    PedersenCommitment<G> cs({
        G::random(rng),
        G::random(rng),
        G::random(rng),
        G::random(rng),
    });
    VectorDense<Scalar> v{
        Scalar::random(rng),
        Scalar::random(rng),
        Scalar::random(rng),
        Scalar::random(rng),
    };
    G c;

    for (auto _ : state) {
        c = cs.commit(v);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(v);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PedersenCommitmentVector<VestaGroupAffine>);
BENCHMARK(BM_PedersenCommitmentVector<VestaGroupJacobian>);
BENCHMARK(BM_PedersenCommitmentVector<VestaGroupProjective>);
