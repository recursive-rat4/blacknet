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
use blacknet_crypto::algebra::Semiring;
use blacknet_crypto::assigner::assigment::Assigment;
use blacknet_crypto::assigner::compressionfunction::CompressionFunction as CompressionFunctionAssigner;
use blacknet_crypto::assigner::jive::Jive as Assigner;
use blacknet_crypto::assigner::permutation::Permutation as PermutationAssigner;
use blacknet_crypto::circuit::builder::{CircuitBuilder, Constant, LinearCombination};
use blacknet_crypto::circuit::compressionfunction::CompressionFunction as CompressionFunctionCircuit;
use blacknet_crypto::circuit::jive::Jive as Circuit;
use blacknet_crypto::circuit::permutation::Permutation as PermutationCircuit;
use blacknet_crypto::compressionfunction::CompressionFunction;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::jive::Jive;
use blacknet_crypto::permutation::Permutation as PermutationPlain;
use core::array;

type Z = blacknet_crypto::pervushin::PervushinField;

#[derive(Clone, Copy)]
struct TestPermutation {}

impl PermutationPlain for TestPermutation {
    type Domain = [Z; 4];

    fn permute(x: &mut Self::Domain) {
        x.iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e += Z::from(i as i8) + Z::ONE);
    }
}

type JivePlain = Jive<Z, 2, 4, TestPermutation>;

#[test]
fn plain() {
    assert_eq!(
        JivePlain::compress([11, 12].map(Z::from), [13, 14].map(Z::from)),
        [52, 58].map(Z::from)
    );
}

impl PermutationCircuit<Z> for TestPermutation {
    type Domain = [LinearCombination<Z>; 4];

    fn permute(_: &CircuitBuilder<Z>, x: &mut Self::Domain) {
        x.iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e += Constant::new(Z::from(i as i8 + 1)));
    }
}

type JiveCircuit<'a, 'b> = Circuit<'a, 'b, Z, 2, 4, TestPermutation>;

impl PermutationAssigner<Z> for TestPermutation {
    type Domain = [Z; 4];

    fn permute(_: &Assigment<Z>, x: &mut Self::Domain) {
        x.iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e += Z::from(i as i8) + Z::ONE);
    }
}

type JiveAssigner<'a> = Assigner<'a, Z, 2, 4, TestPermutation>;

#[test]
fn circuit() {
    let a_plain: [Z; 2] = [11, 12].map(Z::from);
    let b_plain: [Z; 2] = [13, 14].map(Z::from);
    let c_plain: [Z; 2] = [52, 58].map(Z::from);

    let circuit = CircuitBuilder::<Z>::new(2);
    let scope = circuit.scope("test");
    let jive_circuit = JiveCircuit::new(&circuit);
    let a_circuit: [LinearCombination<Z>; 2] = array::from_fn(|_| scope.public_input().into());
    let b_circuit: [LinearCombination<Z>; 2] = array::from_fn(|_| scope.public_input().into());
    let c_circuit = jive_circuit.compress(&a_circuit, &b_circuit);
    scope.constrain(c_circuit[0].clone(), Constant::new(c_plain[0]));
    scope.constrain(c_circuit[1].clone(), Constant::new(c_plain[1]));
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a_plain);
    z.extend(b_plain);

    let jive_assigner = JiveAssigner::new(&z);
    let c_assigned: [Z; 2] = jive_assigner.compress(a_plain, b_plain);

    assert_eq!(c_assigned, c_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}
