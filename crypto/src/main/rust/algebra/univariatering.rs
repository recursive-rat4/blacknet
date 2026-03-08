/*
 * Copyright (c) 2024-2026 Pavel Vasin
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
    AdditiveCommutativeMagma, AdditiveMonoid, AdditiveSemigroup, Algebra, CommutativeRing,
    Conjugate, DivisionRing, DivisionRingOps, Double, FreeModule, IntegerRing, LeftOne, LeftZero,
    MultiplicativeCommutativeMagma, MultiplicativeMonoid, MultiplicativeSemigroup, One,
    PolynomialRing, PowerOfTwoCyclotomicRing, RightOne, RightZero, RingOps, Semimodule, Set,
    Square, UnitalAlgebra, UnitalRing, Zero,
};
use crate::convolution::{Convolution, Negacyclic};
use crate::duplex::{Absorb, Duplex, Squeeze};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::marker::PhantomData;
use core::mem::{MaybeUninit, transmute_copy};
use core::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};

// Univariate polynomial ring in monomial basis

#[derive(Deserialize, Serialize)]
#[serde(bound(
    deserialize = "FreeModule<R, N>: Deserialize<'de>",
    serialize = "FreeModule<R, N>: Serialize"
))]
pub struct UnivariateRing<R: UnitalRing, const N: usize, C: Convolution<R, N>> {
    coefficients: FreeModule<R, N>,
    #[serde(skip)]
    phantom: PhantomData<C>,
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> UnivariateRing<R, N, C> {
    pub const fn new(coefficients: FreeModule<R, N>) -> Self {
        const {
            assert!(N.is_power_of_two(), "Not implemented");
        };
        Self {
            coefficients,
            phantom: PhantomData,
        }
    }

    pub const fn const_from(scalar: R) -> Self {
        let mut t = [const { MaybeUninit::<R>::uninit() }; N];
        t[0].write(scalar);
        let mut i = 1;
        while i < N {
            t[i].write(R::ZERO);
            i += 1;
        }
        let t: [R; N] = unsafe { transmute_copy(&t) };
        Self::new(FreeModule::<R, N>::new(t))
    }
}

impl<R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> Clone
    for UnivariateRing<R, N, C>
{
    fn clone(&self) -> Self {
        Self::new(self.coefficients.clone())
    }
}

impl<R: UnitalRing + Copy, const N: usize, C: Convolution<R, N>> Copy for UnivariateRing<R, N, C> {}

impl<R: UnitalRing + Debug, const N: usize, C: Convolution<R, N>> Debug
    for UnivariateRing<R, N, C>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Default for UnivariateRing<R, N, C> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: UnitalRing + PartialEq, const N: usize, C: Convolution<R, N>> PartialEq
    for UnivariateRing<R, N, C>
{
    fn eq(&self, rps: &Self) -> bool {
        self.coefficients == rps.coefficients
    }
}

impl<R: UnitalRing + Eq, const N: usize, C: Convolution<R, N>> Eq for UnivariateRing<R, N, C> {}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> From<[R; N]> for UnivariateRing<R, N, C> {
    #[inline]
    fn from(coefficients: [R; N]) -> Self {
        Self::new(coefficients.into())
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> From<FreeModule<R, N>>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn from(coefficients: FreeModule<R, N>) -> Self {
        Self::new(coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> From<R> for UnivariateRing<R, N, C> {
    #[inline]
    fn from(scalar: R) -> Self {
        Self::const_from(scalar)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> From<UnivariateRing<R, N, C>>
    for FreeModule<R, N>
{
    #[inline]
    fn from(polynomial: UnivariateRing<R, N, C>) -> Self {
        polynomial.coefficients
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AsRef<FreeModule<R, N>>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn as_ref(&self) -> &FreeModule<R, N> {
        &self.coefficients
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AsMut<FreeModule<R, N>>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn as_mut(&mut self) -> &mut FreeModule<R, N> {
        &mut self.coefficients
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Borrow<[R]> for UnivariateRing<R, N, C> {
    #[inline]
    fn borrow(&self) -> &[R] {
        self.coefficients.borrow()
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> BorrowMut<[R]>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        self.coefficients.borrow_mut()
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Index<usize> for UnivariateRing<R, N, C> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> IndexMut<usize>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coefficients[index]
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> IntoIterator for UnivariateRing<R, N, C> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<'a, R: UnitalRing, const N: usize, C: Convolution<R, N>> IntoIterator
    for &'a UnivariateRing<R, N, C>
{
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&self.coefficients).into_iter()
    }
}

impl<'a, R: UnitalRing, const N: usize, C: Convolution<R, N>> IntoIterator
    for &'a mut UnivariateRing<R, N, C>
{
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.coefficients).into_iter()
    }
}

#[cfg(feature = "rayon")]
impl<R: UnitalRing + Send, const N: usize, C: Convolution<R, N>> IntoParallelIterator
    for UnivariateRing<R, N, C>
{
    type Item = R;
    type Iter = rayon::array::IntoIter<R, N>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.coefficients.into_par_iter()
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Add for UnivariateRing<R, N, C> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self::new(self.coefficients + rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Add<&Self> for UnivariateRing<R, N, C> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self::new(self.coefficients + &rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Add<UnivariateRing<R, N, C>>
    for &UnivariateRing<R, N, C>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn add(self, rps: UnivariateRing<R, N, C>) -> Self::Output {
        Self::Output::new(&self.coefficients + rps.coefficients)
    }
}

impl<'a, R: UnitalRing, const N: usize, C: Convolution<R, N>> Add<&'a UnivariateRing<R, N, C>>
    for &UnivariateRing<R, N, C>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn add(self, rps: &'a UnivariateRing<R, N, C>) -> Self::Output {
        Self::Output::new(&self.coefficients + &rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AddAssign for UnivariateRing<R, N, C> {
    fn add_assign(&mut self, rps: Self) {
        self.coefficients += rps.coefficients
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AddAssign<&Self>
    for UnivariateRing<R, N, C>
{
    fn add_assign(&mut self, rps: &Self) {
        self.coefficients += &rps.coefficients
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Double for UnivariateRing<R, N, C> {
    type Output = Self;

    fn double(self) -> Self {
        Self::new(self.coefficients.double())
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Double for &UnivariateRing<R, N, C>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn double(self) -> Self::Output {
        Self::Output::new((&self.coefficients).double())
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Neg for UnivariateRing<R, N, C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Neg for &UnivariateRing<R, N, C>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-&self.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Sub for UnivariateRing<R, N, C> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self::new(self.coefficients - rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Sub<&Self> for UnivariateRing<R, N, C> {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        Self::new(self.coefficients - &rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Sub<UnivariateRing<R, N, C>>
    for &UnivariateRing<R, N, C>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn sub(self, rps: UnivariateRing<R, N, C>) -> Self::Output {
        Self::Output::new(&self.coefficients - rps.coefficients)
    }
}

impl<'a, R: UnitalRing, const N: usize, C: Convolution<R, N>> Sub<&'a UnivariateRing<R, N, C>>
    for &UnivariateRing<R, N, C>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn sub(self, rps: &'a UnivariateRing<R, N, C>) -> Self::Output {
        Self::Output::new(&self.coefficients - &rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> SubAssign for UnivariateRing<R, N, C> {
    fn sub_assign(&mut self, rps: Self) {
        self.coefficients -= rps.coefficients
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> SubAssign<&Self>
    for UnivariateRing<R, N, C>
{
    fn sub_assign(&mut self, rps: &Self) {
        self.coefficients -= &rps.coefficients
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul for UnivariateRing<R, N, C> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<&Self> for UnivariateRing<R, N, C> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        &self * rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<UnivariateRing<R, N, C>>
    for &UnivariateRing<R, N, C>
{
    type Output = UnivariateRing<R, N, C>;

    #[inline]
    fn mul(self, rps: UnivariateRing<R, N, C>) -> Self::Output {
        self * &rps
    }
}

impl<'a, R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<&'a UnivariateRing<R, N, C>>
    for &UnivariateRing<R, N, C>
{
    type Output = UnivariateRing<R, N, C>;

    fn mul(self, rps: &'a UnivariateRing<R, N, C>) -> Self::Output {
        let sequence = C::convolute(&self.coefficients, &rps.coefficients);
        let coefficients = FreeModule::<R, N>::new(sequence);
        Self::Output::new(coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MulAssign for UnivariateRing<R, N, C> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MulAssign<&Self>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = &*self * rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Square for UnivariateRing<R, N, C> {
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        &self * &self
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Square for &UnivariateRing<R, N, C> {
    type Output = UnivariateRing<R, N, C>;

    #[inline]
    fn square(self) -> Self::Output {
        self * self
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<R> for UnivariateRing<R, N, C> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self::new(self.coefficients * rps)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<&R> for UnivariateRing<R, N, C> {
    type Output = Self;

    fn mul(self, rps: &R) -> Self::Output {
        Self::new(self.coefficients * rps)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<R> for &UnivariateRing<R, N, C>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn mul(self, rps: R) -> Self::Output {
        Self::Output::new(&self.coefficients * rps)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<&R> for &UnivariateRing<R, N, C>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = UnivariateRing<R, N, C>;

    fn mul(self, rps: &R) -> Self::Output {
        Self::Output::new(&self.coefficients * rps)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MulAssign<R> for UnivariateRing<R, N, C> {
    fn mul_assign(&mut self, rps: R) {
        self.coefficients *= rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MulAssign<&R>
    for UnivariateRing<R, N, C>
{
    fn mul_assign(&mut self, rps: &R) {
        self.coefficients *= rps
    }
}

impl<R: UnitalRing + DivisionRing, const N: usize, C: Convolution<R, N>> Div<R>
    for UnivariateRing<R, N, C>
{
    type Output = Option<Self>;

    fn div(self, rps: R) -> Self::Output {
        (self.coefficients / rps).map(Self::new)
    }
}

impl<R: UnitalRing + DivisionRing, const N: usize, C: Convolution<R, N>> Div<&R>
    for UnivariateRing<R, N, C>
where
    for<'a> &'a R: DivisionRingOps<R>,
{
    type Output = Option<Self>;

    fn div(self, rps: &R) -> Self::Output {
        (self.coefficients / rps).map(Self::new)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Sum for UnivariateRing<R, N, C> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let coefficients = iter.map(|i| i.coefficients).sum();
        Self::new(coefficients)
    }
}

impl<'a, R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> Sum<&'a Self>
    for UnivariateRing<R, N, C>
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let coefficients = iter.map(|i| &i.coefficients).sum();
        Self::new(coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Product for UnivariateRing<R, N, C> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a, R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> Product<&'a Self>
    for UnivariateRing<R, N, C>
{
    fn product<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::ONE,
        };
        iter.fold(first, |lps, rps| lps * rps)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> LeftZero for UnivariateRing<R, N, C> {
    const LEFT_ZERO: Self = Self::new(FreeModule::<R, N>::LEFT_ZERO);
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> RightZero for UnivariateRing<R, N, C> {
    const RIGHT_ZERO: Self = Self::new(FreeModule::<R, N>::RIGHT_ZERO);
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Zero for UnivariateRing<R, N, C> {
    const ZERO: Self = Self::new(FreeModule::<R, N>::ZERO);
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> LeftOne for UnivariateRing<R, N, C> {
    const LEFT_ONE: Self = Self::const_from(R::ONE);
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> RightOne for UnivariateRing<R, N, C> {
    const RIGHT_ONE: Self = Self::const_from(R::ONE);
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> One for UnivariateRing<R, N, C> {
    const ONE: Self = Self::const_from(R::ONE);
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Set for UnivariateRing<R, N, C> {}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AdditiveCommutativeMagma
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AdditiveSemigroup
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> AdditiveMonoid
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing + CommutativeRing, const N: usize, C: Convolution<R, N>>
    MultiplicativeCommutativeMagma for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MultiplicativeSemigroup
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> MultiplicativeMonoid
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> Semimodule<R>
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> Algebra<R>
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> UnitalAlgebra<R>
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing + Clone, const N: usize, C: Convolution<R, N>> PolynomialRing<R>
    for UnivariateRing<R, N, C>
where
    for<'a> &'a R: RingOps<R>,
{
    #[inline]
    fn constant_term(self) -> R {
        self.coefficients.into_iter().next().unwrap_or(R::ZERO)
    }

    fn evaluate(&self, point: &R) -> R {
        let mut sigma = self.coefficients[0].clone();
        let mut power = point.clone();
        for i in 1..N - 1 {
            sigma += &self.coefficients[i] * &power;
            power *= point;
        }
        if N > 1 {
            sigma += &self.coefficients[N - 1] * power;
        }
        sigma
    }
}

impl<R: IntegerRing, const N: usize> Conjugate for UnivariateRing<R, N, Negacyclic>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn conjugate(self) -> Self {
        let mut coefficients = self.coefficients;
        for i in 1..N / 2 {
            let a = -&coefficients[i];
            let b = -&coefficients[N - i];
            coefficients[N - i] = a;
            coefficients[i] = b;
        }
        coefficients[N / 2] = -&coefficients[N / 2];
        coefficients.into()
    }
}

impl<R: IntegerRing + Clone, const N: usize> PowerOfTwoCyclotomicRing<R>
    for UnivariateRing<R, N, Negacyclic>
where
    for<'a> &'a R: RingOps<R>,
{
}

impl<R: UnitalRing + Absorb<R>, const N: usize, C: Convolution<R, N>> Absorb<R>
    for UnivariateRing<R, N, C>
{
    fn absorb_into(self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb(self.coefficients)
    }
}

impl<R: UnitalRing + Squeeze<R>, const N: usize, C: Convolution<R, N>> Squeeze<R>
    for UnivariateRing<R, N, C>
{
    fn squeeze_from(duplex: &mut (impl Duplex<R> + ?Sized)) -> Self {
        duplex.squeeze::<FreeModule<R, N>>().into()
    }
}
