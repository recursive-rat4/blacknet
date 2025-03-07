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
#include <boost/random/mersenne_twister.hpp>
#include <vector>

#include "pastacurves.h"
#include "pedersencommitment.h"

static boost::random::mt19937 rng;

static void BM_PedersenCommitmentAffine(benchmark::State& state) {
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
    VestaGroupAffine c;

    for (auto _ : state) {
        c = cs.commit(v);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(v);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PedersenCommitmentAffine);

static void BM_PedersenCommitmentJacobian(benchmark::State& state) {
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
    VestaGroupJacobian c;

    for (auto _ : state) {
        c = cs.commit(v);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(v);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PedersenCommitmentJacobian);

static void BM_PedersenCommitmentProjective(benchmark::State& state) {
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
    VestaGroupProjective c;

    for (auto _ : state) {
        c = cs.commit(v);

        benchmark::DoNotOptimize(cs);
        benchmark::DoNotOptimize(v);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_PedersenCommitmentProjective);
