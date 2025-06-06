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

#include "dilithium.h"
#include "fastrng.h"
#include "solinas62extension.h"

using namespace blacknet::crypto;

static FastDRG rng;

template<typename R>
static void BM_CyclotomicMul(benchmark::State& state) {
    auto a = R::random(rng);
    auto b = R::random(rng);

    for (auto _ : state) {
        a = a * b;

        benchmark::DoNotOptimize(a);
        benchmark::DoNotOptimize(b);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_CyclotomicMul<dilithium::Rq>);
BENCHMARK(BM_CyclotomicMul<Solinas62RingDegree64NTT>);

template<typename R>
static void BM_CyclotomicCnj(benchmark::State& state) {
    auto a = R::random(rng);

    for (auto _ : state) {
        a = a.conjugate();

        benchmark::DoNotOptimize(a);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_CyclotomicCnj<dilithium::Rq>);
BENCHMARK(BM_CyclotomicCnj<Solinas62RingDegree64NTT>);
