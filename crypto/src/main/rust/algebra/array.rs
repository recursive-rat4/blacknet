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

use crate::algebra::{Dot, Double, Inv, LeftOne, LeftZero, One, RightOne, RightZero, Square, Zero};
use crate::branchless::{BlAssign, BlEq, BlOption, BlSelect};
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use core::array;
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum, zip};
use core::mem::MaybeUninit;
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Array with entrywise operations.
#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize, Zeroize)]
#[serde(bound(
    deserialize = "[T; N]: Deserialize<'de>",
    serialize = "[T; N]: Serialize"
))]
#[repr(transparent)]
pub struct Array<T, const N: usize> {
    values: [T; N],
}

impl<T, const N: usize> Array<T, N> {
    pub const fn new(values: [T; N]) -> Self {
        Self { values }
    }

    #[inline]
    pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
        Self {
            values: array::from_fn(f),
        }
    }

    #[inline]
    pub fn map<F: FnMut(T) -> U, U>(self, f: F) -> Array<U, N> {
        Array::<U, N> {
            values: self.values.map(f),
        }
    }
}

impl<T: Zero, const N: usize> Default for Array<T, N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<T, const N: usize> From<[T; N]> for Array<T, N> {
    #[inline]
    fn from(values: [T; N]) -> Self {
        Self { values }
    }
}

impl<T, const N: usize> From<Array<T, N>> for [T; N] {
    #[inline]
    fn from(array: Array<T, N>) -> Self {
        array.values
    }
}

impl<T: Debug, const N: usize> Debug for Array<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.values)
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Array<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T; N] {
        &self.values
    }
}

impl<T, const N: usize> AsMut<[T; N]> for Array<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.values
    }
}

impl<T, const N: usize> Borrow<[T; N]> for Array<T, N> {
    #[inline]
    fn borrow(&self) -> &[T; N] {
        &self.values
    }
}

impl<T, const N: usize> BorrowMut<[T; N]> for Array<T, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T; N] {
        &mut self.values
    }
}

impl<T, const N: usize> Deref for Array<T, N> {
    type Target = [T; N];

    #[inline]
    fn deref(&self) -> &[T; N] {
        &self.values
    }
}

impl<T, const N: usize> DerefMut for Array<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl<T, const N: usize> Index<usize> for Array<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.values[i]
    }
}

impl<T, const N: usize> IndexMut<usize> for Array<T, N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.values[i]
    }
}

impl<T, const N: usize> IntoIterator for Array<T, N> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Array<T, N> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Array<T, N> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

#[cfg(feature = "rayon")]
impl<T: Send, const N: usize> IntoParallelIterator for Array<T, N> {
    type Item = T;
    type Iter = rayon::array::IntoIter<T, N>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.values.into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, T: Sync, const N: usize> IntoParallelIterator for &'a Array<T, N> {
    type Item = &'a T;
    type Iter = rayon::slice::Iter<'a, T>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&self.values).into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, T: Send, const N: usize> IntoParallelIterator for &'a mut Array<T, N> {
    type Item = &'a mut T;
    type Iter = rayon::slice::IterMut<'a, T>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&mut self.values).into_par_iter()
    }
}

impl<T: AddAssign, const N: usize> Add for Array<T, N> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l += r);
        lps
    }
}

impl<T: for<'a> AddAssign<&'a T>, const N: usize> Add<&Self> for Array<T, N> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l += r);
        lps
    }
}

impl<T, const N: usize> Add<Array<T, N>> for &Array<T, N>
where
    for<'a> &'a T: Add<T, Output = T>,
{
    type Output = Array<T, N>;

    fn add(self, rps: Array<T, N>) -> Self::Output {
        let mut values = [const { MaybeUninit::<T>::uninit() }; N];
        zip(&mut values, zip(self, rps)).for_each(|(o, (l, r))| {
            o.write(l + r);
        });
        let values = values.map(|i| unsafe { i.assume_init() });
        Array::<T, N> { values }
    }
}

