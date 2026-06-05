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

use blacknet_crypto::algebra::{IntegerRing, Inv, Sqrt, Square};
use blacknet_crypto::gf2::GF2;
use blacknet_crypto::norm::InfinityNorm;

#[test]
fn congruence() {
    assert_eq!(GF2::new(-2), GF2::new(2));
    assert_ne!(GF2::new(-3), GF2::new(2));
}

#[test]
fn add() {
    assert_eq!(GF2::new(0) + GF2::new(0), GF2::new(0));
    assert_eq!(GF2::new(0) + GF2::new(1), GF2::new(1));
    assert_eq!(GF2::new(1) + GF2::new(0), GF2::new(1));
    assert_eq!(GF2::new(1) + GF2::new(1), GF2::new(0));
}

#[test]
fn mul() {
    assert_eq!(GF2::new(0) * GF2::new(0), GF2::new(0));
    assert_eq!(GF2::new(0) * GF2::new(1), GF2::new(0));
    assert_eq!(GF2::new(1) * GF2::new(0), GF2::new(0));
    assert_eq!(GF2::new(1) * GF2::new(1), GF2::new(1));
}

#[test]
fn sqr() {
    assert_eq!(GF2::new(0).square(), GF2::new(0));
    assert_eq!(GF2::new(1).square(), GF2::new(1));
}

#[test]
fn neg() {
    assert_eq!(-GF2::new(0), GF2::new(0));
    assert_eq!(-GF2::new(1), GF2::new(1));
}

#[test]
fn sub() {
    assert_eq!(GF2::new(0) - GF2::new(0), GF2::new(0));
    assert_eq!(GF2::new(0) - GF2::new(1), GF2::new(1));
    assert_eq!(GF2::new(1) - GF2::new(0), GF2::new(1));
    assert_eq!(GF2::new(1) - GF2::new(1), GF2::new(0));
}

#[test]
fn inv() {
    assert_eq!(GF2::new(1).inv().unwrap(), GF2::new(1));
    assert!(GF2::new(0).inv().is_none());
}

#[test]
fn div() {
    assert_eq!(GF2::new(0) / GF2::new(0), None);
    assert_eq!((GF2::new(0) / GF2::new(1)).unwrap(), GF2::new(0));
    assert_eq!(GF2::new(1) / GF2::new(0), None);
    assert_eq!((GF2::new(1) / GF2::new(1)).unwrap(), GF2::new(1));
}

#[test]
fn sqrt() {
    assert_eq!(GF2::new(0).sqrt(), GF2::new(0));
    assert_eq!(GF2::new(1).sqrt(), GF2::new(1));
}

#[test]
fn infinity_norm() {
    assert!(!GF2::new(0).check_infinity_norm(&0));
    assert!(GF2::new(0).check_infinity_norm(&1));
    assert!(!GF2::new(1).check_infinity_norm(&0));
    assert!(!GF2::new(1).check_infinity_norm(&1));
}
