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

#include "solinas62.h"

static boost::random::mt19937 rng;

static void BM_ExtFieldAdd(benchmark::State& state) {
    using F = Solinas62RingDegree2;

    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state)
        a += b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_ExtFieldAdd);

static void BM_ExtFieldSub(benchmark::State& state) {
    using F = Solinas62RingDegree2;

    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state)
        a -= b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_ExtFieldSub);

static void BM_ExtFieldMul(benchmark::State& state) {
    using F = Solinas62RingDegree2;

    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state)
        a *= b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_ExtFieldMul);
