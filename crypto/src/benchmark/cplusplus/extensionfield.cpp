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
static void BM_ExtFieldAdd(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a += b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtFieldAdd<Solinas62RingDegree2>);
BENCHMARK(BM_ExtFieldAdd<Solinas62RingDegree3>);
BENCHMARK(BM_ExtFieldAdd<Solinas62RingDegree4>);
BENCHMARK(BM_ExtFieldAdd<PervushinRingDegree2>);
BENCHMARK(BM_ExtFieldAdd<PervushinRingDegree3>);
BENCHMARK(BM_ExtFieldAdd<PervushinRingDegree4>);
BENCHMARK(BM_ExtFieldAdd<LM62RingDegree2>);
BENCHMARK(BM_ExtFieldAdd<LM62RingDegree3>);
BENCHMARK(BM_ExtFieldAdd<LM62RingDegree4>);

template<typename F>
static void BM_ExtFieldSub(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a -= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtFieldSub<Solinas62RingDegree2>);
BENCHMARK(BM_ExtFieldSub<Solinas62RingDegree3>);
BENCHMARK(BM_ExtFieldSub<Solinas62RingDegree4>);
BENCHMARK(BM_ExtFieldSub<PervushinRingDegree2>);
BENCHMARK(BM_ExtFieldSub<PervushinRingDegree3>);
BENCHMARK(BM_ExtFieldSub<PervushinRingDegree4>);
BENCHMARK(BM_ExtFieldSub<LM62RingDegree2>);
BENCHMARK(BM_ExtFieldSub<LM62RingDegree3>);
BENCHMARK(BM_ExtFieldSub<LM62RingDegree4>);

template<typename F>
static void BM_ExtFieldMul(benchmark::State& state) {
    auto a = F::random(rng);
    auto b = F::random(rng);

    for (auto _ : state) {
        a *= b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtFieldMul<Solinas62RingDegree2>);
BENCHMARK(BM_ExtFieldMul<Solinas62RingDegree3>);
BENCHMARK(BM_ExtFieldMul<Solinas62RingDegree4>);
BENCHMARK(BM_ExtFieldMul<PervushinRingDegree2>);
BENCHMARK(BM_ExtFieldMul<PervushinRingDegree3>);
BENCHMARK(BM_ExtFieldMul<PervushinRingDegree4>);
BENCHMARK(BM_ExtFieldMul<LM62RingDegree2>);
BENCHMARK(BM_ExtFieldMul<LM62RingDegree3>);
BENCHMARK(BM_ExtFieldMul<LM62RingDegree4>);

template<typename F>
static void BM_ExtFieldInv(benchmark::State& state) {
    auto a = F::random(rng);

    for (auto _ : state) {
        a = a.invert().value();

        benchmark::DoNotOptimize(a);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_ExtFieldInv<Solinas62RingDegree2>);
BENCHMARK(BM_ExtFieldInv<Solinas62RingDegree3>);
BENCHMARK(BM_ExtFieldInv<Solinas62RingDegree4>);
BENCHMARK(BM_ExtFieldInv<PervushinRingDegree2>);
BENCHMARK(BM_ExtFieldInv<PervushinRingDegree3>);
BENCHMARK(BM_ExtFieldInv<PervushinRingDegree4>);
BENCHMARK(BM_ExtFieldInv<LM62RingDegree2>);
BENCHMARK(BM_ExtFieldInv<LM62RingDegree3>);
BENCHMARK(BM_ExtFieldInv<LM62RingDegree4>);
