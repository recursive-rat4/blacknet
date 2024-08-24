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

#include "poseidon2pasta.h"
#include "poseidon2solinas62.h"

static boost::random::mt19937 rng;

static void BM_Poseidon2_256(benchmark::State& state) {
    using F = PallasField;
    constexpr auto& params = Poseidon2Pallas;

    std::array<F, params.t()> m;
    for (std::size_t i = 0; i < params.t(); ++i) m[i] = F::random(rng);

    for (auto _ : state)
        poseidon2::permute(params, m);

    benchmark::DoNotOptimize(m);
}
BENCHMARK(BM_Poseidon2_256);

static void BM_Poseidon2_64(benchmark::State& state) {
    using R = Solinas62Ring;
    constexpr auto& params = Poseidon2Solinas62;

    std::array<R, params.t()> m;
    for (std::size_t i = 0; i < params.t(); ++i) m[i] = R::random(rng);

    for (auto _ : state)
        poseidon2::permute(params, m);

    benchmark::DoNotOptimize(m);
}
BENCHMARK(BM_Poseidon2_64);
