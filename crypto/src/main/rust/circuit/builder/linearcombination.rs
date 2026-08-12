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

use crate::algebra::{Double, RingOps, SemiringOps, Square, UnitalRing, UnitalSemiring};
use crate::circuit::builder::{
    Constant, Expression, LinearMonoid, LinearSpan, LinearTerm, Variable,
};
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Linear combination is a sum of linear terms.
#[derive(Clone, Default)]
pub struct LinearCombination<R: UnitalSemiring> {
    pub(super) terms: Vec<LinearTerm<R>>,
}

impl<R: UnitalSemiring> LinearCombination<R> {
    /// Construct an empty linear combination.
    pub const fn new() -> Self {
        Self { terms: Vec::new() }
    }

    /// Remove all terms.
    pub fn clear(&mut self) {
        self.terms.clear()
    }

    /// Construct a new linear combination given some terms.
    pub fn with_terms<const N: usize>(terms: [LinearTerm<R>; N]) -> Self {
        let mut lc = Self::new();
        terms.into_iter().for_each(|term| lc += term);
        lc
    }

    /// Empty linear combination.
    pub const ZERO: Self = Self::new();

    /// Remove terms with zero coefficient.
    pub fn reduce(&mut self)
    where
        R: Eq,
    {
        self.terms.retain(|term| term.coefficient != Constant::ZERO)
    }

    fn insert(&mut self, term: LinearTerm<R>) {
        match self
            .terms
            .binary_search_by_key(&term.variable, |term| term.variable)
        {
            Ok(idx) => self.terms[idx].coefficient += term.coefficient,
            Err(idx) => self.terms.insert(idx, term),
        }
    }

    fn merge(&self, rps: &Self) -> Self
    where
        R: Clone,
        for<'a> &'a R: SemiringOps<R>,
    {
        let mut terms = Vec::with_capacity(self.terms.len() + rps.terms.len());

        let (mut i, mut j) = (0, 0);
        while i < self.terms.len() && j < rps.terms.len() {
            let (l, r) = (&self.terms[i], &rps.terms[j]);

            match l.variable.cmp(&r.variable) {
                Ordering::Less => {
                    terms.push(l.clone());
                    i += 1;
                }
                Ordering::Greater => {
                    terms.push(r.clone());
                    j += 1;
                }
                Ordering::Equal => {
                    let coefficient = &l.coefficient + &r.coefficient;
                    let term = LinearTerm::new(l.variable, coefficient);
                    terms.push(term);
                    i += 1;
                    j += 1;
                }
            }
        }

        if i < self.terms.len() {
            terms.extend_from_slice(&self.terms[i..]);
        } else if j < rps.terms.len() {
            terms.extend_from_slice(&rps.terms[j..]);
        }

        Self { terms }
    }
}

impl<R: UnitalSemiring + Eq> Expression<R> for LinearCombination<R> {
    fn span(self) -> LinearSpan<R> {
        vec![self].into()
    }

    fn degree(&self) -> usize {
        if self
            .terms
            .iter()
            .any(|term| term.coefficient != Constant::ZERO)
        {
            1
        } else {
            0
        }
    }
}

impl<R: UnitalSemiring> From<Constant<R>> for LinearCombination<R> {
    fn from(constant: Constant<R>) -> Self {
        let term = LinearTerm::new(Variable::CONSTANT, constant);
        Self::from(term)
    }
}

impl<R: UnitalSemiring> From<Variable<R>> for LinearCombination<R> {
    fn from(variable: Variable<R>) -> Self {
        let term = LinearTerm::new(variable, Constant::ONE);
        Self::from(term)
    }
}

impl<R: UnitalSemiring> From<LinearTerm<R>> for LinearCombination<R> {
    fn from(term: LinearTerm<R>) -> Self {
        let terms = vec![term];
        Self { terms }
    }
}

