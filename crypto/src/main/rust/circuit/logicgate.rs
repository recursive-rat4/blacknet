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

use crate::circuit::circuitbuilder::{CircuitBuilder, Constant, LinearCombination};
use crate::operation::Double;
use crate::ring::UnitalRing;
use alloc::vec::Vec;

pub struct LogicGate<'a, R: UnitalRing> {
    circuit: &'a CircuitBuilder<R>,
}

impl<'a, R: UnitalRing> LogicGate<'a, R> {
    pub const fn new(circuit: &'a CircuitBuilder<R>) -> Self {
        Self { circuit }
    }

    pub fn check_range(&self, a: &LinearCombination<R>) {
        let scope = self.circuit.scope("LogicGate::check_range");
        scope.constrain(a * (a - Constant::UNITY), Constant::ZERO);
    }

    pub fn check_range_slice(&self, a: &[LinearCombination<R>]) {
        let scope = self.circuit.scope("LogicGate::check_range_slice");
        for a in a {
            scope.constrain(a * (a - Constant::UNITY), Constant::ZERO);
        }
    }

    pub fn xor(&self, a: &LinearCombination<R>, b: &LinearCombination<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("LogicGate::xor");
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);
        a + b - ab.double()
    }

    pub fn and(&self, a: &LinearCombination<R>, b: &LinearCombination<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("LogicGate::and");
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);
        ab.into()
    }

    pub fn or(&self, a: &LinearCombination<R>, b: &LinearCombination<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("LogicGate::or");
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);
        a + b - ab
    }

    pub fn not(&self, a: &LinearCombination<R>) -> LinearCombination<R> {
        Constant::UNITY - a
    }

    #[allow(clippy::len_zero)]
    pub fn and_slice(&self, a: &[LinearCombination<R>]) -> LinearCombination<R> {
        if a.len() == 0 {
            return Constant::UNITY.into();
        } else if a.len() == 1 {
            return a[0].clone();
        }
        let scope = self.circuit.scope("LogicGate::and_slice");
        let mut pi = a[0].clone();
        for a in a.iter().skip(1) {
            let p = scope.auxiliary();
            scope.constrain(pi * a, p);
            pi = p.into();
        }
        pi
    }

    pub fn check_less_or_equal(&self, a: &[LinearCombination<R>], b: &[R]) {
        let scope = self.circuit.scope("LogicGate::check_less_or_equal");
        let mut current_run = Vec::<LinearCombination<R>>::with_capacity(b.len());
        let mut last_run: Option<LinearCombination<R>> = None;
        for i in (0..b.len()).rev() {
            let digit = &a[i];
            if b[i] == R::UNITY {
                scope.constrain(digit * (digit - Constant::UNITY), Constant::ZERO);
                current_run.push(digit.clone());
            } else {
                if !current_run.is_empty() {
                    if let Some(last_run) = last_run {
                        current_run.push(last_run);
                    }
                    last_run = Some(self.and_slice(&current_run));
                    current_run.clear();
                }
                if let Some(last_run) = &last_run {
                    scope.constrain(digit * (digit - Constant::UNITY + last_run), Constant::ZERO);
                } else {
                    scope.constrain(digit.clone(), Constant::ZERO);
                }
            }
        }
    }
}
