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

use blacknet_crypto::zeroize as z;

#[test]
fn zeroize() {
    let mut data = u128::MAX;
    let zero = 0u128;

    z::zeroize(&mut data);
    assert_eq!(data, zero);
}

#[test]
fn zeroize_slice() {
    let mut data = Box::<[u8]>::from([1, 2, 3, 4, 5, 6]);
    let zero = Box::<[u8]>::from([0, 0, 0, 0, 0, 0]);

    z::zeroize_slice(&mut data);
    assert_eq!(data, zero);
}

#[test]
fn zeroize_string() {
    let mut data = String::from("123456");
    let zero = String::new();

    z::zeroize_string(&mut data);
    assert_eq!(data, zero);

    unsafe {
        let inner = data.as_mut_vec();
        inner.set_len(inner.capacity());
        assert!(inner.len() >= 6);
        for &mut x in inner {
            assert_eq!(x, 0u8);
        }
    }
}

#[test]
fn zeroize_vec() {
    let mut data = Vec::<u8>::from([1, 2, 3, 4, 5, 6]);
    let zero = Vec::<u8>::new();

    z::zeroize_vec(&mut data);
    assert_eq!(data, zero);

    unsafe {
        data.set_len(data.capacity());
        assert!(data.len() >= 6);
        for x in data {
            assert_eq!(x, 0u8);
        }
    }
}

#[test]
fn zeroize_with_default() {
    let mut data = Def(1);
    let zero = Def(255);

    z::zeroize_with_default(&mut data);
    assert_eq!(data, zero);
}

#[derive(Debug, PartialEq)]
struct Def(u8);

impl Default for Def {
    fn default() -> Self {
        Self(u8::MAX)
    }
}

impl Drop for Def {
    fn drop(&mut self) {}
}
