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

use crate::float::Cast;
use crate::integer::Integer;
use crate::random::{Distribution, Float01Distribution, UniformGenerator, UniformIntDistribution};

/// SampleZ
///
/// <https://eprint.iacr.org/2007/432>
pub struct DiscreteGaussianDistribution<I: Integer, G: UniformGenerator<Output = u8>> {
    mu: f64,
    sigma: f64,
    uid: UniformIntDistribution<I, G>,
    f01: Float01Distribution<f64, G>,
}

impl<I: Integer, G: UniformGenerator<Output = u8>> DiscreteGaussianDistribution<I, G>
where
    f64: Cast<I>,
{
    /// Construct a new distribution.
    pub fn new(mu: f64, sigma: f64) -> Self {
        let range = Self::min(mu, sigma)..=Self::max(mu, sigma);
        Self {
            mu,
            sigma,
            uid: UniformIntDistribution::new(range),
            f01: Float01Distribution::new(),
        }
    }

    const N: usize = 128;

    fn min(mu: f64, sigma: f64) -> I {
        const {
            assert!(Self::N.count_ones() == 1);
        }
        let t = Self::N.trailing_zeros() as f64;
        libm::floor(mu - sigma * t).cast()
    }
    fn max(mu: f64, sigma: f64) -> I {
        const {
            assert!(Self::N.count_ones() == 1);
        }
        let t = Self::N.trailing_zeros() as f64;
        libm::ceil(mu + sigma * t).cast()
    }
}

impl<I: Integer + Cast<f64>, G: UniformGenerator<Output = u8>> Distribution<G>
    for DiscreteGaussianDistribution<I, G>
{
    type Output = I;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        loop {
            let xi: I = self.uid.sample(generator);
            let xf: f64 = xi.cast();
            let ps = libm::exp(-(xf - self.mu) * (xf - self.mu) / (2.0 * self.sigma * self.sigma));
            if self.f01.sample(generator) > ps {
                continue;
            }
            return xi;
        }
    }

    fn reset(&mut self) {
        self.uid.reset();
        self.f01.reset();
    }
}
