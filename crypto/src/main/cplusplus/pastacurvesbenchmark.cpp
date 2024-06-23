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

#include "pastacurves.h"

static boost::random::mt19937 rng;

static void BM_CurveAdd(benchmark::State& state) {
    auto a = PallasGroupJacobian::random(rng);
    auto b = PallasGroupJacobian::random(rng);

    for (auto _ : state)
        a = a + b;

    benchmark::DoNotOptimize(a);
    benchmark::DoNotOptimize(b);
}
BENCHMARK(BM_CurveAdd);

static void BM_CurveDbl(benchmark::State& state) {
    auto a = PallasGroupJacobian::random(rng);

    for (auto _ : state)
        a = a.douple();

    benchmark::DoNotOptimize(a);
}
BENCHMARK(BM_CurveDbl);
