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

use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::*;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(crit: &mut Criterion) {
    let secret_key = black_box(
        SecretKey::try_from("1EB1997993844282E4B33463E55B77A6EE8C3ED5AD175CC1F0DB746242EB2DE7")
            .unwrap(),
    );
    let public_key = black_box(
        PublicKey::try_from("27A2C7CE9EE9AF0458832079017A5FBBB1F1551932C4CB901396BAE95F7D0F0A")
            .unwrap(),
    );
    let hash = black_box(
        Hash::try_from("9909FF1647FE84CBA8C3495A2A9BACE506C70B431B08235DBC3FE8EC49429465").unwrap(),
    );
    let signature = black_box(Signature::try_from("6D5D4F6A81C601B1834701BDE84785470F92DFA517975BED9AAEA035FBDB0072327EFD207195B7202B5A72BB9CC37443A011C35137E1DF1C11BB5E9C60125B04").unwrap());

    crit.bench_function("sign", |bench| bench.iter(|| sign(hash, secret_key)));
    crit.bench_function("verify", |bench| {
        bench.iter(|| verify(signature, hash, public_key))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
