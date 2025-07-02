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
#include "lm62extension.h"
#include "pervushinextension.h"
#include "solinas62extension.h"

using namespace blacknet::crypto;

static FastDRG rng;

template<typename F>
static void BM_ExtensionFieldAdd(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a += b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtensionFieldAdd<Solinas62RingDegree2>);
BENCHMARK(BM_ExtensionFieldAdd<Solinas62RingDegree3>);
BENCHMARK(BM_ExtensionFieldAdd<Solinas62RingDegree4>);
BENCHMARK(BM_ExtensionFieldAdd<PervushinRingDegree2>);
BENCHMARK(BM_ExtensionFieldAdd<PervushinRingDegree3>);
BENCHMARK(BM_ExtensionFieldAdd<PervushinRingDegree4>);
BENCHMARK(BM_ExtensionFieldAdd<LM62RingDegree2>);
BENCHMARK(BM_ExtensionFieldAdd<LM62RingDegree3>);
BENCHMARK(BM_ExtensionFieldAdd<LM62RingDegree4>);

template<typename F>
static void BM_ExtensionFieldSub(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a -= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtensionFieldSub<Solinas62RingDegree2>);
BENCHMARK(BM_ExtensionFieldSub<Solinas62RingDegree3>);
BENCHMARK(BM_ExtensionFieldSub<Solinas62RingDegree4>);
BENCHMARK(BM_ExtensionFieldSub<PervushinRingDegree2>);
BENCHMARK(BM_ExtensionFieldSub<PervushinRingDegree3>);
BENCHMARK(BM_ExtensionFieldSub<PervushinRingDegree4>);
BENCHMARK(BM_ExtensionFieldSub<LM62RingDegree2>);
BENCHMARK(BM_ExtensionFieldSub<LM62RingDegree3>);
BENCHMARK(BM_ExtensionFieldSub<LM62RingDegree4>);

template<typename F>
static void BM_ExtensionFieldMul(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a *= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtensionFieldMul<Solinas62RingDegree2>);
BENCHMARK(BM_ExtensionFieldMul<Solinas62RingDegree3>);
BENCHMARK(BM_ExtensionFieldMul<Solinas62RingDegree4>);
BENCHMARK(BM_ExtensionFieldMul<PervushinRingDegree2>);
BENCHMARK(BM_ExtensionFieldMul<PervushinRingDegree3>);
BENCHMARK(BM_ExtensionFieldMul<PervushinRingDegree4>);
BENCHMARK(BM_ExtensionFieldMul<LM62RingDegree2>);
BENCHMARK(BM_ExtensionFieldMul<LM62RingDegree3>);
BENCHMARK(BM_ExtensionFieldMul<LM62RingDegree4>);

template<typename F>
static void BM_ExtensionFieldInv(benchmark::State& state) {
    auto a = F::random(rng);

    for (auto _ : state) {
        a = a.invert().value();

        benchmark::DoNotOptimize(a);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtensionFieldInv<Solinas62RingDegree2>);
BENCHMARK(BM_ExtensionFieldInv<Solinas62RingDegree3>);
BENCHMARK(BM_ExtensionFieldInv<Solinas62RingDegree4>);
BENCHMARK(BM_ExtensionFieldInv<PervushinRingDegree2>);
BENCHMARK(BM_ExtensionFieldInv<PervushinRingDegree3>);
BENCHMARK(BM_ExtensionFieldInv<PervushinRingDegree4>);
BENCHMARK(BM_ExtensionFieldInv<LM62RingDegree2>);
BENCHMARK(BM_ExtensionFieldInv<LM62RingDegree3>);
BENCHMARK(BM_ExtensionFieldInv<LM62RingDegree4>);
