/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

use crate::circuit::builder::{LinearCombination, LinearSpan, Variable, VariableKind};
use crate::customizableconstraintsystem::CustomizableConstraintSystem;
use crate::r1cs::R1CS;
use crate::ring::UnitalRing;
use crate::semiring::Semiring;
use crate::sparsematrix::SparseMatrixBuilder;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::cmp::max;
use core::fmt::{Display, Formatter, Result};
use orx_tree::{Dyn, DynTree, NodeIdx, NodeRef};

/// An expression to be constrained.
pub trait Expression<'a, R: Semiring + 'a>: 'a {
    fn span(&self) -> LinearSpan<R>;
    fn degree(&self) -> usize;
}

/// An equivalence constraint.
pub struct Constraint<'a, R: Semiring> {
    lps: Box<dyn Expression<'a, R>>,
    rps: Box<dyn Expression<'a, R>>,
}

/// The builder.
pub struct CircuitBuilder<'a, R: Semiring> {
    degree: usize,
    public_inputs: Cell<usize>,
    public_outputs: Cell<usize>,
    private_inputs: Cell<usize>,
    private_outputs: Cell<usize>,
    auxiliaries: Cell<usize>,
    constraints: RefCell<Vec<Constraint<'a, R>>>,
    scopes: RefCell<DynTree<ScopeInfo>>,
    current_scope: RefCell<NodeIdx<Dyn<ScopeInfo>>>,
}

impl<'a, R: Semiring> CircuitBuilder<'a, R> {
    /// Construct a new builder with a maximum `degree` of constraints.
    pub fn new(degree: usize) -> Self {
        let mut tree = DynTree::empty();
        let root = tree.push_root(ScopeInfo::root());
        Self {
            degree,
            public_inputs: Cell::new(0),
            public_outputs: Cell::new(0),
            private_inputs: Cell::new(0),
            private_outputs: Cell::new(0),
            auxiliaries: Cell::new(0),
            constraints: RefCell::new(Vec::new()),
            scopes: RefCell::new(tree),
            current_scope: RefCell::new(root),
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
    pub fn scope<'b>(&'b self, name: &'static str) -> Scope<'b, 'a, R> {
        let mut scopes = self.scopes.borrow_mut();
        let mut current_scope = self.current_scope.borrow_mut();
        let info = ScopeInfo::new(name);
        let mut node = scopes.get_node_mut(*current_scope).expect("Scope");
        *current_scope = node.push_child(info);
        Scope { builder: self }
    }

    #[must_use = "Circuit variable should be constrained"]
    fn allocate(&self, kind: VariableKind) -> Variable<R> {
        let mut scopes = self.scopes.borrow_mut();
        let current_scope = self.current_scope.borrow();
        let mut scope = scopes.get_node_mut(*current_scope).expect("Scope");
        let info = scope.data_mut();
        info.variables += 1;

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

    fn constrain(&self, constraint: Constraint<'a, R>) {
        let mut scopes = self.scopes.borrow_mut();
        let current_scope = self.current_scope.borrow();
        let mut scope = scopes.get_node_mut(*current_scope).expect("Scope");
        let info = scope.data_mut();

        assert!(
            self.degree >= constraint.lps.degree(),
            "In scope {} constraint left degree {} is higher than circuit degree {}",
            info.name,
            constraint.lps.degree(),
            self.degree
        );
        assert!(
            self.degree >= constraint.rps.degree(),
            "In scope {} constraint right degree {} is higher than circuit degree {}",
            info.name,
            constraint.rps.degree(),
            self.degree
        );

        info.constraints += 1;
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

impl<'a, R: Semiring + Eq> CircuitBuilder<'a, R> {
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
            m.column(column, coefficient.value);
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
        let (lps_degree, rps_degree) = constraints
            .iter()
            .map(|c| (c.lps.degree(), c.rps.degree()))
            .fold((0, 0), |acc, x| (max(acc.0, x.0), max(acc.1, x.1)));
        assert!(
            lps_degree <= 2 && rps_degree <= 1,
            "Shape [{lps_degree}, {rps_degree}] is not compatible with [2, 1]"
        );
        let mut a = SparseMatrixBuilder::<R>::new(constraints_num, variables_num);
        let mut b = SparseMatrixBuilder::<R>::new(constraints_num, variables_num);
        let mut c = SparseMatrixBuilder::<R>::new(constraints_num, variables_num);

        self.lay_out();
        for constraint in constraints {
            let (lps_span, rps_span) = (constraint.lps.span(), constraint.rps.span());
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

impl<'a, R: UnitalRing + Eq> CircuitBuilder<'a, R> {
    /// Compile to CCS.
    pub fn ccs(self) -> CustomizableConstraintSystem<R> {
        let (constraints_num, variables_num) = (self.constraints(), self.variables());
        let constraints = self.constraints.take();
        let (lps_degree, rps_degree) = constraints
            .iter()
            .map(|c| (c.lps.degree(), c.rps.degree()))
            .fold((0, 0), |acc, x| (max(acc.0, x.0), max(acc.1, x.1)));
        let (mut lps_matrices, mut rps_matrices) = (Vec::new(), Vec::new());
        lps_matrices.resize_with(lps_degree, || {
            SparseMatrixBuilder::<R>::new(constraints_num, variables_num)
        });
        rps_matrices.resize_with(rps_degree, || {
            SparseMatrixBuilder::<R>::new(constraints_num, variables_num)
        });

        self.lay_out();
        #[allow(clippy::needless_range_loop)]
        for constraint in constraints {
            let (lps_span, rps_span) = (constraint.lps.span(), constraint.rps.span());
            for i in 0..lps_span.dimension() {
                self.put(&mut lps_matrices[i], &lps_span[i])
            }
            for i in lps_span.dimension()..lps_degree {
                self.pad(&mut lps_matrices[i]);
            }
            for i in 0..rps_span.dimension() {
                self.put(&mut rps_matrices[i], &rps_span[i])
            }
            for i in rps_span.dimension()..rps_degree {
                self.pad(&mut rps_matrices[i]);
            }
        }

        let mut matrices = Vec::with_capacity(lps_degree + rps_degree);
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

impl<'a, R: Semiring> Display for CircuitBuilder<'a, R> {
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
pub struct Scope<'a, 'b, R: Semiring> {
    builder: &'a CircuitBuilder<'b, R>,
}

impl<'a, 'b, R: Semiring> Scope<'a, 'b, R> {
    /// Build a constraint `lps == rps`.
    ///
    /// # Panics
    ///
    /// If constraint degree is higher than circuit degree.
    pub fn constrain<LPS: Expression<'b, R>, RPS: Expression<'b, R>>(&self, lps: LPS, rps: RPS) {
        self.builder.constrain(Constraint {
            lps: Box::new(lps),
            rps: Box::new(rps),
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

impl<'a, 'b, R: Semiring> Drop for Scope<'a, 'b, R> {
    fn drop(&mut self) {
        let scopes = self.builder.scopes.borrow();
        let mut current_scope = self.builder.current_scope.borrow_mut();
        let node = scopes.get_node(*current_scope).expect("Tree");
        *current_scope = node.parent().expect("Scope").idx();
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
