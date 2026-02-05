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

use blacknet_crypto::chacha::BLOCK_SIZE;
use blacknet_crypto::random::{FastDRG, UniformGenerator};
use core::array;

#[test]
fn discard() {
    const SIZE: usize = BLOCK_SIZE * 2 + 1;
    let mut drg = FastDRG::default();

    let _: [u8; SIZE] = array::from_fn(|_| drg.generate());
    let buf1: [u8; SIZE] = array::from_fn(|_| drg.generate());

    drg.seed(Default::default());
    drg.discard(SIZE);
    let buf2: [u8; SIZE] = array::from_fn(|_| drg.generate());

    assert_eq!(buf2, buf1);
}
