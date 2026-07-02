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

use crate::algebra::{
    AdditiveCommutativeMagma, AdditiveSemigroup, Conjugate, Double, FreeModule, Inv, LeftZero,
    QuaternionAlgebra, RightZero, RingOps, Semimodule, Set, Square, UnitalRing, Zero,
};
use crate::branchless::BlOption;
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Sum, zip};
use core::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// A submodule of quaternion algebra
///
/// with basis `{i, j, k}` and `Tr(x) = 0`.
#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize, Zeroize)]
#[zeroize(bound = "R: Zeroize")]
pub struct TracelessQuaternion<R: UnitalRing> {
    coefficients: FreeModule<R, 3>,
}

impl<R: UnitalRing> TracelessQuaternion<R> {
    /// Construct a new element.
    pub const fn new(coefficients: FreeModule<R, 3>) -> Self {
        Self { coefficients }
    }

    fn reduced_norm(&self) -> R
    where
        for<'a> &'a R: RingOps<R>,
    {
        self.coefficients.iter().map(Square::square).sum::<R>()
    }
}

impl<R: UnitalRing + Debug> Debug for TracelessQuaternion<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.coefficients)
    }
}

impl<R: UnitalRing> Default for TracelessQuaternion<R> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: UnitalRing> From<[R; 3]> for TracelessQuaternion<R> {
    #[inline]
    fn from(coefficients: [R; 3]) -> Self {
        Self::new(coefficients.into())
    }
}

impl<R: UnitalRing> From<FreeModule<R, 3>> for TracelessQuaternion<R> {
    #[inline]
    fn from(coefficients: FreeModule<R, 3>) -> Self {
        Self::new(coefficients)
    }
}

impl<R: UnitalRing> From<TracelessQuaternion<R>> for FreeModule<R, 3> {
    #[inline]
    fn from(element: TracelessQuaternion<R>) -> Self {
        element.coefficients
    }
}

impl<R: UnitalRing> From<TracelessQuaternion<R>> for QuaternionAlgebra<R> {
    #[inline]
    fn from(element: TracelessQuaternion<R>) -> Self {
        let mut components = [R::ZERO; 4];
        for (l, r) in zip(&mut components[1..], element.coefficients) {
            *l = r
        }
        components.into()
    }
}

impl<R: UnitalRing> AsRef<FreeModule<R, 3>> for TracelessQuaternion<R> {
    #[inline]
    fn as_ref(&self) -> &FreeModule<R, 3> {
        &self.coefficients
    }
}

impl<R: UnitalRing> AsMut<FreeModule<R, 3>> for TracelessQuaternion<R> {
    #[inline]
    fn as_mut(&mut self) -> &mut FreeModule<R, 3> {
        &mut self.coefficients
    }
}

impl<R: UnitalRing> Borrow<[R]> for TracelessQuaternion<R> {
    #[inline]
    fn borrow(&self) -> &[R] {
        self.coefficients.borrow()
    }
}

impl<R: UnitalRing> BorrowMut<[R]> for TracelessQuaternion<R> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        self.coefficients.borrow_mut()
    }
}

impl<R: UnitalRing> Index<usize> for TracelessQuaternion<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl<R: UnitalRing> IndexMut<usize> for TracelessQuaternion<R> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coefficients[index]
    }
}

impl<R: UnitalRing> IntoIterator for TracelessQuaternion<R> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, 3>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<'a, R: UnitalRing> IntoIterator for &'a TracelessQuaternion<R> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&self.coefficients).into_iter()
    }
}

impl<'a, R: UnitalRing> IntoIterator for &'a mut TracelessQuaternion<R> {
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.coefficients).into_iter()
    }
}

#[cfg(feature = "rayon")]
impl<R: UnitalRing + Send> IntoParallelIterator for TracelessQuaternion<R> {
    type Item = R;
    type Iter = rayon::array::IntoIter<R, 3>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.coefficients.into_par_iter()
    }
}

