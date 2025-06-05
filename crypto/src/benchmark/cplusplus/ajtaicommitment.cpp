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
#include "matrix.h"
#include "solinas62.h"
#include "solinas62extension.h"
#include "vector.h"

using namespace blacknet::crypto;

static FastDRG rng;

static void BM_AjtaiCommitment(benchmark::State& state) {
    using LatticeFold = LatticeFold<
        Solinas62Ring,
        Solinas62RingDegree2,
        Solinas62RingDegree64,
        Solinas62RingDegree64NTT
    >;
    using R = Solinas62RingDegree64NTT;
    std::size_t M = 1;

    AjtaiCommitment<R, NormP::Infinity> cs(
        Matrix<R>::random(rng, LatticeFold::K, M),
        LatticeFold::B
    );
    Vector<R> m = Vector<R>::random(rng, M);
    Vector<R> c;

    for (auto _ : state) {
        c = cs.commit(m);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(m);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_AjtaiCommitment);
