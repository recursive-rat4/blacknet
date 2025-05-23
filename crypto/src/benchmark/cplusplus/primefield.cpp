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

static void BM_FieldAdd(benchmark::State& state) {
    auto a = PallasField::random(rng);
    auto b = PallasField::random(rng);

    for (auto _ : state) {
        a += b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_FieldAdd);

static void BM_FieldSub(benchmark::State& state) {
    auto a = PallasField::random(rng);
    auto b = PallasField::random(rng);

    for (auto _ : state) {
        a -= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_FieldSub);

static void BM_FieldMul(benchmark::State& state) {
    auto a = PallasField::random(rng);
    auto b = PallasField::random(rng);

    for (auto _ : state) {
        a *= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_FieldMul);

static void BM_FieldDiv(benchmark::State& state) {
    auto a = PallasField::random(rng);
    auto b = PallasField::random(rng);

    for (auto _ : state) {
        a /= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_FieldDiv);
