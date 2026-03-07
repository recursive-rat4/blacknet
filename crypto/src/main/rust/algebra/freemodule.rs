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

use crate::algebra::{
    AdditiveCommutativeMagma, AdditiveMonoid, AdditiveSemigroup, DivisionRing, Double, LeftZero,
    RightZero, Ring, RingOps, Semimodule, Set, Zero,
};
use crate::duplex::{Absorb, Duplex, Squeeze};
use core::array;
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Sum, zip};
use core::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
#[serde(bound(
    deserialize = "[R; N]: Deserialize<'de>",
    serialize = "[R; N]: Serialize"
))]
pub struct FreeModule<R: Ring, const N: usize> {
    components: [R; N],
}

impl<R: Ring, const N: usize> FreeModule<R, N> {
    pub const fn new(components: [R; N]) -> Self {
        Self { components }
    }

    #[inline]
    pub fn from_fn<F: FnMut(usize) -> R>(f: F) -> Self {
        Self::new(array::from_fn(f))
    }

    pub const fn swap(&mut self, a: usize, b: usize) {
        self.components.swap(a, b)
    }
}

impl<R: Ring, const N: usize> Default for FreeModule<R, N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: Ring, const N: usize> From<[R; N]> for FreeModule<R, N> {
    #[inline]
    fn from(components: [R; N]) -> Self {
        Self { components }
    }
}

impl<R: Ring, const N: usize> From<FreeModule<R, N>> for [R; N] {
    #[inline]
    fn from(module: FreeModule<R, N>) -> Self {
        module.components
    }
}

impl<R: Ring + Debug, const N: usize> Debug for FreeModule<R, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.components)
    }
}

impl<R: Ring, const N: usize> AsRef<[R]> for FreeModule<R, N> {
    #[inline]
    fn as_ref(&self) -> &[R] {
        &self.components
    }
}

impl<R: Ring, const N: usize> AsRef<[R; N]> for FreeModule<R, N> {
    #[inline]
    fn as_ref(&self) -> &[R; N] {
        &self.components
    }
}

impl<R: Ring, const N: usize> AsMut<[R]> for FreeModule<R, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R] {
        &mut self.components
    }
}

impl<R: Ring, const N: usize> AsMut<[R; N]> for FreeModule<R, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R; N] {
        &mut self.components
    }
}

impl<R: Ring, const N: usize> Borrow<[R]> for FreeModule<R, N> {
    #[inline]
    fn borrow(&self) -> &[R] {
        &self.components
    }
}

impl<R: Ring, const N: usize> Borrow<[R; N]> for FreeModule<R, N> {
    #[inline]
    fn borrow(&self) -> &[R; N] {
        &self.components
    }
}

impl<R: Ring, const N: usize> BorrowMut<[R]> for FreeModule<R, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        &mut self.components
    }
}

impl<R: Ring, const N: usize> BorrowMut<[R; N]> for FreeModule<R, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R; N] {
        &mut self.components
    }
}

impl<R: Ring, const N: usize> Index<usize> for FreeModule<R, N> {
    type Output = R;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.components[i]
    }
}

impl<R: Ring, const N: usize> IndexMut<usize> for FreeModule<R, N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.components[i]
    }
}

impl<R: Ring, const N: usize> IntoIterator for FreeModule<R, N> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}

impl<'a, R: Ring, const N: usize> IntoIterator for &'a FreeModule<R, N> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.components.iter()
    }
}

impl<'a, R: Ring, const N: usize> IntoIterator for &'a mut FreeModule<R, N> {
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.components.iter_mut()
    }
}

#[cfg(feature = "rayon")]
impl<R: Ring + Send, const N: usize> IntoParallelIterator for FreeModule<R, N> {
    type Item = R;
    type Iter = rayon::array::IntoIter<R, N>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.components.into_par_iter()
    }
}

impl<R: Ring, const N: usize> Add for FreeModule<R, N> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l + r);
        out
    }
}

impl<R: Ring, const N: usize> Add<&Self> for FreeModule<R, N> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l + r);
        out
    }
}

impl<R: Ring, const N: usize> Add<FreeModule<R, N>> for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    fn add(self, rps: FreeModule<R, N>) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l + r);
        out
    }
}

impl<'a, R: Ring, const N: usize> Add<&'a FreeModule<R, N>> for &FreeModule<R, N>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    fn add(self, rps: &'a FreeModule<R, N>) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l + r);
        out
    }
}

impl<R: Ring, const N: usize> AddAssign for FreeModule<R, N> {
    fn add_assign(&mut self, rps: Self) {
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<R: Ring, const N: usize> AddAssign<&Self> for FreeModule<R, N> {
    fn add_assign(&mut self, rps: &Self) {
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<R: Ring, const N: usize> Double for FreeModule<R, N> {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            components: self.components.map(Double::double),
        }
    }
}

impl<R: Ring, const N: usize> Double for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    fn double(self) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, self).for_each(|(o, l)| *o = l.double());
        out
    }
}

