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

#include "blacklemon.h"
#include "fastrng.h"

using namespace blacknet::crypto;

static FastDRG rng;

static void BM_BlackLemon_GenerateSecretKey(benchmark::State& state) {
    BlackLemon bl;
    BlackLemon::SecretKey sk;

    for (auto _ : state) {
        sk = bl.generateSecretKey(rng);

        benchmark::DoNotOptimize(sk);
        benchmark::DoNotOptimize(rng);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_BlackLemon_GenerateSecretKey);

static void BM_BlackLemon_GeneratePublicKey(benchmark::State& state) {
    BlackLemon bl;
    BlackLemon::SecretKey sk = bl.generateSecretKey(rng);
    BlackLemon::PublicKey pk;

    for (auto _ : state) {
        pk = bl.generatePublicKey(rng, sk);

        benchmark::DoNotOptimize(sk);
        benchmark::DoNotOptimize(pk);
        benchmark::DoNotOptimize(rng);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_BlackLemon_GeneratePublicKey);

static void BM_BlackLemon_Encrypt(benchmark::State& state) {
    BlackLemon bl;
    BlackLemon::SecretKey sk = bl.generateSecretKey(rng);
    BlackLemon::PublicKey pk = bl.generatePublicKey(rng, sk);
    BlackLemon::PlainText pt = BlackLemon::Rt::random(rng);
    BlackLemon::CipherText ct;

    for (auto _ : state) {
        ct = bl.encrypt(rng, pk, pt);

        benchmark::DoNotOptimize(pk);
        benchmark::DoNotOptimize(pt);
        benchmark::DoNotOptimize(ct);
        benchmark::DoNotOptimize(rng);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_BlackLemon_Encrypt);

static void BM_BlackLemon_Decrypt(benchmark::State& state) {
    BlackLemon bl;
    BlackLemon::SecretKey sk = bl.generateSecretKey(rng);
    BlackLemon::CipherText ct = { BlackLemon::Rq::random(rng), BlackLemon::Rq::random(rng)};
    BlackLemon::PlainText pt;

    for (auto _ : state) {
        pt = bl.decrypt(sk, ct);

        benchmark::DoNotOptimize(sk);
        benchmark::DoNotOptimize(pt);
        benchmark::DoNotOptimize(ct);
        benchmark::ClobberMemory();
    }
}
BENCHMARK(BM_BlackLemon_Decrypt);
