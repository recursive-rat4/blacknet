/*
 * Copyright (c) 2018-2026 Pavel Vasin
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

use blacknet_compat::Mode;
use blacknet_crypto::symmetric::Blake2b256;
use blacknet_kernel::{
    blake2b::Hash256,
    ed25519::{PublicKey, SecretKey, Signature, sign, verify},
    error::Result,
};

const SIGN_MAGIC: &str = " Signed Message:\n";

pub fn sign_message(mode: &Mode, secret_key: &SecretKey, message: &str) -> Signature {
    sign(hash(mode, message), secret_key)
}

pub fn verify_message(
    mode: &Mode,
    public_key: PublicKey,
    signature: Signature,
    message: &str,
) -> Result<()> {
    verify(signature, hash(mode, message), public_key)
}

fn hash(mode: &Mode, message: &str) -> Hash256 {
    let mut hasher = Blake2b256::new();
    hasher.update(mode.message_sign_name());
    hasher.update(SIGN_MAGIC);
    hasher.update(message);
    hasher.finalize().into()
}
