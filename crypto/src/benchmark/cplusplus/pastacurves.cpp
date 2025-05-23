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

using namespace blacknet::crypto;

static FastDRG rng;

static void BM_CurveAdd(benchmark::State& state) {
    auto a = PallasGroupJacobian::random(rng);
    auto b = PallasGroupJacobian::random(rng);

    for (auto _ : state) {
        a = a + b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_CurveAdd);

static void BM_CurveDbl(benchmark::State& state) {
    auto a = PallasGroupJacobian::random(rng);

    for (auto _ : state) {
        a = a.douple();

        benchmark::DoNotOptimize(a);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_CurveDbl);

static void BM_CurveSub(benchmark::State& state) {
    auto a = PallasGroupJacobian::random(rng);
    auto b = PallasGroupJacobian::random(rng);

    for (auto _ : state) {
        a = a - b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_CurveSub);

static void BM_CurveMul(benchmark::State& state) {
    auto a = PallasGroupJacobian::random(rng);
    auto b = VestaField::random(rng);

    for (auto _ : state) {
        a = a * b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_CurveMul);
