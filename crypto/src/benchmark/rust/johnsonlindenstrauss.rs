/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use blacknet_crypto::johnsonlindenstrauss::JohnsonLindenstrauss;
use blacknet_crypto::matrix::DenseVector;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

type Z = blacknet_crypto::lm::LMField;
type DRG = blacknet_crypto::symmetric::DuplexPoseidon2LM;

const N: usize = 1024;

fn criterion_benchmark(crit: &mut Criterion) {
    let mut grp = crit.benchmark_group("JohnsonLindenstrauss");
    grp.throughput(Throughput::Elements(N as u64));

    let mut drg = black_box(DRG::default());
    let jl = black_box(JohnsonLindenstrauss::<Z>::random(&mut drg, N));
    let v: DenseVector<Z> = black_box((0..N).map(|i| Z::from(i as i32)).collect());

    grp.bench_function("sample", |bench| {
        bench.iter(|| JohnsonLindenstrauss::<Z>::random(&mut drg, N))
    });
    grp.bench_function("project", |bench| bench.iter(|| jl.project(&v)));

    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
