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
use crate::matrix::{SparseMatrix, SparseMatrixBuilder};
use crate::r1cs::R1CS;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
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
    shape: [usize; 2],
    public_inputs: Cell<usize>,
    public_offset: Cell<usize>,
    private_inputs: Cell<usize>,
    private_offset: Cell<usize>,
    auxiliaries: Cell<usize>,
    auxiliary_offset: Cell<usize>,
    laid_out: Cell<bool>,
    constraints: Cell<usize>,
    lps_matrices: RefCell<Vec<SparseMatrixBuilder<R>>>,
    rps_matrices: RefCell<Vec<SparseMatrixBuilder<R>>>,
    scopes: RefCell<Tree<ScopeInfo>>,
    current_scope: Cell<NodeId>,
}

impl<R: UnitalSemiring> CircuitBuilder<R> {
    /// Construct a new builder with R1CS shape.
    pub fn r1cs() -> Self {
        Self::with_shape([2, 1])
    }

    /// Construct a new builder.
    pub fn with_shape(shape: [usize; 2]) -> Self {
        let lps_matrices = (0..shape[0]).map(|_| SparseMatrixBuilder::new()).collect();
        let rps_matrices = (0..shape[1]).map(|_| SparseMatrixBuilder::new()).collect();
        let (tree, root) = Tree::with_root(ScopeInfo::root());
        Self {
            shape,
            public_inputs: Cell::new(0),
            public_offset: Cell::new(0),
            private_inputs: Cell::new(0),
            private_offset: Cell::new(0),
            auxiliaries: Cell::new(0),
            auxiliary_offset: Cell::new(0),
            laid_out: Cell::new(false),
            constraints: Cell::new(0),
            lps_matrices: RefCell::new(lps_matrices),
            rps_matrices: RefCell::new(rps_matrices),
            scopes: RefCell::new(tree),
            current_scope: Cell::new(root),
        }
    }

    /// Shape of circuit.
    pub const fn shape(&self) -> &[usize; 2] {
        &self.shape
    }

    /// Number of constraints.
    pub const fn constraints(&self) -> usize {
        self.constraints.get()
    }

