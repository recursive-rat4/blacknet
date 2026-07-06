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

/// Conditional swap.
pub trait BlSwap {
    /// Swap if condition is true.
    fn bl_swap(&mut self, rps: &mut Self, condition: bool);
}

macro_rules! impl_swap {
    ( $($x:ty),+ ) => {
        $(
            impl BlSwap for $x {
                fn bl_swap(&mut self, rps: &mut $x, condition: bool) {
                    cfg_select! {
                        feature = "cmov" => {
                            use cmov::Cmov;
                            let t = *self;
                            self.cmovnz(&*rps, condition as u8);
                            rps.cmovnz(&t, condition as u8);
                        }
                        _ => {
                            let mask = (condition as $x).wrapping_neg();
                            let t = mask & (&*self ^ &*rps);
                            *self ^= t;
                            *rps ^= t;
                        }
                    }
                }
            }
        )+
    };
}

impl_swap!(i8, i16, i32, i64, u8, u16, u32, u64);

impl BlSwap for bool {
    fn bl_swap(&mut self, rps: &mut bool, condition: bool) {
        cfg_select! {
            feature = "cmov" => {
                use cmov::Cmov;
                let (mut l, mut r) = (*self as u8, *rps as u8);
                let t = l;
                l.cmovnz(&r, condition as u8);
                r.cmovnz(&t, condition as u8);
                (*self, *rps) = (l != 0, r != 0);
            }
            _ => {
                let t = condition & (*self ^ *rps);
                *self ^= t;
                *rps ^= t;
            }
        }
    }
}

impl<T: BlSwap, const N: usize> BlSwap for [T; N] {
    fn bl_swap(&mut self, rps: &mut Self, condition: bool) {
        for (l, r) in zip(self, rps) {
            l.bl_swap(r, condition)
        }
    }
}
