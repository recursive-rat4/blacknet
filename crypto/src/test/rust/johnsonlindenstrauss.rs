/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use blacknet_crypto::johnsonlindenstrauss::JohnsonLindenstrauss;
use blacknet_crypto::matrix::DenseVector;
use blacknet_crypto::norm::EuclideanNorm;

type Z = blacknet_crypto::pervushin::PervushinField;
type DRG = blacknet_crypto::poseidon2pervushin::DuplexPoseidon2Pervushin;

#[test]
fn test() {
    let slack_min = 5.0;
    let slack_max = 19.0;
    let high: DenseVector<Z> = [100, 200, 300, 400, 500, 600, 700, 800].map(Z::from).into();

    let mut drg = DRG::default();
    let jl = JohnsonLindenstrauss::<Z>::new(&mut drg, high.dimension());
    let low = jl.project(&high);

    assert!(low.euclidean_norm() > high.euclidean_norm() * slack_min);
    assert!(low.euclidean_norm() < high.euclidean_norm() * slack_max);
}
