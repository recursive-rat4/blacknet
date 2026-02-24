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

use crate::algebra::Double;
use crate::algebra::UnitalRing;
use crate::circuit::builder::{CircuitBuilder, Constant, LinearCombination};
use alloc::vec::Vec;

/// Logic gates over a unital ring.
pub struct LogicGate<'a, 'b, R: UnitalRing> {
    circuit: &'a CircuitBuilder<'b, R>,
}

impl<'a, 'b, R: UnitalRing + Eq> LogicGate<'a, 'b, R> {
    pub const fn new(circuit: &'a CircuitBuilder<'b, R>) -> Self {
        Self { circuit }
    }

    /// Constrain that `a` is a Boolean value.
    pub fn check_range(&self, a: &LinearCombination<R>) {
        let scope = self.circuit.scope("LogicGate::check_range");
        scope.constrain(a * (a - Constant::ONE), Constant::ZERO);
    }

    /// Constrain that `a` is a sequence of Boolean values.
    pub fn check_range_slice(&self, a: &[LinearCombination<R>]) {
        let scope = self.circuit.scope("LogicGate::check_range_slice");
        for a in a {
            scope.constrain(a * (a - Constant::ONE), Constant::ZERO);
        }
    }

    /// Compute the exclusive disjunction `a ⊻ b`.
    pub fn xor(&self, a: &LinearCombination<R>, b: &LinearCombination<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("LogicGate::xor");
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);
        a + b - ab.double()
    }

    /// Compute the conjunction `a ∧ b`.
    pub fn and(&self, a: &LinearCombination<R>, b: &LinearCombination<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("LogicGate::and");
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);
        ab.into()
    }

    /// Compute the disjunction `a ∨ b`.
    pub fn or(&self, a: &LinearCombination<R>, b: &LinearCombination<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("LogicGate::or");
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);
        a + b - ab
    }

    /// Compute the negation `¬a`.
    pub fn not(&self, a: &LinearCombination<R>) -> LinearCombination<R> {
        Constant::ONE - a
    }

    /// Compute the n-ary conjunction `⋀a`.
    pub fn and_slice(&self, a: &[LinearCombination<R>]) -> LinearCombination<R> {
        match a.len() {
            0 => Constant::ONE.into(),
            1 => a[0].clone(),
            _ => {
                let scope = self.circuit.scope("LogicGate::and_slice");
                let mut pi = a[0].clone();
                for a in a.iter().skip(1) {
                    let p = scope.auxiliary();
                    scope.constrain(pi * a, p);
                    pi = p.into();
                }
                pi
            }
        }
    }

    /// Constrain the inequality `a ≤ b` in the binary numeral system.
    pub fn check_less_or_equal(&self, a: &[LinearCombination<R>], b: &[R]) {
        let scope = self.circuit.scope("LogicGate::check_less_or_equal");
        let mut current_run = Vec::<LinearCombination<R>>::with_capacity(b.len());
        let mut last_run: Option<LinearCombination<R>> = None;
        for i in (0..b.len()).rev() {
            let digit = &a[i];
            if b[i] == R::ONE {
                scope.constrain(digit * (digit - Constant::ONE), Constant::ZERO);
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
                    scope.constrain(digit * (digit - Constant::ONE + last_run), Constant::ZERO);
                } else {
                    scope.constrain(digit.clone(), Constant::ZERO);
                }
            }
        }
    }
}
