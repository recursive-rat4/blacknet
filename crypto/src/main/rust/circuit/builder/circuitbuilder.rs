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

use crate::algebra::{UnitalRing, UnitalSemiring};
use crate::circuit::builder::{
    LinearCombination, LinearSpan, Variable, VariableKind,
    tree::{NodeId, Tree},
};
use crate::customizableconstraintsystem::CustomizableConstraintSystem;
use crate::matrix::SparseMatrixBuilder;
use crate::r1cs::R1CS;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::cmp::max;
use core::fmt::{Display, Formatter, Result};
use core::iter::zip;

/// An expression to be constrained.
pub trait Expression<R: UnitalSemiring> {
    fn span(self) -> LinearSpan<R>;
    fn degree(&self) -> usize;
}

/// An equivalence constraint.
pub struct Constraint<R: UnitalSemiring> {
    lps: LinearSpan<R>,
    rps: LinearSpan<R>,
}

/// The builder.
pub struct CircuitBuilder<R: UnitalSemiring> {
    degree: usize,
    public_inputs: Cell<usize>,
    public_outputs: Cell<usize>,
    private_inputs: Cell<usize>,
    private_outputs: Cell<usize>,
    auxiliaries: Cell<usize>,
    constraints: RefCell<Vec<Constraint<R>>>,
    scopes: RefCell<Tree<ScopeInfo>>,
    current_scope: Cell<NodeId>,
}

impl<R: UnitalSemiring> CircuitBuilder<R> {
    /// Construct a new builder with a maximum `degree` of constraints.
    pub fn new(degree: usize) -> Self {
        let (tree, root) = Tree::with_root(ScopeInfo::root());
        Self {
            degree,
            public_inputs: Cell::new(0),
            public_outputs: Cell::new(0),
            private_inputs: Cell::new(0),
            private_outputs: Cell::new(0),
            auxiliaries: Cell::new(0),
            constraints: RefCell::new(Vec::new()),
            scopes: RefCell::new(tree),
            current_scope: Cell::new(root),
        }
    }

    /// Maximum degree of constraints.
    pub const fn degree(&self) -> usize {
        self.degree
    }

    /// Number of constraints.
    pub fn constraints(&self) -> usize {
        self.constraints.borrow().len()
    }

    /// Number of variables.
    pub const fn variables(&self) -> usize {
        1 + self.public_inputs.get()
            + self.public_outputs.get()
            + self.private_inputs.get()
            + self.private_outputs.get()
            + self.auxiliaries.get()
    }