    /// Number of variables.
    pub const fn variables(&self) -> usize {
        1 + self.public_inputs.get() + self.private_inputs.get() + self.auxiliaries.get()
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
            VariableKind::Public => {
                assert!(!self.laid_out.get(), "Inputs are already laid out");
                let n = self.public_inputs.get();
                self.public_inputs.update(|n| n + 1);
                n
            }
            VariableKind::Private => {
                assert!(!self.laid_out.get(), "Inputs are already laid out");
                let n = self.private_inputs.get();
                self.private_inputs.update(|n| n + 1);
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

    fn pad(&self, m: &mut SparseMatrixBuilder<R>) {
        unsafe { m.column_unchecked(0, R::ONE) };
        m.row();
    }

    /// Lay out input variables.
    ///
    /// After this new inputs can't be allocated unless auxiliary.
    /// This is required before constraining.
    pub fn lay_out(&self) {
        assert!(!self.laid_out.get(), "Inputs are already laid out");

        let mut n;
        let mut offset = 1;

        n = self.public_inputs.get();
        self.public_offset.set(offset);
        offset += n;

        n = self.private_inputs.get();
        self.private_offset.set(offset);
        offset += n;

        self.auxiliary_offset.set(offset);

        self.laid_out.set(true)
    }
}

impl<R: UnitalSemiring + Clone + Eq> CircuitBuilder<R> {
    fn constrain(&self, constraint: Constraint<R>) {
        assert!(self.laid_out.get(), "Inputs are not laid out yet");

        let mut scopes = self.scopes.borrow_mut();
        let scope = scopes.get_mut(self.current_scope.get()).expect("Scope");

        assert!(
            self.shape[0] >= constraint.lps.dimension(),
            "In scope {} constraint left dimension {} is higher than circuit {}",
            scope.name,
            constraint.lps.dimension(),
            self.shape[0]
        );
        assert!(
            self.shape[1] >= constraint.rps.dimension(),
            "In scope {} constraint right dimension {} is higher than circuit {}",
            scope.name,
            constraint.rps.dimension(),
            self.shape[1]
        );

        let (lps_span, rps_span) = (constraint.lps, constraint.rps);
        let mut lps_matrices = self.lps_matrices.borrow_mut();
        let (lps_put, lps_pad) = lps_matrices.split_at_mut(lps_span.dimension());
        for (matrix, lc) in zip(lps_put, lps_span) {
            self.put(matrix, lc)
        }
        for matrix in lps_pad {
            self.pad(matrix)
        }
        let mut rps_matrices = self.rps_matrices.borrow_mut();
        let (rps_put, rps_pad) = rps_matrices.split_at_mut(rps_span.dimension());
        for (matrix, lc) in zip(rps_put, rps_span) {
            self.put(matrix, lc)
        }
        for matrix in rps_pad {
            self.pad(matrix)
        }

        scope.constraints += 1;
        self.constraints.update(|n| n + 1);
    }

    fn put(&self, m: &mut SparseMatrixBuilder<R>, lc: LinearCombination<R>) {
        for (variable, coefficient) in lc.terms {
            let column: usize = match variable.kind() {
                VariableKind::Constant => 0,
                VariableKind::Public => self.public_offset.get() + variable.number(),
                VariableKind::Private => self.private_offset.get() + variable.number(),
                VariableKind::Auxiliary => self.auxiliary_offset.get() + variable.number(),
            };
            m.column(column, coefficient.value);
        }
        m.row();
    }

    /// Compile to R1CS.
    ///
    /// # Panics
    ///
    /// If the shape is not compatible.
    pub fn to_r1cs(self) -> R1CS<R> {
        assert!(
            self.shape == [2, 1],
            "Shape {:?} is not compatible with [2, 1]",
            self.shape,
        );

        let variables = self.variables();

        let mut lps_matrices = self.lps_matrices.take().into_iter();
        let mut rps_matrices = self.rps_matrices.take().into_iter();
        let mut a = lps_matrices.next().unwrap();
        let mut b = lps_matrices.next().unwrap();
        let mut c = rps_matrices.next().unwrap();
        a.columns(variables);
        b.columns(variables);
        c.columns(variables);

        R1CS::new(a.build(), b.build(), c.build())
    }
}

impl<R: UnitalRing + Clone + Eq> CircuitBuilder<R> {
    /// Compile to CCS.
    pub fn to_ccs(self) -> CustomizableConstraintSystem<R> {
        let variables = self.variables();

        let lps_matrices = self.lps_matrices.take().into_iter();
        let rps_matrices = self.rps_matrices.take().into_iter();
        let matrices: Vec<SparseMatrix<R>> = lps_matrices
            .chain(rps_matrices)
            .map(|mut builder| {
                builder.columns(variables);
                builder.build()
            })
            .collect();

        let multisets = vec![(0..matrices.len() - 1).collect(), vec![matrices.len() - 1]];

        let constants = vec![R::ONE, -R::ONE];

        CustomizableConstraintSystem::new(matrices, multisets, constants)
    }
}

impl<R: UnitalSemiring> Display for CircuitBuilder<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Circuit shape {:?} constraints {} variables {}\n{}",
            self.shape,
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
    pub fn constrain<LPS: Expression<R>, RPS: Expression<R>>(&self, lps: LPS, rps: RPS)
    where
        R: Clone + Eq,
    {
        self.builder.constrain(Constraint {
            lps: lps.span(),
            rps: rps.span(),
        })
    }

    /// Allocate [Public][crate::circuit::builder::VariableKind::Public] variable.
    #[must_use = "Circuit variable should be constrained"]
    pub fn public(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::Public)
    }

    /// Allocate [Private][crate::circuit::builder::VariableKind::Private] variable.
    #[must_use = "Circuit variable should be constrained"]
    pub fn private(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::Private)
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