impl<R: Ring, const N: usize> Neg for FreeModule<R, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            components: self.components.map(Neg::neg),
        }
    }
}

impl<R: Ring, const N: usize> Neg for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    fn neg(self) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, self).for_each(|(o, l)| *o = -l);
        out
    }
}

impl<R: Ring, const N: usize> Sub for FreeModule<R, N> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l - r);
        out
    }
}

impl<R: Ring, const N: usize> Sub<&Self> for FreeModule<R, N> {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l - r);
        out
    }
}

impl<R: Ring, const N: usize> Sub<FreeModule<R, N>> for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    fn sub(self, rps: FreeModule<R, N>) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l - r);
        out
    }
}

impl<'a, R: Ring, const N: usize> Sub<&'a FreeModule<R, N>> for &FreeModule<R, N>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    fn sub(self, rps: &'a FreeModule<R, N>) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, zip(self, rps)).for_each(|(o, (l, r))| *o = l - r);
        out
    }
}

impl<R: Ring, const N: usize> SubAssign for FreeModule<R, N> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring, const N: usize> SubAssign<&Self> for FreeModule<R, N> {
    fn sub_assign(&mut self, rps: &Self) {
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring, const N: usize> Mul<R> for FreeModule<R, N> {
    type Output = Self;

    #[allow(clippy::op_ref)]
    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Ring, const N: usize> Mul<&R> for FreeModule<R, N> {
    type Output = Self;

    fn mul(self, rps: &R) -> Self::Output {
        let mut lps = self;
        lps.components.iter_mut().for_each(|l| *l *= rps);
        lps
    }
}

impl<R: Ring, const N: usize> Mul<R> for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    #[allow(clippy::op_ref)]
    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Ring, const N: usize> Mul<&R> for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = FreeModule<R, N>;

    fn mul(self, rps: &R) -> Self::Output {
        let mut out = Self::Output::ZERO;
        zip(&mut out, self).for_each(|(o, l)| *o = l * rps);
        out
    }
}

impl<R: Ring, const N: usize> MulAssign<R> for FreeModule<R, N> {
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self *= &rps
    }
}

impl<R: Ring, const N: usize> MulAssign<&R> for FreeModule<R, N> {
    fn mul_assign(&mut self, rps: &R) {
        self.components.iter_mut().for_each(|l| *l *= rps);
    }
}

impl<R: DivisionRing, const N: usize> Div<R> for FreeModule<R, N> {
    type Output = Option<Self>;

    fn div(self, rps: R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: DivisionRing, const N: usize> Div<&R> for FreeModule<R, N> {
    type Output = Option<Self>;

    fn div(self, rps: &R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: DivisionRing, const N: usize> Div<R> for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Option<FreeModule<R, N>>;

    fn div(self, rps: R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: DivisionRing, const N: usize> Div<&R> for &FreeModule<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Option<FreeModule<R, N>>;

    fn div(self, rps: &R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: Ring, const N: usize> Sum for FreeModule<R, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a, R: Ring, const N: usize> Sum<&'a Self> for FreeModule<R, N> {
    #[allow(clippy::clone_on_copy)]
    fn sum<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::ZERO,
        };
        iter.fold(first, |lps, rps| lps + rps)
    }
}

impl<R: Ring, const N: usize> LeftZero for FreeModule<R, N> {
    const LEFT_ZERO: Self = Self {
        components: [R::LEFT_ZERO; N],
    };
}

impl<R: Ring, const N: usize> RightZero for FreeModule<R, N> {
    const RIGHT_ZERO: Self = Self {
        components: [R::RIGHT_ZERO; N],
    };
}

impl<R: Ring, const N: usize> Zero for FreeModule<R, N> {
    const ZERO: Self = Self {
        components: [R::ZERO; N],
    };
}

impl<R: Ring, const N: usize> Set for FreeModule<R, N> {}

impl<R: Ring, const N: usize> AdditiveCommutativeMagma for FreeModule<R, N> {}

impl<R: Ring, const N: usize> AdditiveSemigroup for FreeModule<R, N> {}

impl<R: Ring, const N: usize> AdditiveMonoid for FreeModule<R, N> {}

impl<R: Ring, const N: usize> Semimodule<R> for FreeModule<R, N> {}

impl<R: Ring + Absorb<R>, const N: usize> Absorb<R> for FreeModule<R, N> {
    fn absorb_into(self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb_iter(self.components.into_iter())
    }
}

impl<R: Ring + Squeeze<R>, const N: usize> Squeeze<R> for FreeModule<R, N> {
    fn squeeze_from(duplex: &mut (impl Duplex<R> + ?Sized)) -> Self {
        duplex.squeeze::<[R; N]>().into()
    }
}
