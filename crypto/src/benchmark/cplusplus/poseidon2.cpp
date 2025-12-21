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
#include "poseidon2lm62.h"
#include "poseidon2pervushin.h"
#include "poseidon2solinas62.h"

using namespace blacknet::crypto;

static FastDRG rng;

template<typename Params>
static void BM_Poseidon2(benchmark::State& state) {
    using F = Params::F;

    std::array<F, Params::t> m;
    for (std::size_t i = 0; i < Params::t; ++i) m[i] = F::random(rng);

    for (auto _ : state) {
        Poseidon2<Params>::permute(m);

        benchmark::DoNotOptimize(m);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_Poseidon2<Poseidon2Solinas62SpongeParams>);
BENCHMARK(BM_Poseidon2<Poseidon2PervushinSpongeParams>);
BENCHMARK(BM_Poseidon2<Poseidon2LM62SpongeParams>);
