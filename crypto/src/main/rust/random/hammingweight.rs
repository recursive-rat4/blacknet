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

use crate::algebra::Zero;
use crate::random::{Distribution, UniformGenerator, UniformIntDistribution};

pub fn fill_with_weight<
    T: Zero + Eq,
    G: UniformGenerator<Output = u8>,
    D: Distribution<G, Output = T>,
>(
    rng: &mut G,
    dst: &mut D,
    slice: &mut [T],
    mut weight: usize,
) {
    debug_assert!(slice.len() >= weight);
    let mut uid = UniformIntDistribution::<usize, G>::new(0..slice.len());
    while weight > 0 {
        let i = uid.sample(rng);
        if slice[i] == T::ZERO {
            loop {
                slice[i] = dst.sample(rng);
                if slice[i] != T::ZERO {
                    break;
                }
            }
            weight -= 1;
        }
    }
}
