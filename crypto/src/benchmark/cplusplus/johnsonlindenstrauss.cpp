/*
 * Copyright (c) 2025 Pavel Vasin
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
#include "johnsonlindenstrauss.h"
#include "lm62.h"
#include "matrix.h"
#include "vector.h"

using namespace blacknet::crypto;

using Z = LM62Ring;
using JL = JohnsonLindenstrauss<Z>;

static std::size_t M = 256;
static std::size_t N = 1024;

static FastDRG rng;

static void BM_JohnsonLindenstrauss_Sample(benchmark::State& state) {
    Matrix<Z> map;

    for (auto _ : state) {
        map = JL::random(rng, M, N);

        benchmark::DoNotOptimize(map);
        benchmark::DoNotOptimize(rng);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_JohnsonLindenstrauss_Sample);

static void BM_JohnsonLindenstrauss_Project(benchmark::State& state) {
    Matrix<Z> map = JL::random(rng, M, N);
    Vector<Z> high = Vector<Z>::random(rng, N);
    Vector<Z> low;

    for (auto _ : state) {
        low = JL::project(map, high);

        benchmark::DoNotOptimize(map);
        benchmark::DoNotOptimize(high);
        benchmark::DoNotOptimize(low);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_JohnsonLindenstrauss_Project);
