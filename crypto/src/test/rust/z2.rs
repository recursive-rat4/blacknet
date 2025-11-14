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

use blacknet_crypto::norm::InfinityNorm;
use blacknet_crypto::operation::{Inv, Square};
use blacknet_crypto::ring::IntegerRing;
use blacknet_crypto::z2::Z2;

#[test]
fn congruence() {
    assert_eq!(Z2::new(-2), Z2::new(2));
    assert_ne!(Z2::new(-3), Z2::new(2));
}

#[test]
fn add() {
    assert_eq!(Z2::new(0) + Z2::new(0), Z2::new(0));
    assert_eq!(Z2::new(0) + Z2::new(1), Z2::new(1));
    assert_eq!(Z2::new(1) + Z2::new(0), Z2::new(1));
    assert_eq!(Z2::new(1) + Z2::new(1), Z2::new(0));
}

#[test]
fn mul() {
    assert_eq!(Z2::new(0) * Z2::new(0), Z2::new(0));
    assert_eq!(Z2::new(0) * Z2::new(1), Z2::new(0));
    assert_eq!(Z2::new(1) * Z2::new(0), Z2::new(0));
    assert_eq!(Z2::new(1) * Z2::new(1), Z2::new(1));
}

#[test]
fn sqr() {
    assert_eq!(Z2::new(0).square(), Z2::new(0));
    assert_eq!(Z2::new(1).square(), Z2::new(1));
}

#[test]
fn sub() {
    assert_eq!(Z2::new(0) - Z2::new(0), Z2::new(0));
    assert_eq!(Z2::new(0) - Z2::new(1), Z2::new(1));
    assert_eq!(Z2::new(1) - Z2::new(0), Z2::new(1));
    assert_eq!(Z2::new(1) - Z2::new(1), Z2::new(0));
}

#[test]
fn inv() {
    assert_eq!(Z2::new(1).inv().unwrap(), Z2::new(1));
    assert!(Z2::new(0).inv().is_none());
}

#[test]
fn infinity_norm() {
    assert!(!Z2::new(0).check_infinity_norm(0));
    assert!(Z2::new(0).check_infinity_norm(1));
    assert!(!Z2::new(1).check_infinity_norm(0));
    assert!(!Z2::new(1).check_infinity_norm(1));
}
