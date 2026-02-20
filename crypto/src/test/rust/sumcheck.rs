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

use blacknet_compat::{assert_err, assert_ok};
use blacknet_crypto::assigner::polynomial::UnivariatePolynomial as UnivariatePolynomialAssigner;
use blacknet_crypto::assigner::poseidon2pervushin::DuplexPoseidon2Pervushin as DuplexPoseidon2PervushinAssigner;
use blacknet_crypto::assigner::sumcheck::{Proof as ProofAssigner, SumCheck as SumCheckAssigner};
use blacknet_crypto::circuit::builder::{CircuitBuilder, VariableKind};
use blacknet_crypto::circuit::poseidon2pervushin::DuplexPoseidon2Pervushin as DuplexPoseidon2PervushinCircuit;
use blacknet_crypto::circuit::sumcheck::{Proof as ProofCircuit, SumCheck as SumCheckCircuit};
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::duplex::Duplex;
use blacknet_crypto::pervushin::PervushinField;
use blacknet_crypto::polynomial::{
    EqExtension, MultilinearExtension, MultivariatePolynomial, Polynomial,
};
use blacknet_crypto::poseidon2pervushin::DuplexPoseidon2Pervushin as DuplexPoseidon2PervushinPlain;
use blacknet_crypto::random::{Distribution, UniformDistribution};
use blacknet_crypto::sumcheck::{Proof as ProofPlain, SumCheck as SumCheckPlain};

type Z = PervushinField;
type D = DuplexPoseidon2PervushinPlain;
type E = UniformDistribution<D>;

