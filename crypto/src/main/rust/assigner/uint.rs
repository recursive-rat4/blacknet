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

use crate::algebra::{Algebra, AlgebraOps};
use crate::assigner::assigment::Assigment;
use crate::gf2::GF2;
use core::array;

pub struct UInt<'a, A: Algebra<GF2>, const N: usize> {
    bits: [A; N],
    assigment: &'a Assigment<A>,
}

impl<'a, A: Algebra<GF2> + Clone, const N: usize> UInt<'a, A, N>
where
    for<'b> &'b A: AlgebraOps<GF2, A>,
{
    pub const fn new(bits: [A; N], assigment: &'a Assigment<A>) -> Self {
        Self { bits, assigment }
    }

    pub fn wrapping_add(&self, rps: &Self) -> Self {
        let mut bits = [A::ZERO; N];
        if N == 0 {
            return Self {
                bits,
                assigment: self.assigment,
            };
        }
        let mut c = A::ZERO;
        #[allow(clippy::needless_range_loop)]
        for i in 0..N - 1 {
            let ac = &self.bits[i] + &c;
            let ac_bc = &ac * (&rps.bits[i] + &c);
            self.assigment.push(ac_bc.clone());
            bits[i] = ac + &rps.bits[i];
            c += ac_bc;
        }
        bits[N - 1] = &self.bits[N - 1] + &rps.bits[N - 1] + c;
        Self {
            bits,
            assigment: self.assigment,
        }
    }

    pub fn rotate_right(&self, n: u32) -> Self {
        let n = n as usize % N;
        let (a_l, a_r) = self.bits.split_at(n);
        let mut bits = [A::ZERO; N];
        let (o_l, o_r) = bits.split_at_mut(N - n);
        o_l.clone_from_slice(a_r);
        o_r.clone_from_slice(a_l);
        Self {
            bits,
            assigment: self.assigment,
        }
    }

    pub fn bitxor(&self, rps: &Self) -> Self {
        Self {
            bits: array::from_fn(|i| &self.bits[i] + &rps.bits[i]),
            assigment: self.assigment,
        }
    }
}
