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

use core::ops::Index;
use std::sync::LazyLock;

pub struct Wordlist {
    wordlist: Vec<&'static str>,
    max_word_size: usize,
}

impl Wordlist {
    /// # Panics
    ///
    /// On unexpected number of words.
    fn parse(txt: &'static str) -> Self {
        let wordlist: Vec<&'static str> = txt.lines().collect();
        assert!(wordlist.len() == 2048);
        let max_word_size = wordlist.iter().copied().map(str::len).max().unwrap();
        Self {
            wordlist,
            max_word_size,
        }
    }

    pub fn by_name(name: &str) -> Option<&Self> {
        match name {
            "english" => Some(&ENGLISH),
            "chinese_simplified" => Some(&CHINESE_SIMPLIFIED),
            "chinese_traditional" => Some(&CHINESE_TRADITIONAL),
            "italian" => Some(&ITALIAN),
            "korean" => Some(&KOREAN),
            _ => None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    pub const fn len(&self) -> u16 {
        2048
    }

    pub const fn max_word_size(&self) -> usize {
        self.max_word_size
    }
}

impl Index<u16> for Wordlist {
    type Output = str;

    fn index(&self, idx: u16) -> &Self::Output {
        self.wordlist[idx as usize]
    }
}

static ENGLISH: LazyLock<Wordlist> = LazyLock::new(|| {
    Wordlist::parse(include_str!(
        "../../../../../kernel/src/main/resources/bip39/english.txt"
    ))
});
static CHINESE_SIMPLIFIED: LazyLock<Wordlist> = LazyLock::new(|| {
    Wordlist::parse(include_str!(
        "../../../../../kernel/src/main/resources/bip39/chinese_simplified.txt"
    ))
});
static CHINESE_TRADITIONAL: LazyLock<Wordlist> = LazyLock::new(|| {
    Wordlist::parse(include_str!(
        "../../../../../kernel/src/main/resources/bip39/chinese_traditional.txt"
    ))
});
static ITALIAN: LazyLock<Wordlist> = LazyLock::new(|| {
    Wordlist::parse(include_str!(
        "../../../../../kernel/src/main/resources/bip39/italian.txt"
    ))
});
static KOREAN: LazyLock<Wordlist> = LazyLock::new(|| {
    Wordlist::parse(include_str!(
        "../../../../../kernel/src/main/resources/bip39/korean.txt"
    ))
});