impl<'a, T, const N: usize> Add<&'a Array<T, N>> for &Array<T, N>
where
    for<'b> &'b T: Add<Output = T>,
{
    type Output = Array<T, N>;

    fn add(self, rps: &'a Array<T, N>) -> Self::Output {
        Self::Output::from_fn(|i| &self.values[i] + &rps.values[i])
    }
}

impl<T: AddAssign, const N: usize> AddAssign for Array<T, N> {
    fn add_assign(&mut self, rps: Self) {
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<T: for<'a> AddAssign<&'a T>, const N: usize> AddAssign<&Self> for Array<T, N> {
    fn add_assign(&mut self, rps: &Self) {
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<T: Double<Output = T>, const N: usize> Double for Array<T, N> {
    type Output = Self;

    fn double(self) -> Self {
        self.map(Double::double)
    }
}

impl<T, const N: usize> Double for &Array<T, N>
where
    for<'a> &'a T: Double<Output = T>,
{
    type Output = Array<T, N>;

    fn double(self) -> Self::Output {
        Self::Output::from_fn(|i| (&self.values[i]).double())
    }
}

impl<T: Neg<Output = T>, const N: usize> Neg for Array<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl<T, const N: usize> Neg for &Array<T, N>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = Array<T, N>;

    fn neg(self) -> Self::Output {
        Self::Output::from_fn(|i| -&self.values[i])
    }
}

impl<T: SubAssign, const N: usize> Sub for Array<T, N> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l -= r);
        lps
    }
}

impl<T: for<'a> SubAssign<&'a T>, const N: usize> Sub<&Self> for Array<T, N> {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l -= r);
        lps
    }
}

impl<T, const N: usize> Sub<Array<T, N>> for &Array<T, N>
where
    for<'a> &'a T: Sub<T, Output = T>,
{
    type Output = Array<T, N>;

    fn sub(self, rps: Array<T, N>) -> Self::Output {
        let mut values = [const { MaybeUninit::<T>::uninit() }; N];
        zip(&mut values, zip(self, rps)).for_each(|(o, (l, r))| {
            o.write(l - r);
        });
        let values = values.map(|i| unsafe { i.assume_init() });
        Array::<T, N> { values }
    }
}

impl<'a, T, const N: usize> Sub<&'a Array<T, N>> for &Array<T, N>
where
    for<'b> &'b T: Sub<Output = T>,
{
    type Output = Array<T, N>;

    fn sub(self, rps: &'a Array<T, N>) -> Self::Output {
        Self::Output::from_fn(|i| &self.values[i] - &rps.values[i])
    }
}

impl<T: SubAssign, const N: usize> SubAssign for Array<T, N> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<T: for<'a> SubAssign<&'a T>, const N: usize> SubAssign<&Self> for Array<T, N> {
    fn sub_assign(&mut self, rps: &Self) {
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<T: MulAssign, const N: usize> Mul for Array<T, N> {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l *= r);
        lps
    }
}

impl<T: for<'a> MulAssign<&'a T>, const N: usize> Mul<&Self> for Array<T, N> {
    type Output = Self;

    fn mul(self, rps: &Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l *= r);
        lps
    }
}

impl<T, const N: usize> Mul<Array<T, N>> for &Array<T, N>
where
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = Array<T, N>;

    fn mul(self, rps: Array<T, N>) -> Self::Output {
        let mut values = [const { MaybeUninit::<T>::uninit() }; N];
        zip(&mut values, zip(self, rps)).for_each(|(o, (l, r))| {
            o.write(l * r);
        });
        let values = values.map(|i| unsafe { i.assume_init() });
        Array::<T, N> { values }
    }
}

impl<'a, T, const N: usize> Mul<&'a Array<T, N>> for &Array<T, N>
where
    for<'b> &'b T: Mul<Output = T>,
{
    type Output = Array<T, N>;

    fn mul(self, rps: &'a Array<T, N>) -> Self::Output {
        Self::Output::from_fn(|i| &self.values[i] * &rps.values[i])
    }
}

impl<T: MulAssign, const N: usize> MulAssign for Array<T, N> {
    fn mul_assign(&mut self, rps: Self) {
        zip(self, rps).for_each(|(l, r)| *l *= r);
    }
}