impl<R: UnitalSemiring> Add<LinearTerm<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: LinearTerm<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring + Clone> Add<&LinearTerm<R>> for LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearTerm<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalSemiring> AddAssign<LinearTerm<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: LinearTerm<R>) {
        self.insert(rps)
    }
}

impl<R: UnitalSemiring + Clone> AddAssign<&LinearTerm<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &LinearTerm<R>) {
        *self += rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Add<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring> Add<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Constant<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring + Clone> Add<&Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(self, rps: &Constant<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalSemiring> AddAssign<Constant<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Constant<R>) {
        *self += LinearTerm::new(Variable::CONSTANT, rps)
    }
}

impl<R: UnitalSemiring + Clone> AddAssign<&Constant<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &Constant<R>) {
        *self += rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Add<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone> Add<&Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &Constant<R>) -> Self::Output {
        self.clone() + rps.clone()
    }
}

impl<R: UnitalSemiring> Add<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Variable<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring> AddAssign<Variable<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Variable<R>) {
        *self += LinearTerm::new(rps, Constant::ONE)
    }
}

impl<R: UnitalSemiring> AddAssign<&Variable<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &Variable<R>) {
        *self += *rps
    }
}

impl<R: UnitalSemiring + Clone> Add<Variable<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone> Add for LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        &self + &rps
    }
}

impl<R: UnitalSemiring + Clone> AddAssign for LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn add_assign(&mut self, rps: Self) {
        *self += &rps
    }
}

impl<R: UnitalSemiring + Clone> Add<&Self> for LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        &self + rps
    }
}

impl<R: UnitalSemiring + Clone> AddAssign<&Self> for LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn add_assign(&mut self, rps: &Self) {
        *self = &*self + rps
    }
}

impl<R: UnitalSemiring + Clone> Add<LinearCombination<R>> for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearCombination<R>) -> Self::Output {
        self + &rps
    }
}

impl<R: UnitalSemiring + Clone> Add for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        self.merge(rps)
    }
}

impl<R: UnitalSemiring> Double for LinearCombination<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            terms: self.terms.into_iter().map(Double::double).collect(),
        }
    }
}

impl<R: UnitalSemiring> Double for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn double(self) -> Self::Output {
        LinearCombination::<R> {
            terms: self.terms.iter().map(Double::double).collect(),
        }
    }
}

impl<R: UnitalRing> Neg for LinearCombination<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        LinearCombination::<R> {
            terms: self.terms.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: UnitalRing> Neg for &LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn neg(self) -> Self::Output {
        LinearCombination::<R> {
            terms: self.terms.iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: UnitalRing> Sub<LinearTerm<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: LinearTerm<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign<LinearTerm<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: LinearTerm<R>) {
        self.insert(-rps)
    }
}

impl<R: UnitalRing> SubAssign<&LinearTerm<R>> for LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    fn sub_assign(&mut self, rps: &LinearTerm<R>) {
        self.insert(-rps)
    }
}

impl<R: UnitalRing + Clone> Sub<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Constant<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing + Clone> Sub<&Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(self, rps: &Constant<R>) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: UnitalRing> SubAssign<Constant<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Constant<R>) {
        *self -= LinearTerm::new(Variable::CONSTANT, rps)
    }
}

impl<R: UnitalRing + Clone> SubAssign<&Constant<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: &Constant<R>) {
        *self -= rps.clone()
    }
}

impl<R: UnitalRing + Clone> Sub<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone> Sub<&Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &Constant<R>) -> Self::Output {
        self.clone() - rps.clone()
    }
}

impl<R: UnitalRing> Sub<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Variable<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign<Variable<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Variable<R>) {
        *self -= LinearTerm::new(rps, Constant::ONE)
    }
}

impl<R: UnitalRing + Clone> Sub<Variable<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone> Sub for LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        self.merge(&-rps)
    }
}

impl<R: UnitalRing + Clone> SubAssign for LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    fn sub_assign(&mut self, rps: Self) {
        *self = &*self - rps
    }
}

impl<R: UnitalRing + Clone> Sub<&Self> for LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        self.merge(&-rps)
    }
}

