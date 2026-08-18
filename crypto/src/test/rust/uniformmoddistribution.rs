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

use blacknet_crypto::algebra::IntegerModRing;
use blacknet_crypto::random::{Distribution, UniformGenerator, UniformModDistribution};
use core::array;

struct TestGenerator {
    i: u8,
}

impl TestGenerator {
    fn new() -> Self {
        Self { i: 0xFD }
    }
}

impl UniformGenerator for TestGenerator {
    type Output = u8;

    fn generate(&mut self) -> Self::Output {
        let result = self.i;
        self.i = self.i.wrapping_add(1);
        result
    }
}

#[test]
fn even_prime() {
    type Z = blacknet_crypto::gf2::GF2;
    let mut g = TestGenerator::new();
    let mut umd = UniformModDistribution::<Z>::new();
    let a: [Z; 4] = [1, 0, 1, 0].map(Z::with_int);
    let b: [Z; 4] = array::from_fn(|_| umd.sample(&mut g));
    assert_eq!(b, a);
}

#[test]
fn pow2() {
    type Z = blacknet_crypto::uring::U8Ring;
    let mut g = TestGenerator::new();
    let mut umd = UniformModDistribution::<Z>::new();
    let a: [Z; 4] = [0xFD, 0xFE, 0xFF, 0x00].map(Z::with_int);
    let b: [Z; 4] = array::from_fn(|_| umd.sample(&mut g));
    assert_eq!(b, a);
}

#[test]
fn odd_prime() {
    type Z = blacknet_crypto::fermat::FermatField;
    let mut g = TestGenerator::new();
    let mut umd = UniformModDistribution::<Z>::new();
    let a: [Z; 4] = [0x0100, 0x0706, 0x0D0C, 0x1312].map(Z::with_int);
    let b: [Z; 4] = array::from_fn(|_| umd.sample(&mut g));
    assert_eq!(b, a);
}