impl<T: for<'a> MulAssign<&'a T>, const N: usize> MulAssign<&Self> for Array<T, N> {
    fn mul_assign(&mut self, rps: &Self) {
        zip(self, rps).for_each(|(l, r)| *l *= r);
    }
}

impl<T: Square<Output = T>, const N: usize> Square for Array<T, N> {
    type Output = Self;

    fn square(self) -> Self {
        self.map(Square::square)
    }
}

impl<T, const N: usize> Square for &Array<T, N>
where
    for<'a> &'a T: Square<Output = T>,
{
    type Output = Array<T, N>;

    fn square(self) -> Self::Output {
        Self::Output::from_fn(|i| (&self.values[i]).square())
    }
}

impl<T: for<'a> MulAssign<&'a T>, const N: usize> Mul<T> for Array<T, N> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: T) -> Self::Output {
        self * &rps
    }
}

impl<T: for<'a> MulAssign<&'a T>, const N: usize> Mul<&T> for Array<T, N> {
    type Output = Self;

    fn mul(self, rps: &T) -> Self::Output {
        let mut lps = self;
        lps.values.iter_mut().for_each(|l| *l *= rps);
        lps
    }
}

impl<T, const N: usize> Mul<T> for &Array<T, N>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Array<T, N>;

    #[inline]
    fn mul(self, rps: T) -> Self::Output {
        self * &rps
    }
}

impl<T, const N: usize> Mul<&T> for &Array<T, N>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Array<T, N>;

    fn mul(self, rps: &T) -> Self::Output {
        Self::Output::from_fn(|i| &self.values[i] * rps)
    }
}

impl<T: for<'a> MulAssign<&'a T>, const N: usize> MulAssign<T> for Array<T, N> {
    #[inline]
    fn mul_assign(&mut self, rps: T) {
        *self *= &rps
    }
}

impl<T: for<'a> MulAssign<&'a T>, const N: usize> MulAssign<&T> for Array<T, N> {
    fn mul_assign(&mut self, rps: &T) {
        self.values.iter_mut().for_each(|l| *l *= rps);
    }
}

impl<T: for<'a> MulAssign<&'a T> + Inv<Output = BlOption<T>>, const N: usize> Div<T>
    for Array<T, N>
{
    type Output = BlOption<Self>;

    fn div(self, rps: T) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<T: for<'a> MulAssign<&'a T>, const N: usize> Div<&T> for Array<T, N>
where
    for<'a> &'a T: Inv<Output = BlOption<T>>,
{
    type Output = BlOption<Self>;

    fn div(self, rps: &T) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<T: Inv<Output = BlOption<T>>, const N: usize> Div<T> for &Array<T, N>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = BlOption<Array<T, N>>;

    fn div(self, rps: T) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<T, const N: usize> Div<&T> for &Array<T, N>
where
    for<'a> &'a T: Mul<Output = T> + Inv<Output = BlOption<T>>,
{
    type Output = BlOption<Array<T, N>>;

    fn div(self, rps: &T) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<T: LeftZero + AddAssign, const N: usize> Sum for Array<T, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::LEFT_ZERO)
    }
}

impl<'a, T: LeftZero + for<'b> AddAssign<&'b T> + Clone, const N: usize> Sum<&'a Self>
    for Array<T, N>
{
    fn sum<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::LEFT_ZERO,
        };
        iter.fold(first, |lps, rps| lps + rps)
    }
}

impl<T: LeftOne + MulAssign, const N: usize> Product for Array<T, N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::LEFT_ONE)
    }
}

impl<'a, T: LeftOne + for<'b> MulAssign<&'b T> + Clone, const N: usize> Product<&'a Self>
    for Array<T, N>
{
    fn product<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::LEFT_ONE,
        };
        iter.fold(first, |lps, rps| lps * rps)
    }
}

impl<T: Mul<Output = T> + Sum, const N: usize> Dot for Array<T, N> {
    type Output = T;

    fn dot(self, rps: Self) -> Self::Output {
        zip(self, rps).map(|(l, r)| l * r).sum()
    }
}

impl<T: for<'a> Mul<&'a T, Output = T> + Sum, const N: usize> Dot<&Self> for Array<T, N> {
    type Output = T;

