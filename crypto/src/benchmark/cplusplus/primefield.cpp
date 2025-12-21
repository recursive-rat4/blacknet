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

#include "edwards25519.h"
#include "fastrng.h"

using namespace blacknet::crypto;

static FastDRG rng;

template<typename F>
static void BM_PrimeFieldAdd(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a += b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PrimeFieldAdd<Field25519>);

template<typename F>
static void BM_PrimeFieldSub(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a -= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PrimeFieldSub<Field25519>);

template<typename F>
static void BM_PrimeFieldMul(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a *= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PrimeFieldMul<Field25519>);

template<typename F>
static void BM_PrimeFieldDiv(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a /= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PrimeFieldDiv<Field25519>);
