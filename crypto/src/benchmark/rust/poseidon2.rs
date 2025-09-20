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

use blacknet_crypto::lm::LMField;
use blacknet_crypto::pervushin::PervushinField;
use blacknet_crypto::poseidon2::Poseidon2Plain;
use blacknet_crypto::poseidon2lm::Poseidon2LM12;
use blacknet_crypto::poseidon2pervushin::Poseidon2Pervushin12;
use blacknet_crypto::ring::IntegerRing;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(crit: &mut Criterion) {
    let mut a = black_box([LMField::new(1234567); 12]);
    crit.bench_function("Poseidon2 LMField", |bench| {
        bench.iter(|| Poseidon2LM12::permute(&mut a))
    });
    let mut b = black_box([PervushinField::new(1234567); 12]);
    crit.bench_function("Poseidon2 PervushinField", |bench| {
        bench.iter(|| Poseidon2Pervushin12::permute(&mut b))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
