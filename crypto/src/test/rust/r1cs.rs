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

use blacknet_crypto::matrixdense::MatrixDense;
use blacknet_crypto::matrixsparse::MatrixSparse;
use blacknet_crypto::r1cs::R1CS;
use blacknet_crypto::vectordense::VectorDense;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn satisfaction() {
    #[rustfmt::skip]
    let a = MatrixDense::new(3, 5, [
        0, 0, 1, 0, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 0, 1,
    ].map(R::from).into());
    #[rustfmt::skip]
    let b = MatrixDense::new(3, 5, [
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 0, 1,
    ].map(R::from).into());
    #[rustfmt::skip]
    let c = MatrixDense::new(3, 5, [
        4, 1, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 1, 0,
    ].map(R::from).into());
    let z = VectorDense::from([1, 60, 16, 4, 2].map(R::from));
    let r1cs = R1CS::new(
        MatrixSparse::from(&a),
        MatrixSparse::from(&b),
        MatrixSparse::from(&c),
    );
    assert!(r1cs.is_satisfied(&z));
}
