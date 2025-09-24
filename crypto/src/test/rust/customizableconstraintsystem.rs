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
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::customizableconstraintsystem::CustomizableConstraintSystem;
use blacknet_crypto::matrixdense::MatrixDense;
use blacknet_crypto::matrixsparse::MatrixSparse;
use blacknet_crypto::vectordense::VectorDense;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn satisfaction() {
    let m1 = MatrixDense::new(1, 3, [0, 0, 1].map(R::from).into());
    let m2 = MatrixDense::new(1, 3, [0, 1, 0].map(R::from).into());
    let z = VectorDense::from([1, 16, 2].map(R::from));

    let ccs = CustomizableConstraintSystem::new(
        vec![MatrixSparse::from(&m1), MatrixSparse::from(&m2)],
        vec![vec![0, 0, 0, 0], vec![1]],
        [1, -1].map(R::from).into(),
    );

    assert_eq!(ccs.degree(), 4);
    assert_eq!(ccs.constraints(), 1);
    assert_eq!(ccs.variables(), 3);

    assert_ok!(ccs.is_satisfied(&z));
}
