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

template<typename ECG>
static void BM_EllipticCurveAdd(benchmark::State& state) {
    auto a = ECG::random(rng);
    auto b = ECG::random(rng);

    for (auto _ : state) {
        a = a + b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_EllipticCurveAdd<Edwards25519GroupExtended>);

template<typename ECG>
static void BM_EllipticCurveDbl(benchmark::State& state) {
    auto a = ECG::random(rng);

    for (auto _ : state) {
        a = a.douple();

        benchmark::DoNotOptimize(a);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_EllipticCurveDbl<Edwards25519GroupExtended>);

template<typename ECG>
static void BM_EllipticCurveSub(benchmark::State& state) {
    auto a = ECG::random(rng);
    auto b = ECG::random(rng);

    for (auto _ : state) {
        a = a - b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_EllipticCurveSub<Edwards25519GroupExtended>);

template<typename ECG>
static void BM_EllipticCurveMul(benchmark::State& state) {
    auto a = ECG::random(rng);
    auto b = ECG::Scalar::random(rng);

    for (auto _ : state) {
        a = a * b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_EllipticCurveMul<Edwards25519GroupExtended>);
