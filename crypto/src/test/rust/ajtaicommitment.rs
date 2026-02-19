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

use blacknet_crypto::ajtaicommitment::AjtaiCommitment;
use blacknet_crypto::algebra::IntegerRing;
use blacknet_crypto::commitmentscheme::CommitmentScheme;
use blacknet_crypto::matrix::{DenseMatrix, DenseVector};
use blacknet_crypto::norm::{L2, LInf, NormBound};

type Z = blacknet_crypto::pervushin::PervushinField;

#[test]
fn test() {
    let setup = DenseMatrix::new(
        2,
        2,
        [
            0x27266DA3020839D0u64 as i64,
            0x82C5945C2F27D053u64 as i64,
            0xCBC815FE12387BF6u64 as i64,
            0xC006EACA4C2362E9u64 as i64,
        ]
        .map(Z::new)
        .into(),
    );
    let b_ecd = NormBound::<L2, f64>::new(7.22);
    let b_inf = NormBound::<LInf, <Z as IntegerRing>::Int>::new(8);
    let cs_ecd = AjtaiCommitment::new(setup.clone(), b_ecd);
    let cs_inf = AjtaiCommitment::new(setup, b_inf);
    let z1 = Z::new(1);
    let z2 = Z::new(2);
    let z3 = Z::new(3);
    let z4 = Z::new(4);
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
        "Homomorphism"
    );

    assert!(cs_ecd.open(&c12, &m12, &()), "Opening");
    assert!(!cs_ecd.open(&c34, &m12, &()), "Binding");
    assert!(!cs_ecd.open(&c12, &m21, &()), "Positional binding");
    assert!(
        cs_ecd.open(&(&c12 + &c34), &(&m12 + &m34), &()),
        "Homomorphism"
    );
}
