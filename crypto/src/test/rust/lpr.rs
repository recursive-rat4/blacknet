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

use blacknet_crypto::lpr::*;
use blacknet_crypto::random::FastDRG;
use core::array;

#[test]
fn crypt() {
    let bytes: [u8; 128] = array::from_fn(|i| i as u8);
    let pt = encode(&bytes);
    let mut drg = FastDRG::default();
    let sk = generate_secret_key(&mut drg);
    let pk = generate_public_key(&mut drg, &sk);
    let ct = encrypt(&mut drg, &pk, &pt);
    let decrypted = decrypt(&sk, &ct);
    let decoded = decode(&decrypted);
    assert_eq!(decoded, bytes);
}
