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

use blacknet_compat::assert_ok;
use blacknet_crypto::assigner::distribution::Distribution as DistributionAssigner;
use blacknet_crypto::assigner::quartaryuniformdistribution::QuartaryUniformDistribution as Assigner;
use blacknet_crypto::circuit::circuitbuilder::{CircuitBuilder, Constant, LinearCombination};
use blacknet_crypto::circuit::distribution::Distribution as DistributionCircuit;
use blacknet_crypto::circuit::quartaryuniformdistribution::QuartaryUniformDistribution as Circuit;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::distribution::{Distribution, UniformGenerator};
use blacknet_crypto::quartaryuniformdistribution::QuartaryUniformDistribution;
use blacknet_crypto::ring::IntegerRing;
use core::array;

type Z = blacknet_crypto::lm::LMField;

struct GeneratorPlain {
    i: i16,
}

impl GeneratorPlain {
    fn new() -> Self {
        Self { i: 1234 }
    }
}

impl UniformGenerator for GeneratorPlain {
    type Output = Z;

    fn generate(&mut self) -> Self::Output {
        let result = self.i;
        self.i += 1;
        result.into()
    }
}

#[test]
fn plain_reproducible() {
    let mut g = GeneratorPlain::new();
    let mut qud = QuartaryUniformDistribution::<GeneratorPlain>::default();
    let a: [Z; 16] = [-1, 0, 2, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(Z::from);
    let b: [Z; 16] = array::from_fn(|_| qud.sample(&mut g));
    assert_eq!(b, a);
}

struct GeneratorCircuit {
    i: Z,
}

impl GeneratorCircuit {
    fn new() -> Self {
        Self { i: 1234.into() }
    }
}

impl UniformGenerator for GeneratorCircuit {
    type Output = LinearCombination<Z>;

    fn generate(&mut self) -> Self::Output {
        let result = Constant::new(self.i);
        self.i += Z::new(1);
        result.into()
    }
}

#[test]
fn circuit_reproducible() {
    let a_plain: [Z; 16] = [-1, 0, 2, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(Z::from);

    let circuit = CircuitBuilder::<Z>::new(2);
    let scope = circuit.scope("test");
    let mut g_circuit = GeneratorCircuit::new();
    let mut qud_circuit = Circuit::<Z, GeneratorCircuit>::new(&circuit);
    let _a_circuit: [LinearCombination<Z>; 16] =
        array::from_fn(|_| qud_circuit.sample(&mut g_circuit));
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();

    let mut g_assigner = GeneratorPlain::new();
    let mut qud_assigner = Assigner::<GeneratorPlain>::new(&z);
    let a_assigned: [Z; 16] = array::from_fn(|_| qud_assigner.sample(&mut g_assigner));

    assert_eq!(a_assigned, a_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}
