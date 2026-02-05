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

use blacknet_crypto::bigint::UInt256;
use blacknet_crypto::random::{Distribution, UniformGenerator, UniformIntDistribution};
use core::array;

struct TestGenerator {
    i: u8,
}

impl TestGenerator {
    fn new() -> Self {
        Self { i: 0xE2 }
    }
}

impl UniformGenerator for TestGenerator {
    type Output = u8;

    fn generate(&mut self) -> Self::Output {
        let result = self.i;
        self.i = self.i.wrapping_add(1);
        result.into()
    }
}

#[test]
fn binary() {
    let mut g = TestGenerator::new();
    let mut bud = UniformIntDistribution::<u16, TestGenerator>::new(0..2);
    let a: [u16; 16] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    let b: [u16; 16] = array::from_fn(|_| bud.sample(&mut g));
    assert_eq!(b, a);

    bud.set_range(1..3);
    let c: [u16; 16] = [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2];
    let d: [u16; 16] = array::from_fn(|_| bud.sample(&mut g));
    assert_eq!(d, c);
}

#[test]
fn ternary() {
    let mut g = TestGenerator::new();
    let mut tud = UniformIntDistribution::<i8, TestGenerator>::new(0..3);
    let a: [i8; 6] = [2, 0, 1, 2, 0, 1];
    let b: [i8; 6] = array::from_fn(|_| tud.sample(&mut g));
    assert_eq!(b, a);

    tud.set_range(-1..=1);
    let c: [i8; 6] = [1, -1, 0, 1, -1, 0];
    let d: [i8; 6] = array::from_fn(|_| tud.sample(&mut g));
    assert_eq!(d, c);
}

#[test]
fn u8() {
    let mut g = TestGenerator::new();
    let mut uid = UniformIntDistribution::<u8, TestGenerator>::new(0..=u8::MAX);
    let a: [u8; 6] = [0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7];
    let b: [u8; 6] = array::from_fn(|_| uid.sample(&mut g));
    assert_eq!(b, a);

    uid.set_range(0xE0..0xF0);
    let c: [u8; 6] = [0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED];
    let d: [u8; 6] = array::from_fn(|_| uid.sample(&mut g));
    assert_eq!(d, c);
}

#[test]
fn u16() {
    let mut g = TestGenerator::new();
    let mut uid = UniformIntDistribution::<u16, TestGenerator>::new(0..=u16::MAX);
    let a: [u16; 3] = [0xE3E2, 0xE5E4, 0xE7E6];
    let b: [u16; 3] = array::from_fn(|_| uid.sample(&mut g));
    assert_eq!(b, a);

    uid.set_range(0..256);
    let c: [u16; 6] = [0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED];
    let d: [u16; 6] = array::from_fn(|_| uid.sample(&mut g));
    assert_eq!(d, c);
}

#[test]
fn uint256() {
    let mut g = TestGenerator::new();
    let mut uid =
        UniformIntDistribution::<UInt256, TestGenerator>::new(UInt256::ZERO..=UInt256::MAX);
    let a = UInt256::from_hex("0100FFFEFDFCFBFAF9F8F7F6F5F4F3F2F1F0EFEEEDECEBEAE9E8E7E6E5E4E3E2");
    let b = uid.sample(&mut g);
    assert_eq!(b, a);
}
