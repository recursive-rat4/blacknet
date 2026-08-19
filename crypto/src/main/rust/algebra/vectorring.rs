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
    AdditiveCommutativeMagma, AdditiveSemigroup, Array, CommutativeSemiring, Dot, Double, Inv,
    LeftOne, LeftZero, MultiplicativeCommutativeMagma, MultiplicativeSemigroup, One, RightOne,
    RightZero, Ring, RingOps, SemifieldOps, Semimodule, Semiring, SemiringOps, Set, Square,
    UnitalSemiring, Zero,
};
use crate::branchless::BlOption;
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// A ring of vectors where multiplication is defined as Hadamard product.
#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize, Zeroize)]
#[serde(bound(
    deserialize = "[R; N]: Deserialize<'de>",
    serialize = "[R; N]: Serialize"
))]
#[zeroize(bound = "R: Zeroize")]
pub struct VectorRing<R: Semiring, const N: usize> {
    elements: Array<R, N>,
}

impl<R: Semiring, const N: usize> VectorRing<R, N> {
    pub const fn new(elements: Array<R, N>) -> Self {
        Self { elements }
    }
}

impl<R: Semiring, const N: usize> Default for VectorRing<R, N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: Semiring, const N: usize> From<[R; N]> for VectorRing<R, N> {
    #[inline]
    fn from(elements: [R; N]) -> Self {
        Self::new(Array::new(elements))
    }
}

impl<R: Semiring, const N: usize> From<VectorRing<R, N>> for Array<R, N> {
    #[inline]
    fn from(vector: VectorRing<R, N>) -> Self {
        vector.elements
    }
}

impl<R: Semiring + Debug, const N: usize> Debug for VectorRing<R, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<R: Semiring, const N: usize> AsRef<[R; N]> for VectorRing<R, N> {
    #[inline]
    fn as_ref(&self) -> &[R; N] {
        &self.elements
    }
}

impl<R: Semiring, const N: usize> AsMut<[R; N]> for VectorRing<R, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R; N] {
        &mut self.elements
    }
}

impl<R: Semiring, const N: usize> Borrow<[R; N]> for VectorRing<R, N> {
    #[inline]
    fn borrow(&self) -> &[R; N] {
        &self.elements
    }
}

impl<R: Semiring, const N: usize> BorrowMut<[R; N]> for VectorRing<R, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R; N] {
        &mut self.elements
    }
}

impl<R: Semiring, const N: usize> Deref for VectorRing<R, N> {
    type Target = Array<R, N>;

    #[inline]
    fn deref(&self) -> &Array<R, N> {
        &self.elements
    }
}

impl<R: Semiring, const N: usize> DerefMut for VectorRing<R, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl<R: Semiring, const N: usize> Index<usize> for VectorRing<R, N> {
    type Output = R;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.elements[i]
    }
}

impl<R: Semiring, const N: usize> IndexMut<usize> for VectorRing<R, N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.elements[i]
    }
}

impl<R: Semiring, const N: usize> IntoIterator for VectorRing<R, N> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, R: Semiring, const N: usize> IntoIterator for &'a VectorRing<R, N> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, R: Semiring, const N: usize> IntoIterator for &'a mut VectorRing<R, N> {
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

#[cfg(feature = "rayon")]
impl<R: Semiring + Send, const N: usize> IntoParallelIterator for VectorRing<R, N> {
    type Item = R;
    type Iter = rayon::array::IntoIter<R, N>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.elements.into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, R: Semiring + Sync, const N: usize> IntoParallelIterator for &'a VectorRing<R, N> {
    type Item = &'a R;
    type Iter = rayon::slice::Iter<'a, R>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&self.elements).into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, R: Semiring + Send, const N: usize> IntoParallelIterator for &'a mut VectorRing<R, N> {
    type Item = &'a mut R;
    type Iter = rayon::slice::IterMut<'a, R>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&mut self.elements).into_par_iter()
    }
}

impl<R: Semiring, const N: usize> Add for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn add(self, rps: Self) -> Self::Output {
        Self::new(self.elements + rps.elements)
    }
}

impl<R: Semiring, const N: usize> Add<&Self> for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        Self::new(self.elements + &rps.elements)
    }
}

impl<R: Semiring, const N: usize> Add<VectorRing<R, N>> for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn add(self, rps: VectorRing<R, N>) -> Self::Output {
        Self::Output::new(&self.elements + rps.elements)
    }
}