    fn dot(self, rps: &Self) -> Self::Output {
        zip(self, rps).map(|(l, r)| l * r).sum()
    }
}

impl<T: Sum, const N: usize> Dot<Array<T, N>> for &Array<T, N>
where
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = T;

    fn dot(self, rps: Array<T, N>) -> Self::Output {
        zip(self, rps).map(|(l, r)| l * r).sum()
    }
}

impl<T: Sum, const N: usize> Dot for &Array<T, N>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = T;

    fn dot(self, rps: Self) -> Self::Output {
        zip(self, rps).map(|(l, r)| l * r).sum()
    }
}

impl<T: LeftZero, const N: usize> LeftZero for Array<T, N> {
    const LEFT_ZERO: Self = Self {
        values: [T::LEFT_ZERO; N],
    };
}

impl<T: RightZero, const N: usize> RightZero for Array<T, N> {
    const RIGHT_ZERO: Self = Self {
        values: [T::RIGHT_ZERO; N],
    };
}

impl<T: Zero, const N: usize> Zero for Array<T, N> {
    const ZERO: Self = Self {
        values: [T::ZERO; N],
    };
}

impl<T: LeftOne, const N: usize> LeftOne for Array<T, N> {
    const LEFT_ONE: Self = Self {
        values: [T::LEFT_ONE; N],
    };
}

impl<T: RightOne, const N: usize> RightOne for Array<T, N> {
    const RIGHT_ONE: Self = Self {
        values: [T::RIGHT_ONE; N],
    };
}

impl<T: One, const N: usize> One for Array<T, N> {
    const ONE: Self = Self {
        values: [T::ONE; N],
    };
}

impl<T: BlAssign, const N: usize> BlAssign for Array<T, N> {
    fn bl_assign(&mut self, rps: Self, condition: bool) {
        self.values.bl_assign(rps.values, condition)
    }
}

impl<T: for<'a> BlAssign<&'a T>, const N: usize> BlAssign<&Self> for Array<T, N> {
    fn bl_assign(&mut self, rps: &Self, condition: bool) {
        self.values.bl_assign(&rps.values, condition)
    }
}

impl<T: BlSelect<Output = T>, const N: usize> BlSelect for Array<T, N> {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let values = self.values.bl_select(rps.values, condition);
        Self { values }
    }
}

impl<T: for<'a> BlSelect<&'a T, Output = T>, const N: usize> BlSelect<&Self> for Array<T, N> {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let values = self.values.bl_select(&rps.values, condition);
        Self { values }
    }
}

impl<T, const N: usize> BlSelect<Array<T, N>> for &Array<T, N>
where
    for<'a> &'a T: BlSelect<T, Output = T>,
{
    type Output = Array<T, N>;

    fn bl_select(self, rps: Array<T, N>, condition: bool) -> Self::Output {
        let values = (&self.values).bl_select(rps.values, condition);
        Self::Output { values }
    }
}

impl<T, const N: usize> BlSelect for &Array<T, N>
where
    for<'a> &'a T: BlSelect<Output = T>,
{
    type Output = Array<T, N>;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        let values = (&self.values).bl_select(&rps.values, condition);
        Self::Output { values }
    }
}

impl<T: BlEq, const N: usize> BlEq for Array<T, N> {
    fn bl_eq(&self, rps: &Self) -> bool {
        self.values.bl_eq(&rps.values)
    }

    fn bl_ne(&self, rps: &Self) -> bool {
        self.values.bl_ne(&rps.values)
    }
}

impl<Msg, T: Absorb<Msg>, const N: usize> Absorb<Msg> for Array<T, N> {
    fn absorb_into<D: Duplexer<Msg = Msg>>(self, duplex: &mut D) {
        duplex.absorb_iter(self.values)
    }
}

impl<Msg, T: Squeeze<Msg>, const N: usize> Squeeze<Msg> for Array<T, N> {
    fn squeeze_from<D: Duplexer<Msg = Msg>>(duplex: &mut D) -> Self {
        Self::from_fn(|_| duplex.squeeze())
    }
}
