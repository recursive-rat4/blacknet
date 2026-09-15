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

mod binaryuniformdistribution;
mod discretegaussiandistribution;
mod distribution;
mod drg;
#[cfg(feature = "std")]
mod fastrng;
mod float01distribution;
mod generator;
pub mod hammingweight;
mod quartaryuniformdistribution;
#[cfg(feature = "std")]
mod seed;
mod uniformintdistribution;
mod uniformmoddistribution;

pub use binaryuniformdistribution::BinaryUniformDistribution;
pub use discretegaussiandistribution::DiscreteGaussianDistribution;
pub use distribution::{Distribution, UniformDistribution};
pub use drg::{FastDRG, StrongDRG};
#[cfg(feature = "std")]
pub use fastrng::FAST_RNG;
pub use float01distribution::Float01Distribution;
pub use generator::{BufferedGenerator, Seedable, UniformBitGenerator, UniformGenerator};
pub use hammingweight::fill_with_weight;
pub use quartaryuniformdistribution::QuartaryUniformDistribution;
#[cfg(feature = "std")]
pub use seed::seed;
pub use uniformintdistribution::UniformIntDistribution;
pub use uniformmoddistribution::UniformModDistribution;
