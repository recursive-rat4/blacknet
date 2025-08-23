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

use blacknet_crypto::assigner::assigment::Assigment;
use blacknet_crypto::assigner::duplex::DuplexImpl as Assigner;
use blacknet_crypto::assigner::permutation::Permutation as PermutationAssigner;
use blacknet_crypto::circuit::circuitbuilder::{CircuitBuilder, Constant, LinearCombination};
use blacknet_crypto::circuit::duplex::DuplexImpl as Circuit;
use blacknet_crypto::circuit::permutation::Permutation as PermutationCircuit;
use blacknet_crypto::duplex::{Duplex, DuplexImpl};
use blacknet_crypto::permutation::Permutation as PermutationPlain;
use blacknet_crypto::ring::UnitalRing;
use core::array;

type Z = blacknet_crypto::pervushin::PervushinField;

#[derive(Clone, Copy)]
struct TestPermutation {}

impl PermutationPlain for TestPermutation {
    type Domain = [Z; 4];

    fn permute(x: &mut Self::Domain) {
        x.iter_mut().for_each(|i| *i += Z::UNITY);
    }
}

type DuplexPlain = DuplexImpl<Z, 2, 2, 4, TestPermutation>;

#[test]
fn plain_blacknet() {
    let mut duplex1 = DuplexPlain::with_iv(&[10, 0].map(Z::from));
    assert_eq!(duplex1.sneak(), [0, 0, 10, 0].map(Z::from));
    duplex1.absorb(&Z::from(2));
    assert_eq!(duplex1.sneak(), [2, 0, 10, 0].map(Z::from));
    duplex1.absorb(&Z::from(4));
    assert_eq!(duplex1.sneak(), [2, 4, 10, 0].map(Z::from));
    duplex1.absorb(&Z::from(6));
    assert_eq!(duplex1.sneak(), [6, 5, 11, 1].map(Z::from));

    let mut duplex2 = duplex1;
    duplex2.absorb(&Z::from(8));
    assert_eq!(duplex2.sneak(), [6, 8, 11, 1].map(Z::from));

    assert_eq!(duplex1.squeeze::<Z>(), Z::from(7));
    assert_eq!(duplex1.sneak(), [7, 2, 12, 4].map(Z::from));
    assert_eq!(duplex1.squeeze::<Z>(), Z::from(2));
    assert_eq!(duplex1.sneak(), [7, 2, 12, 4].map(Z::from));
    assert_eq!(duplex1.squeeze::<Z>(), Z::from(8));
    assert_eq!(duplex1.sneak(), [8, 3, 13, 5].map(Z::from));
    duplex1.absorb(&Z::from(9));
    assert_eq!(duplex1.sneak(), [9, 3, 13, 5].map(Z::from));

    duplex2.absorb(&Z::from(10));
    assert_eq!(duplex2.sneak(), [10, 9, 12, 2].map(Z::from));
    duplex2.absorb(&Z::from(12));
    assert_eq!(duplex2.sneak(), [10, 12, 12, 2].map(Z::from));
    assert_eq!(duplex2.squeeze::<Z>(), Z::from(11));
    assert_eq!(duplex2.sneak(), [11, 13, 13, 4].map(Z::from));

    let mut duplex3 = DuplexPlain::with_iv(&[10, 0].map(Z::from));
    assert_eq!(duplex3.squeeze::<Z>(), Z::from(2));
    assert_eq!(duplex3.sneak(), [2, 1, 11, 3].map(Z::from));
}

impl PermutationCircuit<Z> for TestPermutation {
    type Domain = [LinearCombination<Z>; 4];

    fn permute(_: &CircuitBuilder<Z>, x: &mut Self::Domain) {
        x.iter_mut().for_each(|i| *i += Constant::UNITY);
    }
}

type DuplexCircuit<'a> = Circuit<'a, Z, 2, 2, 4, TestPermutation>;

impl PermutationAssigner<Z> for TestPermutation {
    type Domain = [Z; 4];

    fn permute(_: &Assigment<Z>, x: &mut Self::Domain) {
        x.iter_mut().for_each(|i| *i += Z::UNITY);
    }
}

type DuplexAssigner<'a> = Assigner<'a, Z, 2, 2, 4, TestPermutation>;

#[test]
fn circuit_blacknet() {
    let x_plain: [Z; 3] = [2, 4, 6].map(Z::from);
    let y_plain: [Z; 3] = [7, 2, 8].map(Z::from);

    let circuit = CircuitBuilder::<Z>::new(2);
    let scope = circuit.scope("test");
    let mut duplex_circuit = DuplexCircuit::new(&circuit);
    let x_circuit: [LinearCombination<Z>; 3] = array::from_fn(|_| scope.public_input().into());
    duplex_circuit.absorb(&x_circuit);
    let y_circuit: [LinearCombination<Z>; 3] = duplex_circuit.squeeze();
    scope.constrain(y_circuit[0].clone(), Constant::from(y_plain[0]));
    scope.constrain(y_circuit[1].clone(), Constant::from(y_plain[1]));
    scope.constrain(y_circuit[2].clone(), Constant::from(y_plain[2]));
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(x_plain);

    let mut duplex_assigner = DuplexAssigner::new(&z);
    duplex_assigner.absorb(&x_plain);
    let y_assigned: [Z; 3] = duplex_assigner.squeeze();

    assert_eq!(y_assigned, y_plain);
    assert!(r1cs.is_satisfied(&z.finish()));
}
