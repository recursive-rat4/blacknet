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

use crate::algebra::{One, Square};
use crate::bigint::UInt256;
use crate::ed25519::Field25519;

pub(super) const E25519_D: Field25519 = unsafe {
    Field25519::from_unchecked(UInt256::from_hex(
        "52036CEE2B6FFE738CC740797779E89800700A4D4141D8AB75EB4DCA135978A3",
    ))
};
pub(super) const E25519_D_TWICE: Field25519 = unsafe {
    Field25519::from_unchecked(UInt256::from_hex(
        "2406D9DC56DFFCE7198E80F2EEF3D13000E0149A8283B156EBD69B9426B2F159",
    ))
};

pub(super) fn is_on_curve25519(x: Field25519, y: Field25519) -> bool {
    let xx = x.square();
    let yy = y.square();
    yy - xx == Field25519::ONE + E25519_D * xx * yy
}
