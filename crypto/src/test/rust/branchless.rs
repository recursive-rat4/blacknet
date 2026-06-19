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

use blacknet_crypto::branchless::*;

#[test]
fn abs() {
    let a = i16::MAX;
    let b: i16 = 0;
    let c = i16::MIN + 1;
    let d = i16::MIN;

    assert_eq!(a.abs(), a.bl_abs());
    assert_eq!(b.abs(), b.bl_abs());
    assert_eq!(c.abs(), c.bl_abs());

    assert_eq!(a.unsigned_abs(), a.bl_unsigned_abs());
    assert_eq!(b.unsigned_abs(), b.bl_unsigned_abs());
    assert_eq!(c.unsigned_abs(), c.bl_unsigned_abs());
    assert_eq!(d.unsigned_abs(), d.bl_unsigned_abs());

    assert_eq!(a.wrapping_abs(), a.bl_wrapping_abs());
    assert_eq!(b.wrapping_abs(), b.bl_wrapping_abs());
    assert_eq!(c.wrapping_abs(), c.bl_wrapping_abs());
    assert_eq!(d.wrapping_abs(), d.bl_wrapping_abs());
}

#[test]
fn assign() {
    let a: i16 = 2;
    let b: i16 = 3;
    let mut c = a;
    c.bl_assign(b, false);
    assert_eq!(c, a);
    c.bl_assign(b, true);
    assert_eq!(c, b);
}

#[test]
fn eq() {
    let a: [i16; 2] = [2, 2];
    let b: [i16; 2] = [2, 3];

    assert_eq!(false, a.bl_eq(&b));
    assert_eq!(true, a.bl_eq(&a));

    assert_eq!(false, a.bl_ne(&a));
    assert_eq!(true, a.bl_ne(&b));
}

#[test]
fn select() {
    let a: i16 = 2;
    let b: i16 = 3;
    assert_eq!(a, a.bl_select(b, false));
    assert_eq!(b, a.bl_select(b, true));
}

#[test]
fn swap() {
    let (a, b): (i16, i16) = (2, 3);
    let (mut c, mut d) = (a, b);
    c.bl_swap(&mut d, false);
    assert_eq!((a, b), (c, d));
    c.bl_swap(&mut d, true);
    assert_eq!((b, a), (c, d));
}
