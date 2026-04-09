/*
 * Copyright (c) 2026 Pavel Vasin
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

use blacknet_kernel::ed25519::*;
use data_encoding::HEXUPPER;

#[test]
fn public_key() {
    let mnemonic = "疗 昨 示 穿 偏 贷 五 袁 色 烂 撒 殖";
    let private_key = to_private_key(mnemonic).unwrap();
    let public_key = to_public_key(private_key);
    let bytes: PublicKey = HEXUPPER
        .decode(b"27A2C7CE9EE9AF0458832079017A5FBBB1F1551932C4CB901396BAE95F7D0F0A")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(public_key, bytes);
}
