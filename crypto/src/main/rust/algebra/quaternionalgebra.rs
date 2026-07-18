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
    AdditiveCommutativeMagma, AdditiveSemigroup, Algebra, CommutativeRing, Commutator, Conjugate,
    Double, FreeModule, Inv, LeftOne, LeftZero, MultiplicativeSemigroup, One, RightOne, RightZero,
    RingOps, Semimodule, Set, Square, TracelessQuaternion, UnitalAlgebra, UnitalRing, Zero,
};
use crate::branchless::BlOption;
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::mem::{MaybeUninit, transmute_copy};
use core::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Quaternion algebra of characteristic ≠ 2
///
/// with basis `{1, i, j, k}` where `i² = -1`, `j² = -1`, `k = ij = -ji`.
#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize, Zeroize)]
#[zeroize(bound = "R: Zeroize")]
pub struct QuaternionAlgebra<R: UnitalRing> {
    coefficients: FreeModule<R, 4>,
}

impl<R: UnitalRing> QuaternionAlgebra<R> {
    /// Construct a new element.
    pub const fn new(coefficients: FreeModule<R, 4>) -> Self {
        Self { coefficients }
    }

    pub const fn const_from(scalar: R) -> Self {
        let mut t = [const { MaybeUninit::<R>::uninit() }; 4];
        t[0].write(scalar);
        let mut i = 1;
        while i < 4 {
            t[i].write(R::ZERO);
            i += 1;
        }
        let t: [R; 4] = unsafe { transmute_copy(&t) };
        Self::new(FreeModule::<R, 4>::new(t))
    }

    fn reduced_norm(&self) -> R
    where
        for<'a> &'a R: RingOps<R>,
    {
        self.coefficients.iter().map(Square::square).sum::<R>()
    }
}

impl<R: UnitalRing + Debug> Debug for QuaternionAlgebra<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.coefficients)
    }
}

impl<R: UnitalRing> Default for QuaternionAlgebra<R> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: UnitalRing> From<[R; 4]> for QuaternionAlgebra<R> {
    #[inline]
    fn from(coefficients: [R; 4]) -> Self {
        Self::new(coefficients.into())
    }
}

impl<R: UnitalRing> From<FreeModule<R, 4>> for QuaternionAlgebra<R> {
    #[inline]
    fn from(coefficients: FreeModule<R, 4>) -> Self {
        Self::new(coefficients)
    }
}

impl<R: UnitalRing> From<R> for QuaternionAlgebra<R> {
    #[inline]
    fn from(scalar: R) -> Self {
        Self::const_from(scalar)
    }
}

impl<R: UnitalRing> From<QuaternionAlgebra<R>> for FreeModule<R, 4> {
    #[inline]
    fn from(element: QuaternionAlgebra<R>) -> Self {
        element.coefficients
    }
}

impl<R: UnitalRing> AsRef<FreeModule<R, 4>> for QuaternionAlgebra<R> {
    #[inline]
    fn as_ref(&self) -> &FreeModule<R, 4> {
        &self.coefficients
    }
}

impl<R: UnitalRing> AsMut<FreeModule<R, 4>> for QuaternionAlgebra<R> {
    #[inline]
    fn as_mut(&mut self) -> &mut FreeModule<R, 4> {
        &mut self.coefficients
    }
}

impl<R: UnitalRing> Borrow<[R]> for QuaternionAlgebra<R> {
    #[inline]
    fn borrow(&self) -> &[R] {
        self.coefficients.borrow()
    }
}

impl<R: UnitalRing> BorrowMut<[R]> for QuaternionAlgebra<R> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        self.coefficients.borrow_mut()
    }
}

impl<R: UnitalRing> Index<usize> for QuaternionAlgebra<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl<R: UnitalRing> IndexMut<usize> for QuaternionAlgebra<R> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coefficients[index]
    }
}

impl<R: UnitalRing> IntoIterator for QuaternionAlgebra<R> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, 4>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<'a, R: UnitalRing> IntoIterator for &'a QuaternionAlgebra<R> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&self.coefficients).into_iter()
    }
}

impl<'a, R: UnitalRing> IntoIterator for &'a mut QuaternionAlgebra<R> {
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.coefficients).into_iter()
    }
}

