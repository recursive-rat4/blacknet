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

mod affine;
mod edwards25519;
mod extended;
mod field;
mod projective;
mod scalar;

use edwards25519::{E25519_D, E25519_D_TWICE, is_on_curve25519};

pub use affine::Edwards25519Affine;
pub use extended::Edwards25519Extended;
pub use field::Field25519;
pub use projective::Edwards25519Projective;
pub use scalar::Scalar25519;
