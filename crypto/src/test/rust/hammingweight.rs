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

use blacknet_crypto::random::{FastDRG, UniformIntDistribution, fill_with_weight};

#[test]
fn weight() {
    let mut drg = FastDRG::default();
    let mut dst = UniformIntDistribution::<u16, FastDRG>::new(0..256);
    let mut a = [0_u16; 32];
    fill_with_weight(&mut drg, &mut dst, &mut a, 4);
    let filled = a.into_iter().filter(|i| *i != 0).count();
    assert_eq!(filled, 4);
}
