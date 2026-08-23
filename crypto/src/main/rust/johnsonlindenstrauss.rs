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
use crate::random::{Distribution, UniformBitGenerator};

/// A modular Johnson–Lindenstrauss variant.
///
/// <https://eprint.iacr.org/2021/1397.pdf>
pub struct JohnsonLindenstrauss<Z: IntegerModRing> {
    map: DenseMatrix<Z>,
}

impl<Z: IntegerModRing> JohnsonLindenstrauss<Z> {
    const K: u32 = 256;

    pub fn random<G: UniformBitGenerator>(generator: &mut G, n: u32) -> Self {
        let mut dst = WeightedDistribution::new();
        let map = DenseMatrix::fill_with(Self::K, n, || dst.sample(generator));
        Self { map }
    }

    pub fn project(&self, point: &DenseVector<Z>) -> DenseVector<Z>
    where
        for<'a> &'a Z: RingOps<Z>,
    {
        &self.map * point
    }
}

struct WeightedDistribution {
    cache: u8,
    have_bits: u32,
}

impl WeightedDistribution {
    pub const fn new() -> Self {
        Self {
            cache: 0,
            have_bits: 0,
        }
    }

    pub const fn reset(&mut self) {
        self.have_bits = 0
    }
}

impl Default for WeightedDistribution {
    fn default() -> Self {
        Self::new()
    }
}

impl<Z: IntegerModRing, G: UniformBitGenerator> Distribution<Z, G> for WeightedDistribution {
    fn sample(&mut self, generator: &mut G) -> Z {
        if self.have_bits == 0 {
            self.cache = generator.generate();
            self.have_bits = u8::BITS;
        }
        let a = (self.cache & 1) != 0;
        self.cache >>= 1;
        let b = (self.cache & 1) != 0;
        self.cache >>= 1;
        self.have_bits -= 2;
        match (a, b) {
            (false, false) => Z::ZERO,
            (false, true) => -Z::ONE,
            (true, false) => Z::ONE,
            (true, true) => Z::ZERO,
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
