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

use crate::algebra::{Field, One, Square};

pub trait TwistedEdwardsGroupParams {
    type F: Field;

    const A: Self::F;
    const D: Self::F;

    const A_IS_MINUS_ONE: bool;
}

pub fn is_on_curve<P: TwistedEdwardsGroupParams<F: Eq>>(x: P::F, y: P::F) -> bool {
    let xx = x.square();
    let yy = y.square();
    if P::A_IS_MINUS_ONE {
        yy - xx == P::F::ONE + P::D * xx * yy
    } else {
        yy + P::A * xx == P::F::ONE + P::D * xx * yy
    }
}