impl<R: UnitalRing> Add for TracelessQuaternion<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self::new(self.coefficients + rps.coefficients)
    }
}

impl<R: UnitalRing> Add<&Self> for TracelessQuaternion<R> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self::new(self.coefficients + &rps.coefficients)
    }
}

impl<R: UnitalRing> Add<TracelessQuaternion<R>> for &TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn add(self, rps: TracelessQuaternion<R>) -> Self::Output {
        Self::Output::new(&self.coefficients + rps.coefficients)
    }
}

impl<'a, R: UnitalRing> Add<&'a TracelessQuaternion<R>> for &TracelessQuaternion<R>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn add(self, rps: &'a TracelessQuaternion<R>) -> Self::Output {
        Self::Output::new(&self.coefficients + &rps.coefficients)
    }
}

impl<R: UnitalRing> AddAssign for TracelessQuaternion<R> {
    fn add_assign(&mut self, rps: Self) {
        self.coefficients += rps.coefficients
    }
}

impl<R: UnitalRing> AddAssign<&Self> for TracelessQuaternion<R> {
    fn add_assign(&mut self, rps: &Self) {
        self.coefficients += &rps.coefficients
    }
}

impl<R: UnitalRing> Double for TracelessQuaternion<R> {
    type Output = Self;

    fn double(self) -> Self {
        Self::new(self.coefficients.double())
    }
}

impl<R: UnitalRing> Double for &TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn double(self) -> Self::Output {
        Self::Output::new((&self.coefficients).double())
    }
}

impl<R: UnitalRing> Neg for TracelessQuaternion<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.coefficients)
    }
}

impl<R: UnitalRing> Neg for &TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-&self.coefficients)
    }
}

impl<R: UnitalRing> Sub for TracelessQuaternion<R> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self::new(self.coefficients - rps.coefficients)
    }
}

impl<R: UnitalRing> Sub<&Self> for TracelessQuaternion<R> {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        Self::new(self.coefficients - &rps.coefficients)
    }
}

impl<R: UnitalRing> Sub<TracelessQuaternion<R>> for &TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn sub(self, rps: TracelessQuaternion<R>) -> Self::Output {
        Self::Output::new(&self.coefficients - rps.coefficients)
    }
}

impl<'a, R: UnitalRing> Sub<&'a TracelessQuaternion<R>> for &TracelessQuaternion<R>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn sub(self, rps: &'a TracelessQuaternion<R>) -> Self::Output {
        Self::Output::new(&self.coefficients - &rps.coefficients)
    }
}

impl<R: UnitalRing> SubAssign for TracelessQuaternion<R> {
    fn sub_assign(&mut self, rps: Self) {
        self.coefficients -= rps.coefficients
    }
}

impl<R: UnitalRing> SubAssign<&Self> for TracelessQuaternion<R> {
    fn sub_assign(&mut self, rps: &Self) {
        self.coefficients -= &rps.coefficients
    }
}

impl<R: UnitalRing> Mul<R> for TracelessQuaternion<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self::new(self.coefficients * rps)
    }
}

impl<R: UnitalRing> Mul<&R> for TracelessQuaternion<R> {
    type Output = Self;

    fn mul(self, rps: &R) -> Self::Output {
        Self::new(self.coefficients * rps)
    }
}

impl<R: UnitalRing> Mul<R> for &TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn mul(self, rps: R) -> Self::Output {
        Self::Output::new(&self.coefficients * rps)
    }
}

impl<R: UnitalRing> Mul<&R> for &TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn mul(self, rps: &R) -> Self::Output {
        Self::Output::new(&self.coefficients * rps)
    }
}

impl<R: UnitalRing> MulAssign<R> for TracelessQuaternion<R> {
    fn mul_assign(&mut self, rps: R) {
        self.coefficients *= rps
    }
}

