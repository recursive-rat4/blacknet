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

#include "dilithium.h"
#include "latticefold.h"
#include "polynomialring.h"
#include "solinas62.h"
#include "solinas62field.h"

static boost::random::mt19937 rng;

static void BM_CyclotomicMul_32_256(benchmark::State& state) {
    using R = dilithium::Rq;

    auto a = R::random(rng);
    auto b = R::random(rng);

    for (auto _ : state)
        a = a * b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_CyclotomicMul_32_256);

static void BM_CyclotomicMul_32_64(benchmark::State& state) {
    using R = CyclotomicRing<dilithium::Zq, 64>;

    auto a = R::random(rng);
    auto b = R::random(rng);

    for (auto _ : state)
        a = a * b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_CyclotomicMul_32_64);

static void BM_CyclotomicMul_64_256(benchmark::State& state) {
    using R = CyclotomicRing<Solinas62Ring, 256>;

    auto a = R::random(rng);
    auto b = R::random(rng);

    for (auto _ : state)
        a = a * b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_CyclotomicMul_64_256);

static void BM_CyclotomicMul_64_64(benchmark::State& state) {
    using R = LatticeFold<Solinas62Ring, Solinas62RingDegree4>::Rq;

    auto a = R::random(rng);
    auto b = R::random(rng);

    for (auto _ : state)
        a = a * b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_CyclotomicMul_64_64);
