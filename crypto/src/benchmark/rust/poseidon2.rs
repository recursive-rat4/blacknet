/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

use blacknet_crypto::algebra::IntegerModRing;
use blacknet_crypto::lm::LMField;
use blacknet_crypto::pervushin::PervushinField;
use blacknet_crypto::symmetric::{Poseidon2LM12, Poseidon2Pervushin12, Poseidon2Plain};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(crit: &mut Criterion) {
    let mut grp = crit.benchmark_group("Poseidon2");
    grp.throughput(Throughput::Elements(12));

    let mut a = black_box([LMField::new(1234567); 12]);
    grp.bench_function("LMField", |bench| {
        bench.iter(|| Poseidon2LM12::permute(&mut a))
    });
    let mut b = black_box([PervushinField::new(1234567); 12]);
    grp.bench_function("PervushinField", |bench| {
        bench.iter(|| Poseidon2Pervushin12::permute(&mut b))
    });

    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
