/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

use crate::matrixdense::MatrixDense;
use crate::ring::Ring;
use core::fmt::{Debug, Formatter, Result};
use core::iter::zip;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Eq, PartialEq)]
pub struct VectorDense<R: Ring> {
    elements: Vec<R>,
}

impl<R: Ring> VectorDense<R> {
    pub fn fill(size: usize, element: R) -> Self {
        Self {
            elements: vec![element; size],
        }
    }

    pub fn identity(size: usize) -> Self {
        Self {
            elements: vec![R::UNITY; size],
        }
    }

    pub const fn dimension(&self) -> usize {
        self.elements.len()
    }

    pub const fn elements(&self) -> &Vec<R> {
        &self.elements
    }

    pub fn steal(self) -> Vec<R> {
        self.elements
    }

    pub fn cat(&self, rps: &Self) -> Self {
        let mut elements = Vec::<R>::with_capacity(self.elements.len() + rps.elements.len());
        elements.extend(self.elements.iter());
        elements.extend(rps.elements.iter());
        elements.into()
    }

    pub fn dot(&self, rps: &Self) -> R {
        zip(self.elements.iter(), rps.elements.iter())
            .map(|(&l, &r)| l * r)
            .sum()
    }

    pub fn tensor(&self, rps: &Self) -> MatrixDense<R> {
        let rows = self.elements.len();
        let columns = rps.elements.len();
        let mut elements = Vec::<R>::with_capacity(rows * columns);
        for i in 0..rows {
            for j in 0..columns {
                elements.push(self.elements[i] * rps.elements[j])
            }
        }
        MatrixDense::new(rows, columns, elements)
    }
}

impl<R: Ring, const N: usize> From<[R; N]> for VectorDense<R> {
    fn from(elements: [R; N]) -> Self {
        Self {
            elements: elements.into(),
        }
    }
}

impl<R: Ring> From<Vec<R>> for VectorDense<R> {
    fn from(elements: Vec<R>) -> Self {
        Self { elements }
    }
}

impl<R: Ring> Debug for VectorDense<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<R: Ring> Index<usize> for VectorDense<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<R: Ring> IndexMut<usize> for VectorDense<R> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<R: Ring> Add for VectorDense<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        zip(self.elements, rps.elements)
            .map(|(l, r)| l + r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> AddAssign for VectorDense<R> {
    fn add_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<R: Ring> Add<&VectorDense<R>> for VectorDense<R> {
    type Output = Self;

    fn add(self, rps: &VectorDense<R>) -> Self::Output {
        zip(self.elements, rps.elements.iter())
            .map(|(l, &r)| l + r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> AddAssign<&VectorDense<R>> for VectorDense<R> {
    fn add_assign(&mut self, rps: &VectorDense<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l += r);
    }
}

impl<R: Ring> Add<VectorDense<R>> for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn add(self, rps: VectorDense<R>) -> Self::Output {
        zip(self.elements.iter(), rps.elements)
            .map(|(&l, r)| l + r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> Add for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn add(self, rps: Self) -> Self::Output {
        zip(self.elements.iter(), rps.elements.iter())
            .map(|(&l, &r)| l + r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> Neg for VectorDense<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.elements
            .into_iter()
            .map(|e| -e)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> Neg for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn neg(self) -> Self::Output {
        self.elements.iter().map(|&e| -e).collect::<Vec<R>>().into()
    }
}

impl<R: Ring> Sub for VectorDense<R> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        zip(self.elements, rps.elements)
            .map(|(l, r)| l - r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> SubAssign for VectorDense<R> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring> Sub<&VectorDense<R>> for VectorDense<R> {
    type Output = Self;

    fn sub(self, rps: &VectorDense<R>) -> Self::Output {
        zip(self.elements, rps.elements.iter())
            .map(|(l, &r)| l - r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> SubAssign<&VectorDense<R>> for VectorDense<R> {
    fn sub_assign(&mut self, rps: &VectorDense<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l -= r);
    }
}

impl<R: Ring> Sub<VectorDense<R>> for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn sub(self, rps: VectorDense<R>) -> Self::Output {
        zip(self.elements.iter(), rps.elements)
            .map(|(&l, r)| l - r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> Sub for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn sub(self, rps: Self) -> Self::Output {
        zip(self.elements.iter(), rps.elements.iter())
            .map(|(&l, &r)| l - r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> Mul for VectorDense<R> {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        zip(self.elements, rps.elements)
            .map(|(l, r)| l * r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> MulAssign for VectorDense<R> {
    fn mul_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l *= r);
    }
}

impl<R: Ring> Mul<&VectorDense<R>> for VectorDense<R> {
    type Output = Self;

    fn mul(self, rps: &VectorDense<R>) -> Self::Output {
        zip(self.elements, rps.elements.iter())
            .map(|(l, &r)| l * r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> MulAssign<&VectorDense<R>> for VectorDense<R> {
    fn mul_assign(&mut self, rps: &VectorDense<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l *= r);
    }
}

impl<R: Ring> Mul<VectorDense<R>> for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: VectorDense<R>) -> Self::Output {
        zip(self.elements.iter(), rps.elements)
            .map(|(&l, r)| l * r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> Mul for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: Self) -> Self::Output {
        zip(self.elements.iter(), rps.elements.iter())
            .map(|(&l, &r)| l * r)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> Mul<R> for VectorDense<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        self.elements
            .into_iter()
            .map(|l| l * rps)
            .collect::<Vec<R>>()
            .into()
    }
}

impl<R: Ring> MulAssign<R> for VectorDense<R> {
    fn mul_assign(&mut self, rps: R) {
        self.elements.iter_mut().for_each(|l| *l *= rps);
    }
}

impl<R: Ring> Mul<R> for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: R) -> Self::Output {
        self.elements
            .iter()
            .map(|&l| l * rps)
            .collect::<Vec<R>>()
            .into()
    }
}
