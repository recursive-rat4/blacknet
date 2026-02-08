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

use blacknet_crypto::lpr::*;
use blacknet_crypto::random::FastDRG;
use core::array;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(crit: &mut Criterion) {
    let mut drg = black_box(FastDRG::default());
    let bytes: [u8; 128] = black_box(array::from_fn(|i| i as u8));
    let pt = black_box(encode(&bytes));
    let sk = black_box(generate_secret_key(&mut drg));
    let pk = black_box(generate_public_key(&mut drg, &sk));
    let ct = black_box(encrypt(&mut drg, &pk, &pt));

    crit.bench_function("LPR generate_secret_key", |bench| {
        bench.iter(|| generate_secret_key(&mut drg))
    });
    crit.bench_function("LPR generate_public_key", |bench| {
        bench.iter(|| generate_public_key(&mut drg, &sk))
    });
    crit.bench_function("LPR encrypt", |bench| {
        bench.iter(|| encrypt(&mut drg, &pk, &pt))
    });
    crit.bench_function("LPR decrypt", |bench| bench.iter(|| decrypt(&sk, &ct)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
