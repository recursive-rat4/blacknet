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

use crate::algebra::{IntegerModRing, RingOps};
use crate::matrix::{DenseMatrix, DenseVector};
use crate::random::{BinaryUniformDistribution, Distribution, UniformGenerator};

/// A modular Johnson–Lindenstrauss variant.
///
/// <https://eprint.iacr.org/2021/1397.pdf>
pub struct JohnsonLindenstrauss<Z: IntegerModRing> {
    map: DenseMatrix<Z>,
}

impl<Z: IntegerModRing> JohnsonLindenstrauss<Z> {
    const K: usize = 256;

    pub fn random<G: UniformGenerator<Output = Z>>(generator: &mut G, n: usize) -> Self {
        let mut dst = WeightedDistribution::<Z>::new();
        let elements = (0..Self::K * n).map(|_| dst.sample(generator)).collect();
        let map = DenseMatrix::new(Self::K, n, elements);
        Self { map }
    }

    pub fn project(&self, point: &DenseVector<Z>) -> DenseVector<Z>
    where
        for<'a> &'a Z: RingOps<Z>,
    {
        &self.map * point
    }
}

struct WeightedDistribution<Z: IntegerModRing> {
    bud: BinaryUniformDistribution<Z>,
}

impl<Z: IntegerModRing> WeightedDistribution<Z> {
    pub const fn new() -> Self {
        Self {
            bud: BinaryUniformDistribution::new(),
        }
    }

    pub const fn reset(&mut self) {
        self.bud.reset()
    }
}

impl<Z: IntegerModRing> Default for WeightedDistribution<Z> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Z: IntegerModRing, G: UniformGenerator<Output = Z>> Distribution<Z, G>
    for WeightedDistribution<Z>
{
    fn sample(&mut self, generator: &mut G) -> Z {
        self.bud.sample(generator) + self.bud.sample(generator) - Z::ONE
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
