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

use blacknet_crypto::distribution::{Distribution, UniformGenerator};
use blacknet_crypto::ternaryuniformdistribution::TernaryUniformDistribution;
use core::array;

type Int = i16;

struct TestGenerator {
    i: Int,
}

impl TestGenerator {
    fn new() -> Self {
        Self {
            i: 0xE2E4u16 as i16,
        }
    }
}

impl UniformGenerator for TestGenerator {
    type Output = Int;

    fn generate(&mut self) -> Self::Output {
        let result = self.i;
        self.i += 1;
        result.into()
    }
}

#[test]
fn reproducible() {
    let mut g = TestGenerator::new();
    let mut tud = TernaryUniformDistribution::<TestGenerator>::default();
    let a: [Int; 6] = [-1, 0, 1, 1, -1, 1];
    let b: [Int; 6] = array::from_fn(|_| tud.sample(&mut g));
    assert_eq!(b, a);
}
