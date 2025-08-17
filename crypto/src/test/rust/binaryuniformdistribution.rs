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

use blacknet_crypto::binaryuniformdistribution::BinaryUniformDistribution;
use blacknet_crypto::distribution::{Distribution, UniformGenerator};
use core::array;

type Z = blacknet_crypto::pervushin::PervushinField;

struct TestGenerator {
    i: i16,
}

impl TestGenerator {
    fn new() -> Self {
        Self { i: 1234 }
    }
}

impl UniformGenerator for TestGenerator {
    type Output = Z;

    fn generate(&mut self) -> Self::Output {
        let result = self.i;
        self.i += 1;
        result.into()
    }
}

#[test]
fn reproducible() {
    let mut g = TestGenerator::new();
    let mut bud = BinaryUniformDistribution::<TestGenerator>::default();
    let a: [Z; 16] = [0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0].map(Z::from);
    let b: [Z; 16] = array::from_fn(|_| bud.sample(&mut g));
    assert_eq!(b, a);
}
