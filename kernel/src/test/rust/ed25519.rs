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

use blacknet_compat::{assert_err, assert_ok};
use blacknet_kernel::blake2b::Blake2b256;
use blacknet_kernel::ed25519::*;
use data_encoding::HEXUPPER;
use digest::Digest;

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

#[test]
fn signing() {
    let mnemonic = "疗 昨 示 穿 偏 贷 五 袁 色 烂 撒 殖";
    let private_key = to_private_key(mnemonic).unwrap();
    let message = "Blacknet Signed Message:\nBlacknet test message 2";
    let hash = Blake2b256::digest(message).into();
    let signature = sign(hash, private_key);
    let bytes: Signature = HEXUPPER
        .decode(b"6D5D4F6A81C601B1834701BDE84785470F92DFA517975BED9AAEA035FBDB0072327EFD207195B7202B5A72BB9CC37443A011C35137E1DF1C11BB5E9C60125B04")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(signature, bytes);
}

#[test]
fn verifying() {
    let public_key: PublicKey = HEXUPPER
        .decode(b"27A2C7CE9EE9AF0458832079017A5FBBB1F1551932C4CB901396BAE95F7D0F0A")
        .unwrap()
        .try_into()
        .unwrap();
    let signature: Signature = HEXUPPER
        .decode(b"6D5D4F6A81C601B1834701BDE84785470F92DFA517975BED9AAEA035FBDB0072327EFD207195B7202B5A72BB9CC37443A011C35137E1DF1C11BB5E9C60125B04")
        .unwrap()
        .try_into()
        .unwrap();

    let message = "Blacknet Signed Message:\nBlacknet test message 1";
    let hash = Blake2b256::digest(message).into();
    assert_err!(verify(signature, hash, public_key));

    let message = "Blacknet Signed Message:\nBlacknet test message 2";
    let hash = Blake2b256::digest(message).into();
    assert_ok!(verify(signature, hash, public_key));
}