impl<'a, R: Semiring, const N: usize> Add<&'a VectorRing<R, N>> for &VectorRing<R, N>
where
    for<'b> &'b R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn add(self, rps: &'a VectorRing<R, N>) -> Self::Output {
        Self::Output::new(&self.elements + &rps.elements)
    }
}

impl<R: Semiring, const N: usize> AddAssign for VectorRing<R, N> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        self.elements += rps.elements
    }
}

impl<R: Semiring, const N: usize> AddAssign<&Self> for VectorRing<R, N> {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        self.elements += &rps.elements
    }
}

impl<R: Semiring, const N: usize> Double for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn double(self) -> Self {
        Self::new(self.elements.double())
    }
}

impl<R: Semiring, const N: usize> Double for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn double(self) -> Self::Output {
        Self::Output::new((&self.elements).double())
    }
}

impl<R: Ring, const N: usize> Neg for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.elements)
    }
}

impl<R: Ring, const N: usize> Neg for &VectorRing<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-&self.elements)
    }
}

impl<R: Ring, const N: usize> Sub for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: Self) -> Self::Output {
        Self::new(self.elements - rps.elements)
    }
}

impl<R: Ring, const N: usize> Sub<&Self> for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        Self::new(self.elements - &rps.elements)
    }
}

impl<R: Ring, const N: usize> Sub<VectorRing<R, N>> for &VectorRing<R, N>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn sub(self, rps: VectorRing<R, N>) -> Self::Output {
        Self::Output::new(&self.elements - rps.elements)
    }
}

impl<'a, R: Ring, const N: usize> Sub<&'a VectorRing<R, N>> for &VectorRing<R, N>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn sub(self, rps: &'a VectorRing<R, N>) -> Self::Output {
        Self::Output::new(&self.elements - &rps.elements)
    }
}

impl<R: Ring, const N: usize> SubAssign for VectorRing<R, N> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        self.elements -= rps.elements
    }
}

impl<R: Ring, const N: usize> SubAssign<&Self> for VectorRing<R, N> {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        self.elements -= &rps.elements
    }
}

impl<R: Semiring, const N: usize> Mul for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        Self::new(self.elements * rps.elements)
    }
}

impl<R: Semiring, const N: usize> Mul<&Self> for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        Self::new(self.elements * &rps.elements)
    }
}

impl<R: Semiring, const N: usize> Mul<VectorRing<R, N>> for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn mul(self, rps: VectorRing<R, N>) -> Self::Output {
        Self::Output::new(&self.elements * rps.elements)
    }
}

impl<'a, R: Semiring, const N: usize> Mul<&'a VectorRing<R, N>> for &VectorRing<R, N>
where
    for<'b> &'b R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn mul(self, rps: &'a VectorRing<R, N>) -> Self::Output {
        Self::Output::new(&self.elements * &rps.elements)
    }
}

impl<R: Semiring, const N: usize> MulAssign for VectorRing<R, N> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        self.elements *= rps.elements
    }
}

impl<R: Semiring, const N: usize> MulAssign<&Self> for VectorRing<R, N> {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        self.elements *= &rps.elements
    }
}

impl<R: Semiring, const N: usize> Square for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        Self::new(self.elements.square())
    }
}

impl<R: Semiring, const N: usize> Square for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn square(self) -> Self::Output {
        Self::Output::new((&self.elements).square())
    }
}

impl<R: Semiring, const N: usize> Mul<R> for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Semiring, const N: usize> Mul<&R> for VectorRing<R, N> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &R) -> Self::Output {
        Self::new(self.elements * rps)
    }
}

impl<R: Semiring, const N: usize> Mul<R> for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Semiring, const N: usize> Mul<&R> for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn mul(self, rps: &R) -> Self::Output {
        Self::Output::new(&self.elements * rps)
    }
}

impl<R: Semiring, const N: usize> MulAssign<R> for VectorRing<R, N> {
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self *= &rps
    }
}

impl<R: Semiring, const N: usize> MulAssign<&R> for VectorRing<R, N> {
    #[inline]
    fn mul_assign(&mut self, rps: &R) {
        self.elements *= rps
    }
}

impl<R: Semiring + Inv<Output = BlOption<R>>, const N: usize> Div<R> for VectorRing<R, N> {
    type Output = BlOption<Self>;

