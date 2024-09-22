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

static void BM_IntegerRingAdd(benchmark::State& state) {
    using Z = Solinas62Ring;

    auto a = Z::random(rng);
    auto b = Z::random(rng);

    for (auto _ : state)
        a += b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_IntegerRingAdd);

static void BM_IntegerRingSub(benchmark::State& state) {
    using Z = Solinas62Ring;

    auto a = Z::random(rng);
    auto b = Z::random(rng);

    for (auto _ : state)
        a -= b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_IntegerRingSub);

static void BM_IntegerRingMul(benchmark::State& state) {
    using Z = Solinas62Ring;

    auto a = Z::random(rng);
    auto b = Z::random(rng);

    for (auto _ : state)
        a *= b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_IntegerRingMul);

static void BM_IntegerRingInv(benchmark::State& state) {
    using Z = Solinas62Ring;

    auto a = Z::random(rng);

    for (auto _ : state)
        a = a.invert().value();

    benchmark::DoNotOptimize(a);
}
BENCHMARK(BM_IntegerRingInv);
