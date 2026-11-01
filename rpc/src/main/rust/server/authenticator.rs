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

use blacknet_compat::XDGDirectories;
use blacknet_crypto::{
    branchless::BlEq,
    random::{FAST_RNG, UniformGenerator},
};
use blacknet_io::{Error as IoError, Write, file::replace};
use core::{array, error::Error, fmt};

pub struct Authenticator {
    cookie: [u8; 32],
}

impl Authenticator {
    pub fn new(dirs: &XDGDirectories) -> Result<Self, IoError> {
        let cookie = FAST_RNG.with_borrow_mut(|rng| array::from_fn(|_| rng.generate()));
        replace(dirs.state(), "rpc.cookie", |buffered| {
            buffered.write_all(&cookie)
        })?;
        Ok(Self { cookie })
    }

    pub fn verify_cookie(&self, cookie: &[u8; 32]) -> Result<(), AuthError> {
        if self.cookie.bl_eq(cookie) {
            Ok(())
        } else {
            Err(AuthError::Credentials)
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    Credentials,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credentials => f.write_str("Denied authentication credentials"),
        }
    }
}

impl Error for AuthError {}
