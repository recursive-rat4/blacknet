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

use blacknet_crypto::symmetric::chacha::{BLOCK_SIZE, ChaCha, IV_SIZE, KEY_SIZE};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

const N: usize = BLOCK_SIZE;

fn criterion_benchmark(crit: &mut Criterion) {
    let mut grp = crit.benchmark_group("ChaCha");
    grp.throughput(Throughput::Bytes(N as u64));

    let key = black_box([0u8; KEY_SIZE]);
    let iv = black_box([0u8; IV_SIZE]);
    let plain_text = black_box([0u8; N]);
    let mut cipher_text = black_box([0u8; N]);
    let mut chacha8 = black_box(ChaCha::<8>::new(&key, &iv));
    let mut chacha20 = black_box(ChaCha::<20>::new(&key, &iv));

    grp.bench_function("8", |bench| {
        bench.iter(|| chacha8.encrypt(&mut cipher_text, &plain_text))
    });
    grp.bench_function("20", |bench| {
        bench.iter(|| chacha20.encrypt(&mut cipher_text, &plain_text))
    });

    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
