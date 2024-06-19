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

#include "pastacurves.h"
#include "pedersencommitment.h"

static void BM_PedersenCommitment(benchmark::State& state) {
    VestaField  gx; std::istringstream("33074014122d93a8ac69e0bbc472768ebf2760c0e53f73abf0e395d8b1b5b478") >> gx;
    VestaField  gy; std::istringstream("3604f572d11bae3cccf8a6895d8e06b3c7388e54a5acda9f5e62d33a72bbc566") >> gy;
    VestaField  hx; std::istringstream("245a92dbb72f4e95e0be3595344d0bc58978c7b7c9c1a5b2128d9d7eb3d6328a") >> hx;
    VestaField  hy; std::istringstream("11bac7e68bd74ee7a7a43f6b1f9e206e8b8ac7c8d2bae596ef891c301155ad1e") >> hy;
    PallasField m1; std::istringstream("09e21902c37d0c6dc4c1c8143faefa86a192cac72bdc0d89828a2d1ce3d813b3") >> m1;
    PallasField r1; std::istringstream("1ab0bd7178dbc83ec8ec11aa0bf46e5cae406812d865fa9a96beccac98aa0f5d") >> r1;
    VestaGroupProjective g(gx, gy, VestaField(1));
    VestaGroupProjective h(hx, hy, VestaField(1));
    PedersenCommitment cs(g, h);

    for (auto _ : state)
        benchmark::DoNotOptimize(
            cs.commit(m1, r1)
        );
}
BENCHMARK(BM_PedersenCommitment);
