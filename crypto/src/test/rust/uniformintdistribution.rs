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
use blacknet_crypto::uniformintdistribution::UniformIntDistribution;
use core::array;

struct TestGenerator {
    i: i16,
}

impl TestGenerator {
    fn new() -> Self {
        Self {
            i: 0xE2E4u16 as i16,
        }
    }
}

impl UniformGenerator for TestGenerator {
    type Output = i16;

    fn generate(&mut self) -> Self::Output {
        let result = self.i;
        self.i += 1;
        result.into()
    }
}

#[test]
fn binary() {
    let mut g = TestGenerator::new();
    let mut bud = UniformIntDistribution::<TestGenerator>::new(0..2);
    let a: [u16; 16] = [0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1];
    let b: [u16; 16] = array::from_fn(|_| bud.sample(&mut g));
    assert_eq!(b, a);
}

#[test]
fn ternary() {
    let mut g = TestGenerator::new();
    let mut tud = UniformIntDistribution::<TestGenerator>::new(0..3);
    let a: [u16; 6] = [0, 1, 2, 2, 0, 2];
    let b: [u16; 6] = array::from_fn(|_| tud.sample(&mut g));
    assert_eq!(b, a);
}

#[test]
fn byte() {
    let mut g = TestGenerator::new();
    let mut uid = UniformIntDistribution::<TestGenerator>::new(0..256);
    let a: [u16; 6] = [0xE4, 0xE2, 0xE5, 0xE2, 0xE6, 0xE2];
    let b: [u16; 6] = array::from_fn(|_| uid.sample(&mut g));
    assert_eq!(b, a);
}

#[test]
fn max() {
    let mut g = TestGenerator::new();
    let mut uid = UniformIntDistribution::<TestGenerator>::new(0..u16::MAX);
    let a: [u16; 3] = [0xE2E4, 0xE2E5, 0xE2E6];
    let b: [u16; 3] = array::from_fn(|_| uid.sample(&mut g));
    assert_eq!(b, a);
}
