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

#include "ajtaicommitment.h"
#include "fastrng.h"
#include "latticefold.h"
#include "lm62.h"
#include "lm62extension.h"
#include "matrixdense.h"
#include "vectordense.h"
#include "vectorsparse.h"

using namespace blacknet::crypto;

static FastDRG rng;

static void BM_AjtaiCommitment_Dense(benchmark::State& state) {
    using LatticeFold = LatticeFold<
        LM62Ring,
        LM62RingDegree2,
        LM62RingDegree64,
        LM62RingDegree64NTT
    >;
    using R = LM62RingDegree64NTT;
    std::size_t M = 8;

    AjtaiCommitment<R, NormP::Infinity> cs(
        MatrixDense<R>::random(rng, LatticeFold::K, M),
        LatticeFold::B
    );
    VectorDense<R> m = VectorDense<R>::random(rng, M);
    VectorDense<R> c;

    for (auto _ : state) {
        c = cs.commit(m);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(m);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_AjtaiCommitment_Dense);

static void BM_AjtaiCommitment_Sparse(benchmark::State& state) {
    using LatticeFold = LatticeFold<
        LM62Ring,
        LM62RingDegree2,
        LM62RingDegree64,
        LM62RingDegree64NTT
    >;
    using R = LM62RingDegree64NTT;
    std::size_t M = 8;

    AjtaiCommitment<R, NormP::Infinity> cs(
        MatrixDense<R>::random(rng, LatticeFold::K, M),
        LatticeFold::B
    );
    R m1 = R::random(rng);
    VectorSparse<R> m(
        M,
        { M - 1 },
        { m1 }
    );
    VectorDense<R> c;

    for (auto _ : state) {
        c = cs.commit(m);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(m);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_AjtaiCommitment_Sparse);
