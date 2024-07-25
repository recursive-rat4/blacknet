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
#include <vector>

#include "pastacurves.h"
#include "pedersencommitment.h"

static boost::random::mt19937 rng;

static void BM_PedersenCommitmentAffine_1KB(benchmark::State& state) {
    PedersenCommitment<VestaGroupAffine> cs({
        VestaGroupAffine::random(rng),
        VestaGroupAffine::random(rng),
        VestaGroupAffine::random(rng),
        VestaGroupAffine::random(rng),
    });
    std::vector<PallasField> v{
        PallasField::random(rng),
        PallasField::random(rng),
        PallasField::random(rng),
        PallasField::random(rng),
    };

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(v)
        );
}
BENCHMARK(BM_PedersenCommitmentAffine_1KB);

static void BM_PedersenCommitmentJacobian_1KB(benchmark::State& state) {
    PedersenCommitment<VestaGroupJacobian> cs({
        VestaGroupJacobian::random(rng),
        VestaGroupJacobian::random(rng),
        VestaGroupJacobian::random(rng),
        VestaGroupJacobian::random(rng),
    });
    std::vector<PallasField> v{
        PallasField::random(rng),
        PallasField::random(rng),
        PallasField::random(rng),
        PallasField::random(rng),
    };

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(v)
        );
}
BENCHMARK(BM_PedersenCommitmentJacobian_1KB);

static void BM_PedersenCommitmentProjective_1KB(benchmark::State& state) {
    PedersenCommitment<VestaGroupProjective> cs({
        VestaGroupProjective::random(rng),
        VestaGroupProjective::random(rng),
        VestaGroupProjective::random(rng),
        VestaGroupProjective::random(rng),
    });
    std::vector<PallasField> v{
        PallasField::random(rng),
        PallasField::random(rng),
        PallasField::random(rng),
        PallasField::random(rng),
    };

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(v)
        );
}
BENCHMARK(BM_PedersenCommitmentProjective_1KB);