    fn div(self, rps: R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: Semiring, const N: usize> Div<&R> for VectorRing<R, N>
where
    for<'a> &'a R: Inv<Output = BlOption<R>>,
{
    type Output = BlOption<Self>;

    fn div(self, rps: &R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: Semiring + Inv<Output = BlOption<R>>, const N: usize> Div<R> for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = BlOption<VectorRing<R, N>>;

    fn div(self, rps: R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: Semiring, const N: usize> Div<&R> for &VectorRing<R, N>
where
    for<'a> &'a R: SemifieldOps<R>,
{
    type Output = BlOption<VectorRing<R, N>>;

    fn div(self, rps: &R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<R: Semiring, const N: usize> Sum for VectorRing<R, N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|i| i.elements).sum())
    }
}

impl<'a, R: Semiring + Clone, const N: usize> Sum<&'a Self> for VectorRing<R, N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::new(iter.map(|i| &i.elements).sum())
    }
}

impl<R: UnitalSemiring, const N: usize> Product for VectorRing<R, N> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|i| i.elements).product())
    }
}

impl<'a, R: UnitalSemiring + Clone, const N: usize> Product<&'a Self> for VectorRing<R, N> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::new(iter.map(|i| &i.elements).product())
    }
}

impl<R: Semiring, const N: usize> Dot for VectorRing<R, N> {
    type Output = R;

    #[inline]
    fn dot(self, rps: Self) -> Self::Output {
        self.elements.dot(rps.elements)
    }
}

impl<R: Semiring, const N: usize> Dot<&Self> for VectorRing<R, N> {
    type Output = R;

    #[inline]
    fn dot(self, rps: &Self) -> Self::Output {
        self.elements.dot(&rps.elements)
    }
}

impl<R: Semiring, const N: usize> Dot<VectorRing<R, N>> for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = R;

    #[inline]
    fn dot(self, rps: VectorRing<R, N>) -> Self::Output {
        (&self.elements).dot(rps.elements)
    }
}

impl<R: Semiring, const N: usize> Dot for &VectorRing<R, N>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = R;

    #[inline]
    fn dot(self, rps: Self) -> Self::Output {
        (&self.elements).dot(&rps.elements)
    }
}

impl<R: Semiring, const N: usize> LeftZero for VectorRing<R, N> {
    const LEFT_ZERO: Self = Self::ZERO;
}

impl<R: Semiring, const N: usize> RightZero for VectorRing<R, N> {
    const RIGHT_ZERO: Self = Self::ZERO;
}

impl<R: Semiring, const N: usize> Zero for VectorRing<R, N> {
    const ZERO: Self = Self::new(Array::ZERO);
}

impl<R: UnitalSemiring, const N: usize> LeftOne for VectorRing<R, N> {
    const LEFT_ONE: Self = Self::ONE;
}

impl<R: UnitalSemiring, const N: usize> RightOne for VectorRing<R, N> {
    const RIGHT_ONE: Self = Self::ONE;
}

impl<R: UnitalSemiring, const N: usize> One for VectorRing<R, N> {
    const ONE: Self = Self::new(Array::ONE);
}

impl<R: Semiring, const N: usize> Set for VectorRing<R, N> {}

impl<R: Semiring, const N: usize> AdditiveCommutativeMagma for VectorRing<R, N> {}

impl<R: Semiring, const N: usize> AdditiveSemigroup for VectorRing<R, N> {}

impl<R: Semiring + CommutativeSemiring, const N: usize> MultiplicativeCommutativeMagma
    for VectorRing<R, N>
{
}

impl<R: Semiring, const N: usize> MultiplicativeSemigroup for VectorRing<R, N> {}

impl<R: Semiring + Clone, const N: usize> Semimodule<R> for VectorRing<R, N> {}

impl<Msg, R: Semiring + Absorb<Msg>, const N: usize> Absorb<Msg> for VectorRing<R, N> {
    fn absorb_into<D: Duplexer<Msg = Msg>>(self, duplex: &mut D) {
        duplex.absorb_iter(self.elements)
    }
}

impl<Msg, R: Semiring + Squeeze<Msg>, const N: usize> Squeeze<Msg> for VectorRing<R, N> {
    fn squeeze_from<D: Duplexer<Msg = Msg>>(duplex: &mut D) -> Self {
        Self::new(Array::from_fn(|_| duplex.squeeze()))
    }
}