#[cfg(feature = "rayon")]
impl<R: UnitalRing + Send> IntoParallelIterator for QuaternionAlgebra<R> {
    type Item = R;
    type Iter = rayon::array::IntoIter<R, 4>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.coefficients.into_par_iter()
    }
}

impl<R: UnitalRing> Add for QuaternionAlgebra<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self::new(self.coefficients + rps.coefficients)
    }
}

impl<R: UnitalRing> Add<&Self> for QuaternionAlgebra<R> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self::new(self.coefficients + &rps.coefficients)
    }
}

impl<R: UnitalRing> Add<QuaternionAlgebra<R>> for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn add(self, rps: QuaternionAlgebra<R>) -> Self::Output {
        Self::Output::new(&self.coefficients + rps.coefficients)
    }
}

impl<'a, R: UnitalRing> Add<&'a QuaternionAlgebra<R>> for &QuaternionAlgebra<R>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn add(self, rps: &'a QuaternionAlgebra<R>) -> Self::Output {
        Self::Output::new(&self.coefficients + &rps.coefficients)
    }
}

impl<R: UnitalRing> AddAssign for QuaternionAlgebra<R> {
    fn add_assign(&mut self, rps: Self) {
        self.coefficients += rps.coefficients
    }
}

impl<R: UnitalRing> AddAssign<&Self> for QuaternionAlgebra<R> {
    fn add_assign(&mut self, rps: &Self) {
        self.coefficients += &rps.coefficients
    }
}

impl<R: UnitalRing> Double for QuaternionAlgebra<R> {
    type Output = Self;

    fn double(self) -> Self {
        Self::new(self.coefficients.double())
    }
}

impl<R: UnitalRing> Double for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn double(self) -> Self::Output {
        Self::Output::new((&self.coefficients).double())
    }
}

impl<R: UnitalRing> Neg for QuaternionAlgebra<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.coefficients)
    }
}

impl<R: UnitalRing> Neg for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-&self.coefficients)
    }
}

impl<R: UnitalRing> Sub for QuaternionAlgebra<R> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self::new(self.coefficients - rps.coefficients)
    }
}

impl<R: UnitalRing> Sub<&Self> for QuaternionAlgebra<R> {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        Self::new(self.coefficients - &rps.coefficients)
    }
}

impl<R: UnitalRing> Sub<QuaternionAlgebra<R>> for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn sub(self, rps: QuaternionAlgebra<R>) -> Self::Output {
        Self::Output::new(&self.coefficients - rps.coefficients)
    }
}

impl<'a, R: UnitalRing> Sub<&'a QuaternionAlgebra<R>> for &QuaternionAlgebra<R>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn sub(self, rps: &'a QuaternionAlgebra<R>) -> Self::Output {
        Self::Output::new(&self.coefficients - &rps.coefficients)
    }
}

impl<R: UnitalRing> SubAssign for QuaternionAlgebra<R> {
    fn sub_assign(&mut self, rps: Self) {
        self.coefficients -= rps.coefficients
    }
}

impl<R: UnitalRing> SubAssign<&Self> for QuaternionAlgebra<R> {
    fn sub_assign(&mut self, rps: &Self) {
        self.coefficients -= &rps.coefficients
    }
}

impl<R: UnitalRing> Mul for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: UnitalRing> Mul<&Self> for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        &self * rps
    }
}

impl<R: UnitalRing> Mul<QuaternionAlgebra<R>> for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    #[inline]
    fn mul(self, rps: QuaternionAlgebra<R>) -> Self::Output {
        self * &rps
    }
}

impl<'a, R: UnitalRing> Mul<&'a QuaternionAlgebra<R>> for &QuaternionAlgebra<R>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn mul(self, rps: &'a QuaternionAlgebra<R>) -> Self::Output {
        let mut out = Self::Output::ZERO;
        out[0] = &self[0] * &rps[0] - &self[1] * &rps[1] - &self[2] * &rps[2] - &self[3] * &rps[3];
        out[1] = &self[0] * &rps[1] + &self[1] * &rps[0] + &self[2] * &rps[3] - &self[3] * &rps[2];
        out[2] = &self[0] * &rps[2] - &self[1] * &rps[3] + &self[2] * &rps[0] + &self[3] * &rps[1];
        out[3] = &self[0] * &rps[3] + &self[1] * &rps[2] - &self[2] * &rps[1] + &self[3] * &rps[0];
        out
    }
}

