/*
 * Copyright (c) 2024 Xerxes Rånby
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

use crate::algebra::{AdditiveCommutativeMonoid, MultiplicativeCommutativeMonoid};
use crate::algebra::{AdditiveGroup, MultiplicativeGroup};

pub trait AdditiveAbelianGroup: AdditiveGroup + AdditiveCommutativeMonoid {
    // Speeding up the computations on an elliptic curve using addition-subtraction chains
    // ADDSUBCHAIN-D
    // http://www.numdam.org/item/ITA_1990__24_6_531_0/
    fn add_sub_chain<Scalar: IntoIterator<Item = bool>>(self, scalar: Scalar) -> Self {
        let mut p = Self::ZERO;
        let mut q = self;

        let mut q_is_q_double = 0;
        let mut state = 0;

        let update_q = |mut q: Self, q_is_q_double: usize| -> (Self, usize) {
            for _ in 0..q_is_q_double {
                q = q.double();
            }
            (q, 0)
        };

        for bit in scalar {
            match state {
                0 => {
                    if bit {
                        state = 1;
                    } else {
                        q_is_q_double += 1;
                    }
                }
                1 => {
                    // Q only needs to be updated in case P gets updated
                    (q, q_is_q_double) = update_q(q, q_is_q_double);
                    if bit {
                        p -= &q;
                        q_is_q_double += 2;
                        state = 11;
                    } else {
                        p += &q;
                        q_is_q_double += 2;
                        state = 0;
                    }
                }
                11 => {
                    if bit {
                        q_is_q_double += 1;
                    } else {
                        state = 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        if state != 0 {
            // Q only needs to be updated in case P gets updated
            (q, _) = update_q(q, q_is_q_double);
            p += q;
        }

        p
    }
}

impl<G: AdditiveGroup + AdditiveCommutativeMonoid> AdditiveAbelianGroup for G {}

#[rustfmt::skip]
pub trait MultiplicativeAbelianGroup
    : MultiplicativeGroup
    + MultiplicativeCommutativeMonoid
{
}

impl<G: MultiplicativeGroup + MultiplicativeCommutativeMonoid> MultiplicativeAbelianGroup for G {}