    /// Enter a new scope.
    pub fn scope(&self, name: &'static str) -> Scope<'_, R> {
        let mut scopes = self.scopes.borrow_mut();
        let info = ScopeInfo::new(name);
        self.current_scope
            .update(|id| scopes.descend(id, info).expect("Tree"));
        Scope { builder: self }
    }

    #[must_use = "Circuit variable should be constrained"]
    fn allocate(&self, kind: VariableKind) -> Variable<R> {
        let mut scopes = self.scopes.borrow_mut();
        let scope = scopes.get_mut(self.current_scope.get()).expect("Scope");
        scope.variables += 1;

        let n = match kind {
            VariableKind::PublicInput => {
                let n = self.public_inputs.get();
                self.public_inputs.update(|n| n + 1);
                n
            }
            VariableKind::PublicOutput => {
                let n = self.public_outputs.get();
                self.public_outputs.update(|n| n + 1);
                n
            }
            VariableKind::PrivateInput => {
                let n = self.private_inputs.get();
                self.private_inputs.update(|n| n + 1);
                n
            }
            VariableKind::PrivateOutput => {
                let n = self.private_outputs.get();
                self.private_outputs.update(|n| n + 1);
                n
            }
            VariableKind::Auxiliary => {
                let n = self.auxiliaries.get();
                self.auxiliaries.update(|n| n + 1);
                n
            }
            VariableKind::Constant => panic!("New constant variable requested"),
        };
        Variable::new(kind, n)
    }

    fn constrain(&self, constraint: Constraint<R>) {
        let mut scopes = self.scopes.borrow_mut();
        let scope = scopes.get_mut(self.current_scope.get()).expect("Scope");

        assert!(
            self.degree >= constraint.lps.dimension(),
            "In scope {} constraint left dimension {} is higher than circuit degree {}",
            scope.name,
            constraint.lps.dimension(),
            self.degree
        );
        assert!(
            self.degree >= constraint.rps.dimension(),
            "In scope {} constraint right dimension {} is higher than circuit degree {}",
            scope.name,
            constraint.rps.dimension(),
            self.degree
        );

        scope.constraints += 1;
        let mut constraints = self.constraints.borrow_mut();
        constraints.push(constraint)
    }

    fn pad(&self, m: &mut SparseMatrixBuilder<R>) {
        unsafe { m.column_unchecked(0, R::ONE) };
        m.row();
    }

    fn lay_out(&self) {
        let mut n;
        let mut offset = 1;

        n = self.public_inputs.get();
        self.public_inputs.set(offset);
        offset += n;

        n = self.public_outputs.get();
        self.public_outputs.set(offset);
        offset += n;

        n = self.private_inputs.get();
        self.private_inputs.set(offset);
        offset += n;

        n = self.private_outputs.get();
        self.private_outputs.set(offset);
        offset += n;

        self.auxiliaries.set(offset);
    }
}

impl<R: UnitalSemiring + Clone + Eq> CircuitBuilder<R> {
    fn put(&self, m: &mut SparseMatrixBuilder<R>, lc: &LinearCombination<R>) {
        for (variable, coefficient) in &lc.terms {
            let column: usize = match variable.kind {
                VariableKind::Constant => 0,
                VariableKind::PublicInput => self.public_inputs.get() + variable.number,
                VariableKind::PublicOutput => self.public_outputs.get() + variable.number,
                VariableKind::PrivateInput => self.private_inputs.get() + variable.number,
                VariableKind::PrivateOutput => self.private_outputs.get() + variable.number,
                VariableKind::Auxiliary => self.auxiliaries.get() + variable.number,
            };
            m.column_ref(column, &coefficient.value);
        }
        m.row();
    }

    /// Compile to R1CS.
    ///
    /// # Panics
    ///
    /// If the shape is not compatible.
    pub fn r1cs(self) -> R1CS<R> {
        let (constraints_num, variables_num) = (self.constraints(), self.variables());
        let constraints = self.constraints.take();
        let (lps_dimension, rps_dimension) = constraints
            .iter()
            .map(|c| (c.lps.dimension(), c.rps.dimension()))
            .fold((0, 0), |acc, x| (max(acc.0, x.0), max(acc.1, x.1)));
        assert!(
            lps_dimension <= 2 && rps_dimension <= 1,
            "Shape [{lps_dimension}, {rps_dimension}] is not compatible with [2, 1]"
        );
        let mut a = SparseMatrixBuilder::<R>::new(constraints_num, variables_num);
        let mut b = SparseMatrixBuilder::<R>::new(constraints_num, variables_num);
        let mut c = SparseMatrixBuilder::<R>::new(constraints_num, variables_num);

        self.lay_out();
        for constraint in constraints {
            let (lps_span, rps_span) = (constraint.lps, constraint.rps);
            match lps_span.dimension() {
                2 => {
                    self.put(&mut a, &lps_span[0]);
                    self.put(&mut b, &lps_span[1]);
                }
                1 => {
                    self.put(&mut a, &lps_span[0]);
                    self.pad(&mut b);
                }
                0 => {
                    self.pad(&mut a);
                    self.pad(&mut b);
                }
                _ => unreachable!(),
            }
            match rps_span.dimension() {
                1 => {
                    self.put(&mut c, &rps_span[0]);
                }
                0 => {
                    self.pad(&mut c);
                }
                _ => unreachable!(),
            }
        }

        R1CS::new(a.build(), b.build(), c.build())
    }
}

impl<R: UnitalRing + Clone + Eq> CircuitBuilder<R> {
    /// Compile to CCS.
    pub fn ccs(self) -> CustomizableConstraintSystem<R> {
        let (constraints_num, variables_num) = (self.constraints(), self.variables());
        let constraints = self.constraints.take();
        let (lps_dimension, rps_dimension) = constraints
            .iter()
            .map(|c| (c.lps.dimension(), c.rps.dimension()))
            .fold((0, 0), |acc, x| (max(acc.0, x.0), max(acc.1, x.1)));
        let (mut lps_matrices, mut rps_matrices) = (Vec::new(), Vec::new());
        lps_matrices.resize_with(lps_dimension, || {
            SparseMatrixBuilder::<R>::new(constraints_num, variables_num)
        });
        rps_matrices.resize_with(rps_dimension, || {
            SparseMatrixBuilder::<R>::new(constraints_num, variables_num)
        });

        self.lay_out();
        for constraint in constraints {
            let (lps_span, rps_span) = (constraint.lps, constraint.rps);
            for (matrix, lc) in zip(&mut lps_matrices, &lps_span) {
                self.put(matrix, lc)
            }
            for matrix in lps_matrices.iter_mut().skip(lps_span.dimension()) {
                self.pad(matrix)
            }
            for (matrix, lc) in zip(&mut rps_matrices, &rps_span) {
                self.put(matrix, lc)
            }
            for matrix in rps_matrices.iter_mut().skip(rps_span.dimension()) {
                self.pad(matrix)
            }
        }

        let mut matrices = Vec::with_capacity(lps_dimension + rps_dimension);
        lps_matrices
            .into_iter()
            .for_each(|b| matrices.push(b.build()));
        rps_matrices
            .into_iter()
            .for_each(|b| matrices.push(b.build()));

        let multisets = vec![(0..matrices.len() - 1).collect(), vec![matrices.len() - 1]];

        let constants = vec![R::ONE, -R::ONE];

        CustomizableConstraintSystem::new(matrices, multisets, constants)
    }
}

impl<R: UnitalSemiring> Display for CircuitBuilder<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Circuit degree {} constraints {} variables {}\n{}",
            self.degree,
            self.constraints(),
            self.variables(),
            self.scopes.borrow()
        )
    }
}

