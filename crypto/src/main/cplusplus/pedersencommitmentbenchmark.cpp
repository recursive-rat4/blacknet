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
#include "pedersencommitment.h"

static boost::random::mt19937 rng;

static void BM_PedersenCommitmentAffine(benchmark::State& state) {
    auto g = VestaGroupAffine::random(rng);
    auto h = VestaGroupAffine::random(rng);
    PedersenCommitment cs(g, h);
    auto m = PallasField::random(rng);
    auto r = PallasField::random(rng);

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(m, r)
        );
}
BENCHMARK(BM_PedersenCommitmentAffine);

static void BM_PedersenCommitmentJacobian(benchmark::State& state) {
    auto g = VestaGroupJacobian::random(rng);
    auto h = VestaGroupJacobian::random(rng);
    PedersenCommitment cs(g, h);
    auto m = PallasField::random(rng);
    auto r = PallasField::random(rng);

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(m, r)
        );
}
BENCHMARK(BM_PedersenCommitmentJacobian);

static void BM_PedersenCommitmentProjective(benchmark::State& state) {
    auto g = VestaGroupProjective::random(rng);
    auto h = VestaGroupProjective::random(rng);
    PedersenCommitment cs(g, h);
    auto m = PallasField::random(rng);
    auto r = PallasField::random(rng);

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(m, r)
        );
}
BENCHMARK(BM_PedersenCommitmentProjective);