impl<R: UnitalRing + Clone> SubAssign<&Self> for LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    fn sub_assign(&mut self, rps: &Self) {
        *self = &*self - rps
    }
}

impl<R: UnitalRing + Clone> Sub<LinearCombination<R>> for &LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearCombination<R>) -> Self::Output {
        self.merge(&-rps)
    }
}

impl<R: UnitalRing + Clone> Sub for &LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        self.merge(&-rps)
    }
}

impl<R: UnitalSemiring> Mul<LinearTerm<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [self, rps.into()].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [self.clone(), rps.into()].into()
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn mul(mut self, rps: Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalSemiring> Mul<&Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn mul(mut self, rps: &Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalSemiring> MulAssign<Constant<R>> for LinearCombination<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        *self *= &rps
    }
}

impl<R: UnitalSemiring> MulAssign<&Constant<R>> for LinearCombination<R> {
    fn mul_assign(&mut self, rps: &Constant<R>) {
        for term in self.terms.iter_mut() {
            *term *= rps
        }
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        self * &rps
    }
}

impl<R: UnitalSemiring> Mul<&Constant<R>> for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: &Constant<R>) -> Self::Output {
        LinearCombination::<R> {
            terms: self.terms.iter().map(|term| term * rps).collect(),
        }
    }
}

impl<R: UnitalSemiring> Mul<Variable<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [self, LinearTerm::new(rps, Constant::ONE).into()].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<Variable<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring> Mul for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self, rps].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&Self> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &Self) -> Self::Output {
        [self, rps.clone()].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [self.clone(), rps].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self.clone(), rps.clone()].into()
    }
}

impl<R: UnitalSemiring + Clone> Square for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.clone(), self].into()
    }
}

impl<R: UnitalSemiring + Clone> Square for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.clone(), self.clone()].into()
    }
}

impl<R: UnitalSemiring> Mul<LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self);
        rps
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.clone());
        rps
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self.clone() * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Sum<LinearTerm<R>> for LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn sum<I: Iterator<Item = LinearTerm<R>>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => LinearCombination::from(i),
            None => return Self::ZERO,
        };
        iter.fold(first, |lps, rps| lps + rps)
    }
}

impl<'a, R: UnitalSemiring + Clone> Sum<&'a LinearTerm<R>> for LinearCombination<R>
where
    for<'b> &'b R: SemiringOps<R>,
{
    fn sum<I: Iterator<Item = &'a LinearTerm<R>>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => LinearCombination::from(i.clone()),
            None => return Self::ZERO,
        };
        iter.fold(first, |lps, rps| lps + rps)
    }
}

impl<R: UnitalSemiring + Clone> Sum<Variable<R>> for LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn sum<I: Iterator<Item = Variable<R>>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => LinearCombination::from(i),
            None => return Self::ZERO,
        };
        iter.fold(first, |lps, rps| lps + rps)
    }
}

impl<'a, R: UnitalSemiring + Clone> Sum<&'a Variable<R>> for LinearCombination<R>
where
    for<'b> &'b R: SemiringOps<R>,
{
    fn sum<I: Iterator<Item = &'a Variable<R>>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<R: UnitalSemiring + Clone> Sum for LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a, R: UnitalSemiring + Clone> Sum<&'a Self> for LinearCombination<R>
where
    for<'b> &'b R: SemiringOps<R>,
{
    fn sum<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::ZERO,
        };
        iter.fold(first, |lps, rps| lps + rps)
    }
}

impl<R: UnitalSemiring> Absorb<Self> for LinearCombination<R> {
    fn absorb_into<D: Duplexer<Msg = Self>>(self, duplex: &mut D) {
        duplex.absorb_msg(self)
    }
}

impl<R: UnitalSemiring> Squeeze<Self> for LinearCombination<R> {
    fn squeeze_from<D: Duplexer<Msg = Self>>(duplex: &mut D) -> Self {
        duplex.squeeze_msg()
    }
}