/// A named scope to allocate variables and constrain expressions.
pub struct Scope<'a, R: UnitalSemiring> {
    builder: &'a CircuitBuilder<R>,
}

impl<'a, R: UnitalSemiring> Scope<'a, R> {
    /// Build a constraint `lps == rps`.
    ///
    /// # Panics
    ///
    /// If constraint degree is higher than circuit degree.
    pub fn constrain<LPS: Expression<R>, RPS: Expression<R>>(&self, lps: LPS, rps: RPS) {
        self.builder.constrain(Constraint {
            lps: lps.span(),
            rps: rps.span(),
        })
    }

    /// Allocate [PublicInput][crate::circuit::builder::VariableKind::PublicInput] variable.
    #[must_use = "Circuit variable should be constrained"]
    pub fn public_input(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PublicInput)
    }

    /// Allocate [PublicOutput][crate::circuit::builder::VariableKind::PublicOutput] variable.
    #[must_use = "Circuit variable should be constrained"]
    pub fn public_output(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PublicOutput)
    }

    /// Allocate [PrivateInput][crate::circuit::builder::VariableKind::PrivateInput] variable.
    #[must_use = "Circuit variable should be constrained"]
    pub fn private_input(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PrivateInput)
    }

    /// Allocate [PrivateOutput][crate::circuit::builder::VariableKind::PrivateOutput] variable.
    #[must_use = "Circuit variable should be constrained"]
    pub fn private_output(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PrivateOutput)
    }

    /// Allocate [Auxiliary][crate::circuit::builder::VariableKind::Auxiliary] variable.
    #[must_use = "Circuit variable should be constrained"]
    pub fn auxiliary(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::Auxiliary)
    }

    /// Allocate a variable of given kind.
    ///
    /// # Panics
    ///
    /// If the kind is [Constant][crate::circuit::builder::VariableKind::Constant].
    #[must_use = "Circuit variable should be constrained"]
    pub fn variable(&self, kind: VariableKind) -> Variable<R> {
        self.builder.allocate(kind)
    }
}

impl<'a, R: UnitalSemiring> Drop for Scope<'a, R> {
    fn drop(&mut self) {
        let scopes = self.builder.scopes.borrow();
        self.builder
            .current_scope
            .update(|id| scopes.ascendant(id).expect("Tree"));
    }
}

struct ScopeInfo {
    name: &'static str,
    constraints: usize,
    variables: usize,
}

impl ScopeInfo {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            constraints: 0,
            variables: 0,
        }
    }

    const fn root() -> Self {
        Self {
            name: "Root",
            constraints: 0,
            variables: 1,
        }
    }
}

impl Display for ScopeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}x{}", self.name, self.constraints, self.variables)
    }
}
