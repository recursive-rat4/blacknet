/*
 * Copyright (c) 2026 Pavel Vasin
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

use blacknet_crypto::symmetric::{Blake2b256, Blake2b512};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

const N: usize = 128;

fn criterion_benchmark(crit: &mut Criterion) {
    let mut grp = crit.benchmark_group("Blake2b");
    grp.throughput(Throughput::Bytes(N as u64));

    let message = black_box([0u8; N]);

    grp.bench_function("256", |bench| bench.iter(|| Blake2b256::digest(message)));
    grp.bench_function("512", |bench| bench.iter(|| Blake2b512::digest(message)));

    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