impl<R: UnitalRing> MulAssign for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * rps
    }
}

impl<R: UnitalRing> MulAssign<&Self> for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = &*self * rps
    }
}

impl<R: UnitalRing + CommutativeRing> Square for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        (&self).square()
    }
}

impl<R: UnitalRing + CommutativeRing> Square for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn square(self) -> Self::Output {
        let mut out = Self::Output::ZERO;
        out[0] =
            (&self[0]).square() - (&self[1]).square() - (&self[2]).square() - (&self[3]).square();
        out[1] = (&self[0] * &self[1]).double();
        out[2] = (&self[0] * &self[2]).double();
        out[3] = (&self[0] * &self[3]).double();
        out
    }
}

impl<R: UnitalRing> Mul<R> for QuaternionAlgebra<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self::new(self.coefficients * rps)
    }
}

impl<R: UnitalRing> Mul<&R> for QuaternionAlgebra<R> {
    type Output = Self;

    fn mul(self, rps: &R) -> Self::Output {
        Self::new(self.coefficients * rps)
    }
}

impl<R: UnitalRing> Mul<R> for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn mul(self, rps: R) -> Self::Output {
        Self::Output::new(&self.coefficients * rps)
    }
}

impl<R: UnitalRing> Mul<&R> for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = QuaternionAlgebra<R>;

    fn mul(self, rps: &R) -> Self::Output {
        Self::Output::new(&self.coefficients * rps)
    }
}

impl<R: UnitalRing> MulAssign<R> for QuaternionAlgebra<R> {
    fn mul_assign(&mut self, rps: R) {
        self.coefficients *= rps
    }
}

impl<R: UnitalRing> MulAssign<&R> for QuaternionAlgebra<R> {
    fn mul_assign(&mut self, rps: &R) {
        self.coefficients *= rps
    }
}

impl<R: UnitalRing + Inv<Output = BlOption<R>>> Div<R> for QuaternionAlgebra<R> {
    type Output = BlOption<Self>;

    fn div(self, rps: R) -> Self::Output {
        (self.coefficients / rps).map(Self::new)
    }
}

impl<R: UnitalRing> Div<&R> for QuaternionAlgebra<R>
where
    for<'a> &'a R: Inv<Output = BlOption<R>>,
{
    type Output = BlOption<Self>;

    fn div(self, rps: &R) -> Self::Output {
        (self.coefficients / rps).map(Self::new)
    }
}

impl<R: UnitalRing + Inv<Output = BlOption<R>>> Inv for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = BlOption<Self>;

    fn inv(self) -> Self::Output {
        let (f, is_inv) = self.reduced_norm().inv().into();
        let mut r = self;
        r[0] = &f * &r[0];
        r[1] = &f * -&r[1];
        r[2] = &f * -&r[2];
        r[3] = f * -&r[3];
        BlOption::new(r, is_inv)
    }
}

impl<R: UnitalRing + Inv<Output = BlOption<R>>> Inv for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = BlOption<QuaternionAlgebra<R>>;

    fn inv(self) -> Self::Output {
        let (f, is_inv) = self.reduced_norm().inv().into();
        let mut r = QuaternionAlgebra::<R>::ZERO;
        r[0] = &f * &self.coefficients[0];
        r[1] = &f * -&self.coefficients[1];
        r[2] = &f * -&self.coefficients[2];
        r[3] = f * -&self.coefficients[3];
        BlOption::new(r, is_inv)
    }
}

impl<R: UnitalRing + CommutativeRing> Commutator for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    #[inline]
    fn commutator(self, rps: Self) -> Self::Output {
        (&self).commutator(&rps)
    }
}

impl<R: UnitalRing + CommutativeRing> Commutator<&Self> for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    #[inline]
    fn commutator(self, rps: &Self) -> Self::Output {
        (&self).commutator(rps)
    }
}

impl<R: UnitalRing + CommutativeRing> Commutator<QuaternionAlgebra<R>> for &QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    #[inline]
    fn commutator(self, rps: QuaternionAlgebra<R>) -> Self::Output {
        self.commutator(&rps)
    }
}