impl<R: UnitalRing> MulAssign<&R> for TracelessQuaternion<R> {
    fn mul_assign(&mut self, rps: &R) {
        self.coefficients *= rps
    }
}

impl<R: UnitalRing + Inv<Output = BlOption<R>>> Div<R> for TracelessQuaternion<R> {
    type Output = BlOption<Self>;

    fn div(self, rps: R) -> Self::Output {
        (self.coefficients / rps).map(Self::new)
    }
}

impl<R: UnitalRing> Div<&R> for TracelessQuaternion<R>
where
    for<'a> &'a R: Inv<Output = BlOption<R>>,
{
    type Output = BlOption<Self>;

    fn div(self, rps: &R) -> Self::Output {
        (self.coefficients / rps).map(Self::new)
    }
}

impl<R: UnitalRing + Inv<Output = BlOption<R>>> Inv for TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = BlOption<Self>;

    fn inv(self) -> Self::Output {
        let (f, is_inv) = self.reduced_norm().inv().into();
        let mut r = self;
        r[0] = &f * -&r[0];
        r[1] = &f * -&r[1];
        r[2] = f * -&r[2];
        BlOption::new(r, is_inv)
    }
}

impl<R: UnitalRing + Inv<Output = BlOption<R>>> Inv for &TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = BlOption<TracelessQuaternion<R>>;

    fn inv(self) -> Self::Output {
        let (f, is_inv) = self.reduced_norm().inv().into();
        let mut r = TracelessQuaternion::<R>::ZERO;
        r[0] = &f * -&self.coefficients[0];
        r[1] = &f * -&self.coefficients[1];
        r[2] = f * -&self.coefficients[2];
        BlOption::new(r, is_inv)
    }
}

impl<R: UnitalRing> Sum for TracelessQuaternion<R> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let coefficients = iter.map(|i| i.coefficients).sum();
        Self::new(coefficients)
    }
}

impl<'a, R: UnitalRing + Clone> Sum<&'a Self> for TracelessQuaternion<R> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let coefficients = iter.map(|i| &i.coefficients).sum();
        Self::new(coefficients)
    }
}

impl<R: UnitalRing> LeftZero for TracelessQuaternion<R> {
    const LEFT_ZERO: Self = Self::new(FreeModule::<R, 3>::LEFT_ZERO);
}

impl<R: UnitalRing> RightZero for TracelessQuaternion<R> {
    const RIGHT_ZERO: Self = Self::new(FreeModule::<R, 3>::RIGHT_ZERO);
}

impl<R: UnitalRing> Zero for TracelessQuaternion<R> {
    const ZERO: Self = Self::new(FreeModule::<R, 3>::ZERO);
}

impl<R: UnitalRing> Set for TracelessQuaternion<R> {}

impl<R: UnitalRing> AdditiveCommutativeMagma for TracelessQuaternion<R> {}

impl<R: UnitalRing> AdditiveSemigroup for TracelessQuaternion<R> {}

impl<R: UnitalRing + Clone> Semimodule<R> for TracelessQuaternion<R> {}

impl<R: UnitalRing> Conjugate for TracelessQuaternion<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn conjugate(self) -> Self {
        let mut coefficients = self.coefficients;
        for i in 0..3 {
            coefficients[i] = -&coefficients[i];
        }
        Self::new(coefficients)
    }
}

impl<R: UnitalRing + Absorb<R>> Absorb<R> for TracelessQuaternion<R> {
    fn absorb_into<D: Duplexer<Msg = R>>(self, duplex: &mut D) {
        duplex.absorb(self.coefficients)
    }
}

impl<R: UnitalRing + Squeeze<R>> Squeeze<R> for TracelessQuaternion<R> {
    fn squeeze_from<D: Duplexer<Msg = R>>(duplex: &mut D) -> Self {
        duplex.squeeze::<FreeModule<R, 3>>().into()
    }
}
