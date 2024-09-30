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
#include "poseidon2pervushin.h"
#include "poseidon2solinas62.h"

static boost::random::mt19937 rng;

template<typename Params>
static void BM_Poseidon2(benchmark::State& state) {
    using F = Params::F;

    std::array<F, Params::t> m;
    for (std::size_t i = 0; i < Params::t; ++i) m[i] = F::random(rng);

    for (auto _ : state)
        poseidon2::permute<Params>(m);

    benchmark::DoNotOptimize(m);
}
BENCHMARK(BM_Poseidon2<Poseidon2PallasParams>);
BENCHMARK(BM_Poseidon2<Poseidon2Solinas62Params>);
BENCHMARK(BM_Poseidon2<Poseidon2PervushinParams>);
