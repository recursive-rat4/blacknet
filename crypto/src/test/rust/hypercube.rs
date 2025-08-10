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

use blacknet_crypto::hypercube::Hypercube;
use blacknet_crypto::point::Point;
use core::iter::zip;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn index() {
    let hypercube = Hypercube::<R>::new(4);
    let indices: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let iter = hypercube.iter_index();
    assert_eq!(iter.len(), indices.len());
    zip(iter, indices).for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn vertex() {
    let hypercube = Hypercube::<R>::new(3);
    let vertices: [Point<R>; 8] = [
        [0, 0, 0].map(R::from).into(),
        [0, 0, 1].map(R::from).into(),
        [0, 1, 0].map(R::from).into(),
        [0, 1, 1].map(R::from).into(),
        [1, 0, 0].map(R::from).into(),
        [1, 0, 1].map(R::from).into(),
        [1, 1, 0].map(R::from).into(),
        [1, 1, 1].map(R::from).into(),
    ];
    let iter = hypercube.iter_vertex();
    assert_eq!(iter.len(), vertices.len());
    zip(iter, vertices).for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn rank2() {
    let hypercube = Hypercube::<R>::new(3);
    #[rustfmt::skip]
    let indices: [(usize, usize); 8] = [
        (0, 0), (0, 1),
        (1, 0), (1, 1),
        (2, 0), (2, 1),
        (3, 0), (3, 1),
    ];
    let iter = hypercube.iter_rank2(4, 2);
    assert_eq!(iter.len(), indices.len());
    zip(iter, indices).for_each(|(a, b)| assert_eq!(a, b));
}
