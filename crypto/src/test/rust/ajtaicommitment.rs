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

use blacknet_crypto::ajtaicommitment::AjtaiCommitment;
use blacknet_crypto::matrixdense::MatrixDense;
use blacknet_crypto::ring::IntegerRing;
use blacknet_crypto::vectordense::VectorDense;

type Z = blacknet_crypto::pervushin::PervushinField;

#[test]
fn test() {
    let setup = MatrixDense::new(
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
    let cs = AjtaiCommitment::new(setup);
    #[cfg(feature = "core_float_math")]
    let b_ecd = 7.22;
    let b_inf = 8;
    let z1 = Z::new(1);
    let z2 = Z::new(2);
    let z3 = Z::new(3);
    let z4 = Z::new(4);
    let m12 = VectorDense::from([z1, z2]);
    let m21 = VectorDense::from([z2, z1]);
    let m34 = VectorDense::from([z3, z4]);
    let c12 = cs.commit_dense(&m12);
    let c34 = cs.commit_dense(&m34);

    assert!(cs.open_dense_linf(&c12, &m12, b_inf), "Opening");
    assert!(!cs.open_dense_linf(&c34, &m12, b_inf), "Binding");
    assert!(!cs.open_dense_linf(&c12, &m21, b_inf), "Positional binding");
    assert!(
        cs.open_dense_linf(&(&c12 + &c34), &(&m12 + &m34), b_inf),
        "Homomorphism"
    );

    #[cfg(feature = "core_float_math")]
    assert!(cs.open_dense_l2(&c12, &m12, b_ecd), "Opening");
    #[cfg(feature = "core_float_math")]
    assert!(!cs.open_dense_l2(&c34, &m12, b_ecd), "Binding");
    #[cfg(feature = "core_float_math")]
    assert!(!cs.open_dense_l2(&c12, &m21, b_ecd), "Positional binding");
    #[cfg(feature = "core_float_math")]
    assert!(
        cs.open_dense_l2(&(&c12 + &c34), &(&m12 + &m34), b_ecd),
        "Homomorphism"
    );
}
