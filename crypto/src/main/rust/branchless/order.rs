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

use core::iter::zip;

pub trait BlEq<Rps = Self> {
    fn bl_eq(&self, rps: &Rps) -> bool;

    fn bl_ne(&self, rps: &Rps) -> bool;
}

macro_rules! impl_eq {
    ( $($x:ty),+ ) => {
        $(
            impl BlEq for $x {
                fn bl_eq(&self, rps: &$x) -> bool {
                    self == rps
                }

                fn bl_ne(&self, rps: &$x) -> bool {
                    self != rps
                }
            }
        )+
    };
}

impl_eq!(bool, i8, i16, i32, i64, u8, u16, u32, u64);

impl<T: BlEq, const N: usize> BlEq for [T; N] {
    fn bl_eq(&self, rps: &Self) -> bool {
        let mut eq = true;
        for (l, r) in zip(self, rps) {
            eq &= l.bl_eq(r)
        }
        eq
    }

    fn bl_ne(&self, rps: &Self) -> bool {
        let mut ne = false;
        for (l, r) in zip(self, rps) {
            ne |= l.bl_ne(r)
        }
        ne
    }
}

pub trait BlOrd<Rps = Self>: BlEq<Rps> {
    fn bl_gt(&self, rps: &Rps) -> bool;

    fn bl_lt(&self, rps: &Rps) -> bool;
}

macro_rules! impl_ord {
    ( $($x:ty),+ ) => {
        $(
            impl BlOrd for $x {
                fn bl_gt(&self, rps: &$x) -> bool {
                    self > rps
                }

                fn bl_lt(&self, rps: &$x) -> bool {
                    self < rps
                }
            }
        )+
    };
}

impl_ord!(i8, i16, i32, i64, u8, u16, u32, u64);

impl BlOrd for bool {
    fn bl_gt(&self, rps: &bool) -> bool {
        (self == &true) & (rps == &false)
    }

    fn bl_lt(&self, rps: &bool) -> bool {
        (self == &false) & (rps == &true)
    }
}