#[test]
fn mle() {
    type SC = SumCheckPlain<Z, MultilinearExtension<Z>, D, E>;
    let mut duplex = D::default();
    let mut exceptional_set = E::default();

    let p1 = MultilinearExtension::from([7, 7, 7, 0].map(Z::from));
    let p2 = MultilinearExtension::from([7, 7, 7, 7].map(Z::from));
    let p3 = MultilinearExtension::from([7, 7, 0, 7].map(Z::from));
    let s1 = Z::from(21);
    let s2 = Z::from(28);

    let proof = SC::prove(p1.clone(), s1, &mut duplex, &mut exceptional_set);
    duplex.reset();
    exceptional_set.reset();

    assert_ok!(SC::verify(
        &p1,
        s1,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p1,
        s2,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p2,
        s1,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p2,
        s2,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p3,
        s1,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    let proof2 = SC::prove(p1.clone(), s2, &mut duplex, &mut exceptional_set);
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p1,
        s1,
        &proof2,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p1,
        s2,
        &proof2,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    let proof3 = ProofPlain::default();

    assert_err!(SC::verify(
        &p1,
        s1,
        &proof3,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();
}

#[test]
fn eq() {
    type SC = SumCheckPlain<Z, EqExtension<Z>, D, E>;
    let mut duplex = D::default();
    let mut exceptional_set = E::default();

    let p1 = EqExtension::from([45, 46, 47, 48].map(Z::from));
    let p2 = EqExtension::from([45, 46, 48, 48].map(Z::from));
    let s1 = Z::from(1);
    let s2 = Z::from(2);

    let proof = SC::prove(p1.clone(), s1, &mut duplex, &mut exceptional_set);
    duplex.reset();
    exceptional_set.reset();

    assert_ok!(SC::verify(
        &p1,
        s1,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p1,
        s2,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p2,
        s1,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p2,
        s2,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    let proof2 = SC::prove(p1.clone(), s2, &mut duplex, &mut exceptional_set);
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p1,
        s1,
        &proof2,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify(
        &p1,
        s2,
        &proof2,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    let proof3 = ProofPlain::default();

    assert_err!(SC::verify(
        &p1,
        s1,
        &proof3,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();
}

#[test]
fn early_stopping() {
    type SC = SumCheckPlain<Z, EqExtension<Z>, D, E>;
    let mut duplex = D::default();
    let mut exceptional_set = E::default();

    let p1 = EqExtension::from([45, 46, 47, 48].map(Z::from));
    let p2 = EqExtension::from([45, 46, 48, 48].map(Z::from));
    let s1 = Z::from(1);
    let s2 = Z::from(2);

    let proof = SC::prove(p1.clone(), s1, &mut duplex, &mut exceptional_set);
    duplex.reset();
    exceptional_set.reset();

    let maybe = SC::verify_early_stopping(&p1, s1, &proof, &mut duplex, &mut exceptional_set);
    assert_ok!(&maybe);
    let (point, state) = maybe.unwrap();
    assert_eq!(state, p1.point(&point));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify_early_stopping(
        &p1,
        s2,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify_early_stopping(
        &p2,
        s2,
        &proof,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    let proof2 = SC::prove(p1.clone(), s2, &mut duplex, &mut exceptional_set);
    duplex.reset();
    exceptional_set.reset();

    assert_err!(SC::verify_early_stopping(
        &p1,
        s1,
        &proof2,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();

    let proof3 = ProofPlain::default();

    assert_err!(SC::verify(
        &p1,
        s1,
        &proof3,
        &mut duplex,
        &mut exceptional_set
    ));
    duplex.reset();
    exceptional_set.reset();
}

#[test]
fn circuit() {
    type SCPlain = SumCheckPlain<Z, MultilinearExtension<Z>, D, E>;
    let mut duplex_plain = D::default();
    let mut exceptional_set_plain = E::default();

    let poly_plain = MultilinearExtension::from([7, 7, 7, 0].map(Z::from));
    let sum_plain = Z::from(21);
    let proof_plain = SCPlain::prove(
        poly_plain.clone(),
        sum_plain,
        &mut duplex_plain,
        &mut exceptional_set_plain,
    );
    duplex_plain.reset();
    exceptional_set_plain.reset();
    let (point_plain, state_plain) = SCPlain::verify_early_stopping(
        &poly_plain,
        sum_plain,
        &proof_plain,
        &mut duplex_plain,
        &mut exceptional_set_plain,
    )
    .unwrap();

    let circuit = CircuitBuilder::<Z>::new(2);
    let scope = circuit.scope("test");
    let sum_circuit = scope.public_input();
    let proof_circuit = ProofCircuit::allocate(
        &circuit,
        VariableKind::PublicInput,
        poly_plain.variables(),
        poly_plain.degree(),
    );
    type DuplexCircuit<'a, 'b> = DuplexPoseidon2PervushinCircuit<'a, 'b>;
    let mut duplex_circuit = DuplexCircuit::new(&circuit);
    type UniformDistributionCircuit<'a, 'b> = UniformDistribution<DuplexCircuit<'a, 'b>>;
    let mut exceptional_set_circuit = UniformDistributionCircuit::default();
    type SCCircuit<'a, 'b> = SumCheckCircuit<
        'a,
        'b,
        Z,
        MultilinearExtension<Z>,
        DuplexCircuit<'a, 'b>,
        UniformDistributionCircuit<'a, 'b>,
    >;
    let sumcheck_circuit = SCCircuit::new(&circuit);
    sumcheck_circuit.verify_early_stopping(
        &poly_plain,
        sum_circuit.into(),
        &proof_circuit,
        &mut duplex_circuit,
        &mut exceptional_set_circuit,
    );
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.push(sum_plain);
    z.extend((&proof_plain).into_iter().flatten().copied());

    let proof_assigner = ProofAssigner::from(
        (&proof_plain)
            .into_iter()
            .map(|i| UnivariatePolynomialAssigner::new(i.coefficients().to_owned(), &z))
            .collect::<Vec<_>>(),
    );
    type DuplexAssigner<'a> = DuplexPoseidon2PervushinAssigner<'a>;
    let mut duplex_assigner = DuplexAssigner::new(&z);
    type UniformDistributionAssigner<'a> = UniformDistribution<DuplexAssigner<'a>>;
    let mut exceptional_set_assigner = UniformDistributionAssigner::default();
    type SCAssigner<'a> = SumCheckAssigner<
        'a,
        Z,
        MultilinearExtension<Z>,
        DuplexAssigner<'a>,
        UniformDistributionAssigner<'a>,
    >;
    let sumcheck_assigner = SCAssigner::new(&z);

    let (point_assigned, state_assigned) = sumcheck_assigner.verify_early_stopping(
        &poly_plain,
        sum_plain,
        &proof_assigner,
        &mut duplex_assigner,
        &mut exceptional_set_assigner,
    );
    assert_eq!(point_assigned, point_plain);
    assert_eq!(state_assigned, state_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}
