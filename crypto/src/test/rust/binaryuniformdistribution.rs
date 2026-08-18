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

use blacknet_crypto::random::{BinaryUniformDistribution, Distribution, UniformGenerator};
use core::array;

type Z = blacknet_crypto::uring::U8Ring;

struct TestGenerator {
    i: u8,
}

impl TestGenerator {
    fn new() -> Self {
        Self { i: 0xD7 }
    }
}

impl UniformGenerator for TestGenerator {
    type Output = u8;

    fn generate(&mut self) -> Self::Output {
        let ret = self.i;
        self.i = self.i.wrapping_add(1);
        ret
    }
}

#[test]
fn reproducible() {
    let mut g = TestGenerator::new();
    let mut bud = BinaryUniformDistribution::new();
    let a: [Z; 16] = [1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1].map(Z::from);
    let b: [Z; 16] = array::from_fn(|_| bud.sample(&mut g));
    assert_eq!(b, a);
}