impl<'a, R: UnitalRing + CommutativeRing> Commutator<&'a QuaternionAlgebra<R>>
    for &QuaternionAlgebra<R>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = TracelessQuaternion<R>;

    fn commutator(self, rps: &'a QuaternionAlgebra<R>) -> Self::Output {
        let mut im = FreeModule::<R, 3>::ZERO;
        im[0] = (&self[2] * &rps[3] - &self[3] * &rps[2]).double();
        im[1] = (&self[3] * &rps[1] - &self[1] * &rps[3]).double();
        im[2] = (&self[1] * &rps[2] - &self[2] * &rps[1]).double();
        Self::Output::new(im)
    }
}

impl<R: UnitalRing> Sum for QuaternionAlgebra<R> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let coefficients = iter.map(|i| i.coefficients).sum();
        Self::new(coefficients)
    }
}

impl<'a, R: UnitalRing + Clone> Sum<&'a Self> for QuaternionAlgebra<R> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let coefficients = iter.map(|i| &i.coefficients).sum();
        Self::new(coefficients)
    }
}

impl<R: UnitalRing> Product for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a, R: UnitalRing + Clone> Product<&'a Self> for QuaternionAlgebra<R>
where
    for<'b> &'b R: RingOps<R>,
{
    fn product<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::ONE,
        };
        iter.fold(first, |lps, rps| lps * rps)
    }
}

impl<R: UnitalRing> LeftZero for QuaternionAlgebra<R> {
    const LEFT_ZERO: Self = Self::new(FreeModule::<R, 4>::LEFT_ZERO);
}

impl<R: UnitalRing> RightZero for QuaternionAlgebra<R> {
    const RIGHT_ZERO: Self = Self::new(FreeModule::<R, 4>::RIGHT_ZERO);
}

impl<R: UnitalRing> Zero for QuaternionAlgebra<R> {
    const ZERO: Self = Self::new(FreeModule::<R, 4>::ZERO);
}

impl<R: UnitalRing> LeftOne for QuaternionAlgebra<R> {
    const LEFT_ONE: Self = Self::const_from(R::ONE);
}

impl<R: UnitalRing> RightOne for QuaternionAlgebra<R> {
    const RIGHT_ONE: Self = Self::const_from(R::ONE);
}

impl<R: UnitalRing> One for QuaternionAlgebra<R> {
    const ONE: Self = Self::const_from(R::ONE);
}

impl<R: UnitalRing> Set for QuaternionAlgebra<R> {}

impl<R: UnitalRing> AdditiveCommutativeMagma for QuaternionAlgebra<R> {}

impl<R: UnitalRing> AdditiveSemigroup for QuaternionAlgebra<R> {}

impl<R: UnitalRing + CommutativeRing> MultiplicativeSemigroup for QuaternionAlgebra<R> where
    for<'a> &'a R: RingOps<R>
{
}

impl<R: UnitalRing + Clone> Semimodule<R> for QuaternionAlgebra<R> {}

impl<R: UnitalRing + CommutativeRing + Clone> Algebra<R> for QuaternionAlgebra<R> where
    for<'a> &'a R: RingOps<R>
{
}

impl<R: UnitalRing + CommutativeRing + Clone> UnitalAlgebra<R> for QuaternionAlgebra<R> where
    for<'a> &'a R: RingOps<R>
{
}

impl<R: UnitalRing> Conjugate for QuaternionAlgebra<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn conjugate(self) -> Self {
        let mut coefficients = self.coefficients;
        for i in 1..4 {
            coefficients[i] = -&coefficients[i];
        }
        Self::new(coefficients)
    }
}

impl<Msg, R: UnitalRing + Absorb<Msg>> Absorb<Msg> for QuaternionAlgebra<R> {
    fn absorb_into<D: Duplexer<Msg = Msg>>(self, duplex: &mut D) {
        duplex.absorb(self.coefficients)
    }
}

impl<Msg, R: UnitalRing + Squeeze<Msg>> Squeeze<Msg> for QuaternionAlgebra<R> {
    fn squeeze_from<D: Duplexer<Msg = Msg>>(duplex: &mut D) -> Self {
        duplex.squeeze::<FreeModule<R, 4>>().into()
    }
}
