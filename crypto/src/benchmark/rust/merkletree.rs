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

use blacknet_crypto::{
    algebra::Zero,
    lm::LMField,
    pervushin::PervushinField,
    symmetric::{Blake2b256, MerkleTree, TruncPoseidon2LM, TruncPoseidon2Pervushin},
};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const H: usize = 4;

fn criterion_benchmark(crit: &mut Criterion) {
    let mut grp = crit.benchmark_group("MerkleTree");

    let index = black_box(0);
    let hash = black_box([0u8; 32]);
    let branch = black_box([hash; H]);
    grp.bench_function("Blake2b256", |bench| {
        bench.iter(|| MerkleTree::<Blake2b256>::compute_root(index, hash, &branch))
    });

    let hash = black_box([LMField::ZERO; 4]);
    let branch = black_box([hash; H]);
    grp.bench_function("Poseidon2LM", |bench| {
        bench.iter(|| MerkleTree::<TruncPoseidon2LM>::compute_root(index, hash, &branch))
    });

    let hash = black_box([PervushinField::ZERO; 4]);
    let branch = black_box([hash; H]);
    grp.bench_function("Poseidon2Pervushin", |bench| {
        bench.iter(|| MerkleTree::<TruncPoseidon2Pervushin>::compute_root(index, hash, &branch))
    });

    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
