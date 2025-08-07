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

use blacknet_crypto::module::FreeModule;
use blacknet_crypto::ring::Ring;

type R = blacknet_crypto::fermat::FermatField;
type M = FreeModule<R, 2>;

#[test]
fn right() {
    let r = R::from(3);
    let s = R::from(5);
    let x = M::from([7, 11].map(R::from));
    let y = M::from([13, 17].map(R::from));

    assert_eq!(x * r + y * r, (x + y) * r);
    assert_eq!(x * r + x * s, x * (r + s));
    assert_eq!((x * s) * r, x * (r * s));
    assert_eq!(x, x * R::UNITY);
}
