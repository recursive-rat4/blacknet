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

use core::array;
use core::iter::zip;
use core::mem::MaybeUninit;

pub trait BlSelect<Rps = Self> {
    type Output;

    fn bl_select(self, rps: Rps, condition: bool) -> Self::Output;
}

macro_rules! impl_select {
    ( $($x:ty),+ ) => {
        $(
            impl BlSelect for $x {
                type Output = $x;

                fn bl_select(self, rps: $x, condition: bool) -> $x {
                    let mask = (condition as $x).wrapping_neg();
                    self ^ mask & (self ^ rps)
                }
            }

            impl BlSelect<&$x> for $x {
                type Output = $x;

                fn bl_select(self, rps: &$x, condition: bool) -> $x {
                    let mask = (condition as $x).wrapping_neg();
                    self ^ mask & (self ^ rps)
                }
            }

            impl BlSelect<$x> for &$x {
                type Output = $x;

                fn bl_select(self, rps: $x, condition: bool) -> $x {
                    let mask = (condition as $x).wrapping_neg();
                    self ^ mask & (self ^ rps)
                }
            }

            impl BlSelect for &$x {
                type Output = $x;

                fn bl_select(self, rps: &$x, condition: bool) -> $x {
                    let mask = (condition as $x).wrapping_neg();
                    self ^ mask & (self ^ rps)
                }
            }
        )+
    };
}

impl_select!(i8, i16, i32, i64, u8, u16, u32, u64);

impl BlSelect for bool {
    type Output = bool;

    fn bl_select(self, rps: bool, condition: bool) -> bool {
        self ^ condition & (self ^ rps)
    }
}

impl BlSelect<&bool> for bool {
    type Output = bool;

    fn bl_select(self, rps: &bool, condition: bool) -> bool {
        self ^ condition & (self ^ rps)
    }
}

impl BlSelect<bool> for &bool {
    type Output = bool;

    fn bl_select(self, rps: bool, condition: bool) -> bool {
        self ^ condition & (self ^ rps)
    }
}

impl BlSelect for &bool {
    type Output = bool;

    fn bl_select(self, rps: &bool, condition: bool) -> bool {
        self ^ condition & (self ^ rps)
    }
}

impl<T: BlSelect<Output = T>, const N: usize> BlSelect for [T; N] {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let mut out = [const { MaybeUninit::<T>::uninit() }; N];
        for (o, (l, r)) in zip(&mut out, zip(self, rps)) {
            o.write(l.bl_select(r, condition));
        }
        out.map(|i| unsafe { i.assume_init() })
    }
}

impl<T: for<'a> BlSelect<&'a T, Output = T>, const N: usize> BlSelect<&Self> for [T; N] {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let mut out = [const { MaybeUninit::<T>::uninit() }; N];
        for (o, (l, r)) in zip(&mut out, zip(self, rps)) {
            o.write(l.bl_select(r, condition));
        }
        out.map(|i| unsafe { i.assume_init() })
    }
}

impl<T, const N: usize> BlSelect<[T; N]> for &[T; N]
where
    for<'a> &'a T: BlSelect<T, Output = T>,
{
    type Output = [T; N];

    fn bl_select(self, rps: [T; N], condition: bool) -> Self::Output {
        let mut out = [const { MaybeUninit::<T>::uninit() }; N];
        for (o, (l, r)) in zip(&mut out, zip(self, rps)) {
            o.write(l.bl_select(r, condition));
        }
        out.map(|i| unsafe { i.assume_init() })
    }
}

impl<T, const N: usize> BlSelect for &[T; N]
where
    for<'a> &'a T: BlSelect<Output = T>,
{
    type Output = [T; N];

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        array::from_fn(|i| self[i].bl_select(&rps[i], condition))
    }
}
