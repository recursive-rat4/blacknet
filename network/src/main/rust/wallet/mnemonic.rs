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

use crate::wallet::Wordlist;
use blacknet_crypto::{
    random::{Distribution, StrongDRG, UniformIntDistribution, seed},
    zeroize::{Zeroizing, ZeroizingString},
};
use blacknet_kernel::ed25519::to_secret_key;
use core::fmt;

const WORDS: usize = 12;

pub struct Mnemonic {
    string: ZeroizingString,
}

impl Mnemonic {
    pub const fn new(string: ZeroizingString) -> Self {
        Self { string }
    }

    pub fn generate_v1(lang: &str) -> Result<Self, Error> {
        let wordlist = Wordlist::by_name(lang).ok_or(Error::Wordlist)?;
        let mut rng = Zeroizing::new(seed::<StrongDRG>());
        let mut uid = UniformIntDistribution::<u16>::new(0..wordlist.len());
        let mut string = ZeroizingString::with_capacity(WORDS * wordlist.max_word_size());
        loop {
            Self::generate(&mut string, wordlist, &mut rng, &mut uid);
            if to_secret_key(&string).is_some() {
                return Ok(Self { string });
            } else {
                string.clear();
            }
        }
    }

    fn generate(
        string: &mut ZeroizingString,
        wordlist: &Wordlist,
        rng: &mut StrongDRG,
        uid: &mut UniformIntDistribution<u16>,
    ) {
        for i in 0..WORDS {
            let idx = uid.sample(rng);
            let word = &wordlist[idx];
            string.push_str(word);
            if i < WORDS - 1 {
                string.push(' ');
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.string.into_inner().into_bytes()
    }

    #[inline]
    #[must_use]
    pub fn into_zeroizing_string(self) -> ZeroizingString {
        self.string
    }
}

impl From<&str> for Mnemonic {
    fn from(string: &str) -> Self {
        let string = String::from(string);
        let string = ZeroizingString::from(string);
        Self { string }
    }
}

impl fmt::Debug for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mnemonic").finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub enum Error {
    Wordlist,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wordlist => f.write_str("Unknown wordlist"),
        }
    }
}

impl core::error::Error for Error {}
