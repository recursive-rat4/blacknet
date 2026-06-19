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

pub trait BlAssign<Rps = Self> {
    fn bl_assign(&mut self, rps: Rps, condition: bool);
}

macro_rules! impl_assign {
    ( $($x:ty),+ ) => {
        $(
            impl BlAssign for $x {
                fn bl_assign(&mut self, rps: $x, condition: bool) {
                    let mask = (condition as $x).wrapping_neg();
                    *self ^= mask & (&*self ^ rps);
                }
            }

            impl BlAssign<&$x> for $x {
                fn bl_assign(&mut self, rps: &$x, condition: bool) {
                    let mask = (condition as $x).wrapping_neg();
                    *self ^= mask & (&*self ^ rps);
                }
            }
        )+
    };
}

impl_assign!(i8, i16, i32, i64, u8, u16, u32, u64);

impl BlAssign for bool {
    fn bl_assign(&mut self, rps: bool, condition: bool) {
        *self ^= condition & (*self ^ rps);
    }
}

impl BlAssign<&bool> for bool {
    fn bl_assign(&mut self, rps: &bool, condition: bool) {
        *self ^= condition & (*self ^ rps);
    }
}

impl<T: BlAssign, const N: usize> BlAssign for [T; N] {
    fn bl_assign(&mut self, rps: Self, condition: bool) {
        for (l, r) in zip(self, rps) {
            l.bl_assign(r, condition)
        }
    }
}

impl<T: for<'a> BlAssign<&'a T>, const N: usize> BlAssign<&Self> for [T; N] {
    fn bl_assign(&mut self, rps: &Self, condition: bool) {
        for (l, r) in zip(self, rps) {
            l.bl_assign(r, condition)
        }
    }
}
