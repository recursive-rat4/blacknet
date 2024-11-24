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

#include "ajtaicommitment.h"
#include "latticefold.h"
#include "matrix.h"
#include "solinas62.h"
#include "vector.h"

static boost::random::mt19937 rng;

static void BM_AjtaiCommitment(benchmark::State& state) {
    using LatticeFold = LatticeFold<Solinas62Ring, Solinas62RingDegree4>;
    using R = LatticeFold::Rq;
    std::size_t M = 1;

    AjtaiCommitment<R> cs(
        Matrix<R>::random(rng, LatticeFold::K, M)
    );
    Vector<R> m = Vector<R>::random(rng, M);

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(m)
        );
}
BENCHMARK(BM_AjtaiCommitment);
