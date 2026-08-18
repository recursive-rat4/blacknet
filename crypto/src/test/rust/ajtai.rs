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

use blacknet_crypto::ajtai::{AjtaiCommitment, AjtaiHash};
use blacknet_crypto::algebra::IntegerModRing;
use blacknet_crypto::commitmentscheme::{BindingCommitmentScheme, CommitmentScheme};
use blacknet_crypto::matrix::DenseVector;
use blacknet_crypto::norm::{L2, LInf, NormBound};

type Z = blacknet_crypto::uring::U64Ring;
type DRG = blacknet_crypto::random::FastDRG;

#[test]
fn hash() {
    let mut drg = DRG::default();
    let setup = AjtaiHash::<Z, LInf, <Z as IntegerModRing>::Int>::sis(&mut drg, 2, 2);
    let b_ecd = NormBound::<L2, f64>::new(7.22);
    let b_inf = NormBound::<LInf, <Z as IntegerModRing>::Int>::new(8);
    let cs_ecd = AjtaiHash::<Z, L2, f64>::new(setup.clone(), b_ecd);
    let cs_inf = AjtaiHash::<Z, LInf, <Z as IntegerModRing>::Int>::new(setup, b_inf);
    let z1 = Z::from(1);
    let z2 = Z::from(2);
    let z3 = Z::from(3);
    let z4 = Z::from(4);
    let m12 = DenseVector::from([z1, z2]);
    let m21 = DenseVector::from([z2, z1]);
    let m34 = DenseVector::from([z3, z4]);
    let (c12, _) = cs_inf.commit(&m12);
    let (c34, _) = cs_inf.commit(&m34);

    assert!(cs_inf.open(&c12, &m12, &()), "Opening");
    assert!(!cs_inf.open(&c34, &m12, &()), "Binding");
    assert!(!cs_inf.open(&c12, &m21, &()), "Positional binding");
    assert!(
        cs_inf.open(&(&c12 + &c34), &(&m12 + &m34), &()),
        "Bounded homomorphism"
    );

    assert!(cs_ecd.open(&c12, &m12, &()), "Opening");
    assert!(!cs_ecd.open(&c34, &m12, &()), "Binding");
    assert!(!cs_ecd.open(&c12, &m21, &()), "Positional binding");
    assert!(
        cs_ecd.open(&(&c12 + &c34), &(&m12 + &m34), &()),
        "Bounded homomorphism"
    );
}

#[test]
fn commitment() {
    let mut drg = DRG::default();
    let setup = AjtaiCommitment::<Z, LInf, <Z as IntegerModRing>::Int>::sis(&mut drg, 3, 2, 4);
    let b_ecd = NormBound::<L2, f64>::new(7.22);
    let b_inf = NormBound::<LInf, <Z as IntegerModRing>::Int>::new(8);
    let cs_ecd = AjtaiCommitment::<Z, L2, f64>::new(setup.clone(), b_ecd);
    let cs_inf = AjtaiCommitment::<Z, LInf, <Z as IntegerModRing>::Int>::new(setup, b_inf);
    let z1 = Z::from(1);
    let z2 = Z::from(2);
    let z3 = Z::from(3);
    let z4 = Z::from(4);
    let m12 = DenseVector::from([z1, z2]);
    let m21 = DenseVector::from([z2, z1]);
    let m34 = DenseVector::from([z3, z4]);
    let (c12, o12) = cs_inf.commit(&m12, &mut drg);
    let (c34, o34) = cs_inf.commit(&m34, &mut drg);

    assert!(cs_inf.open(&c12, &m12, &o12), "Opening");
    assert!(!cs_inf.open(&c34, &m12, &o34), "Binding");
    assert!(!cs_inf.open(&c12, &m21, &o12), "Positional binding");
    assert!(
        cs_inf.open(&(&c12 + &c34), &(&m12 + &m34), &(&o12 + &o34)),
        "Bounded homomorphism"
    );

    assert!(cs_ecd.open(&c12, &m12, &o12), "Opening");
    assert!(!cs_ecd.open(&c34, &m12, &o34), "Binding");
    assert!(!cs_ecd.open(&c12, &m21, &o12), "Positional binding");
    assert!(
        cs_ecd.open(&(&c12 + &c34), &(&m12 + &m34), &(&o12 + &o34)),
        "Bounded homomorphism"
    );
}
