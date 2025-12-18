/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::assigner::assigment::Assigment;
use crate::ring::UnitalRing;
use alloc::vec::Vec;

pub struct LogicGate<'a, R: UnitalRing> {
    assigment: &'a Assigment<R>,
}

impl<'a, R: UnitalRing + Eq> LogicGate<'a, R> {
    pub const fn new(assigment: &'a Assigment<R>) -> Self {
        Self { assigment }
    }

    pub fn xor(&self, a: R, b: R) -> R {
        let ab = a * b;
        self.assigment.push(ab);
        a + b - ab.double()
    }

    pub fn and(&self, a: R, b: R) -> R {
        let ab = a * b;
        self.assigment.push(ab);
        ab
    }

    pub fn or(&self, a: R, b: R) -> R {
        let ab = a * b;
        self.assigment.push(ab);
        a + b - ab
    }

    pub fn not(&self, a: R) -> R {
        R::ONE - a
    }

    #[allow(clippy::len_zero)]
    pub fn and_slice(&self, a: &[R]) -> R {
        if a.len() == 0 {
            return R::ONE;
        } else if a.len() == 1 {
            return a[0];
        };
        let mut pi = a[0];
        for &a in a.iter().skip(1) {
            pi = self.and(pi, a);
        }
        pi
    }

    pub fn check_less_or_equal(&self, a: &[R], b: &[R]) {
        let mut current_run = Vec::<R>::with_capacity(b.len());
        let mut last_run: Option<R> = None;
        for i in (0..b.len()).rev() {
            let digit = a[i];
            if b[i] == R::ONE {
                current_run.push(digit);
            } else if !current_run.is_empty() {
                if let Some(last_run) = last_run {
                    current_run.push(last_run);
                }
                last_run = Some(self.and_slice(&current_run));
                current_run.clear();
            }
        }
    }
}
