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

use blacknet_crypto::{
    algebra::{Double, Inv, Square, Zero},
    symmetric::{Blake2bDuplexer, Duplexer},
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::{any::type_name, hint::black_box};

type F = blacknet_crypto::ed25519::Field25519;

fn name() -> &'static str {
    type_name::<F>().split("::").last().unwrap()
}

fn element(drg: &mut impl Duplexer<Msg = u8>) -> F {
    loop {
        let element: F = drg.squeeze();
        if element != F::ZERO {
            return element.square();
        }
    }
}

fn criterion_benchmark(crit: &mut Criterion) {
    let field_name = name();
    let mut bench_group = crit.benchmark_group(field_name);
    bench_group.throughput(Throughput::Elements(1));
    let (a, b): (F, F) = {
        let mut drg = Blake2bDuplexer::new();
        (element(&mut drg), element(&mut drg))
    };

    bench_group.bench_function("add", |bench| bench.iter(|| black_box(a) + black_box(b)));
    bench_group.bench_function("dbl", |bench| bench.iter(|| black_box(a).double()));
    bench_group.bench_function("sub", |bench| bench.iter(|| black_box(a) - black_box(b)));
    bench_group.bench_function("neg", |bench| bench.iter(|| -black_box(a)));
    bench_group.bench_function("mul", |bench| bench.iter(|| black_box(a) * black_box(b)));
    bench_group.bench_function("sqr", |bench| bench.iter(|| black_box(a).square()));
    bench_group.bench_function("div", |bench| bench.iter(|| black_box(a) / black_box(b)));
    bench_group.bench_function("inv", |bench| bench.iter(|| black_box(a).inv()));

    bench_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
