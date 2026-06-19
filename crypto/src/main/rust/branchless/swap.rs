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

pub trait BlSwap {
    fn bl_swap(&mut self, rps: &mut Self, condition: bool);
}

macro_rules! impl_swap {
    ( $($x:ty),+ ) => {
        $(
            impl BlSwap for $x {
                fn bl_swap(&mut self, rps: &mut $x, condition: bool) {
                    let mask = (condition as $x).wrapping_neg();
                    let t = mask & (&*self ^ &*rps);
                    *self ^= t;
                    *rps ^= t;
                }
            }
        )+
    };
}

impl_swap!(i8, i16, i32, i64, u8, u16, u32, u64);

impl BlSwap for bool {
    fn bl_swap(&mut self, rps: &mut bool, condition: bool) {
        let t = condition & (*self ^ *rps);
        *self ^= t;
        *rps ^= t;
    }
}

impl<T: BlSwap, const N: usize> BlSwap for [T; N] {
    fn bl_swap(&mut self, rps: &mut Self, condition: bool) {
        for (l, r) in zip(self, rps) {
            l.bl_swap(r, condition)
        }
    }
}
