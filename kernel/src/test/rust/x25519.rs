/*
 * Copyright (c) 2020-2026 Pavel Vasin
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

use blacknet_kernel::{
    ed25519::{PublicKey, to_secret_key},
    x25519::*,
};
use core::str::FromStr;

#[test]
fn shared_key() {
    let mnemonic1 = "疗 昨 示 穿 偏 贷 五 袁 色 烂 撒 殖";
    let secret_key1 = to_secret_key(mnemonic1).unwrap();
    let public_key1 =
        PublicKey::from_str("27A2C7CE9EE9AF0458832079017A5FBBB1F1551932C4CB901396BAE95F7D0F0A")
            .unwrap();
    let mnemonic2 = "胡 允 空 桥 料 状 纱 角 钠 灌 绝 件";
    let secret_key2 = to_secret_key(mnemonic2).unwrap();
    let public_key2 =
        PublicKey::from_str("A65AEF3E4128031285BF0367832C38AD1366A1E8D5E395BCDC7A17C3B28BAB1D")
            .unwrap();

    assert_eq!(
        x25519(&secret_key1, public_key2).unwrap().as_ref(),
        x25519(&secret_key2, public_key1).unwrap().as_ref()
    );
}
