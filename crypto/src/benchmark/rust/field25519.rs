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

use blacknet_crypto::algebra::{Double, Inv, Square};
use blacknet_crypto::field25519::Field25519;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

type F = Field25519;

fn criterion_benchmark(crit: &mut Criterion) {
    let a = black_box(F::from_hex(
        "67B0007C5617DA6052BB49A64A741BC138790506AB0778F58720910B6C450013",
    ));
    let b = black_box(F::from_hex(
        "138DF5B7F9B0821874C7A7D8D80DED276442FB148F1A19790C6FE9315640F917",
    ));

    crit.bench_function("Field25519 add", |bench| bench.iter(|| a + b));
    crit.bench_function("Field25519 dbl", |bench| bench.iter(|| a.double()));
    crit.bench_function("Field25519 sub", |bench| bench.iter(|| a - b));
    crit.bench_function("Field25519 neg", |bench| bench.iter(|| -a));
    crit.bench_function("Field25519 mul", |bench| bench.iter(|| a * b));
    crit.bench_function("Field25519 sqr", |bench| bench.iter(|| a.square()));
    crit.bench_function("Field25519 div", |bench| bench.iter(|| (a / b).unwrap()));
    crit.bench_function("Field25519 inv", |bench| bench.iter(|| a.inv().unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
