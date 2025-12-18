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

use blacknet_compat::assert_ok;
use blacknet_crypto::assigner::poseidon2::Poseidon2Assigner;
use blacknet_crypto::circuit::builder::{CircuitBuilder, LinearCombination};
use blacknet_crypto::circuit::poseidon2::Poseidon2Circuit;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::lm::LMField;
use blacknet_crypto::pervushin::PervushinField;
use blacknet_crypto::poseidon2::Poseidon2Plain;
use blacknet_crypto::poseidon2lm::{Poseidon2LM8, Poseidon2LM12};
use blacknet_crypto::poseidon2pervushin::{Poseidon2Pervushin8, Poseidon2Pervushin12};
use blacknet_crypto::ring::IntegerRing;
use core::array;
use core::iter::zip;

#[test]
fn lm_12() {
    type F = LMField;
    const W: usize = 12;

    let a_plain = [
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
        0x0000000000000008,
        0x0000000000000009,
        0x000000000000000a,
        0x000000000000000b,
    ]
    .map(F::new);
    let b_plain = [
        0x0833b3a9b92da868,
        0x0534198f3736b18f,
        0x0fba68d9c3b75576,
        0x0de6fe6328030c0c,
        0x0502283ddfbdee88,
        0x0b6e32169cf84d8e,
        0x04d7c5bbd4a00d35,
        0x0db427abb558c484,
        0x0957d084081aa120,
        0x0b4ca9b07051716e,
        0x0fdfa4c346c6bf96,
        0x01198b4d4e4a9785,
    ]
    .map(F::new);
    let mut c_plain = a_plain;
    <Poseidon2LM12 as Poseidon2Plain<F, W, 48, 26, 48>>::permute(&mut c_plain);
    assert_eq!(c_plain, b_plain);

    let circuit = CircuitBuilder::<F>::new(2);
    let scope = circuit.scope("test");
    let a_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let b_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let mut c_circuit = a_circuit.clone();
    <Poseidon2LM12 as Poseidon2Circuit<F, W, 48, 26, 48>>::permute(&circuit, &mut c_circuit);
    zip(c_circuit, b_circuit).for_each(|(c, b)| scope.constrain(c, b));
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a_plain);
    z.extend(b_plain);

    let mut c_assigned = a_plain.clone();
    <Poseidon2LM12 as Poseidon2Assigner<F, W, 48, 26, 48>>::permute(&z, &mut c_assigned);

    assert_eq!(c_assigned, c_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn lm_8() {
    type F = LMField;
    const W: usize = 8;

    let a_plain = [
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
    ]
    .map(F::new);
    let b_plain = [
        0x0bd8c4c53bf8fe12,
        0x0355600cb28c7f23,
        0x016615f801d1ff3d,
        0x0cf8353287cc7856,
        0x0643290ff5147a9e,
        0x0b6eec284395b6cd,
        0x0003d9655be21aaf,
        0x08d63701b7a44777,
    ]
    .map(F::new);
    let mut c_plain = a_plain;
    <Poseidon2LM8 as Poseidon2Plain<F, W, 32, 26, 32>>::permute(&mut c_plain);
    assert_eq!(c_plain, b_plain);

    let circuit = CircuitBuilder::<F>::new(2);
    let scope = circuit.scope("test");
    let a_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let b_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let mut c_circuit = a_circuit.clone();
    <Poseidon2LM8 as Poseidon2Circuit<F, W, 32, 26, 32>>::permute(&circuit, &mut c_circuit);
    zip(c_circuit, b_circuit).for_each(|(c, b)| scope.constrain(c, b));
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a_plain);
    z.extend(b_plain);

    let mut c_assigned = a_plain.clone();
    <Poseidon2LM8 as Poseidon2Assigner<F, W, 32, 26, 32>>::permute(&z, &mut c_assigned);

    assert_eq!(c_assigned, c_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn pervushin_12() {
    type F = PervushinField;
    const W: usize = 12;

    let a_plain = [
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
        0x0000000000000008,
        0x0000000000000009,
        0x000000000000000a,
        0x000000000000000b,
    ]
    .map(F::new);
    let b_plain = [
        0x14ad43d6b732aa1f,
        0x02fbf1c807dd0281,
        0x13e01fc66d9b3d03,
        0x11a1f9de5bad75f6,
        0x18fda95519465f5e,
        0x1a5e99d9a41fe4ce,
        0x1e16576275d7822c,
        0x1106a6eafa54ef7f,
        0x1d5353e179478d6d,
        0x09312dc75ae6f2b3,
        0x1d00514d0694390a,
        0x03f39f82fb43ef6c,
    ]
    .map(F::new);
    let mut c_plain = a_plain;
    <Poseidon2Pervushin12 as Poseidon2Plain<F, W, 48, 12, 48>>::permute(&mut c_plain);
    assert_eq!(c_plain, b_plain);

    let circuit = CircuitBuilder::<F>::new(2);
    let scope = circuit.scope("test");
    let a_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let b_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let mut c_circuit = a_circuit.clone();
    <Poseidon2Pervushin12 as Poseidon2Circuit<F, W, 48, 12, 48>>::permute(&circuit, &mut c_circuit);
    zip(c_circuit, b_circuit).for_each(|(c, b)| scope.constrain(c, b));
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a_plain);
    z.extend(b_plain);

    let mut c_assigned = a_plain.clone();
    <Poseidon2Pervushin12 as Poseidon2Assigner<F, W, 48, 12, 48>>::permute(&z, &mut c_assigned);

    assert_eq!(c_assigned, c_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn pervushin_8() {
    type F = PervushinField;
    const W: usize = 8;

    let a_plain = [
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
    ]
    .map(F::new);
    let b_plain = [
        0x1a8775be9bdb5c86,
        0x084e734b4eba7e69,
        0x0bcf6bc15f7f1390,
        0x165e2e00b93ba0e8,
        0x03bc7c10d705afaa,
        0x05a6da6c5b1c7a16,
        0x0aab068f99aec08b,
        0x1d231eb4c9e7dcdd,
    ]
    .map(F::new);
    let mut c_plain = a_plain;
    <Poseidon2Pervushin8 as Poseidon2Plain<F, W, 32, 12, 32>>::permute(&mut c_plain);
    assert_eq!(c_plain, b_plain);

    let circuit = CircuitBuilder::<F>::new(2);
    let scope = circuit.scope("test");
    let a_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let b_circuit: [LinearCombination<F>; W] = array::from_fn(|_| scope.public_input().into());
    let mut c_circuit = a_circuit.clone();
    <Poseidon2Pervushin8 as Poseidon2Circuit<F, W, 32, 12, 32>>::permute(&circuit, &mut c_circuit);
    zip(c_circuit, b_circuit).for_each(|(c, b)| scope.constrain(c, b));
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a_plain);
    z.extend(b_plain);

    let mut c_assigned = a_plain.clone();
    <Poseidon2Pervushin8 as Poseidon2Assigner<F, W, 32, 12, 32>>::permute(&z, &mut c_assigned);

    assert_eq!(c_assigned, c_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}
