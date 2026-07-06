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

/// Optional value.
#[derive(Clone, Copy)]
pub struct BlOption<T> {
    value: T,
    is_some: bool,
}

impl<T> BlOption<T> {
    /// Construct a new optional value.
    pub const fn new(value: T, is_some: bool) -> Self {
        Self { value, is_some }
    }

    /// Map applicatively.
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> BlOption<U> {
        BlOption {
            value: f(self.value),
            is_some: self.is_some,
        }
    }

    /// Whether a none value.
    pub const fn is_none(&self) -> bool {
        !self.is_some
    }

    /// Whether some value.
    pub const fn is_some(&self) -> bool {
        self.is_some
    }

    /// Return some value or panic with custom message.
    /// # Panics
    /// On a none value.
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        assert!(self.is_some, "Expectation failed: {msg}");
        self.value
    }

    /// Return some value or panic.
    /// # Panics
    /// On a none value.
    #[track_caller]
    pub fn unwrap(self) -> T {
        assert!(self.is_some, "unwrap on a none value");
        self.value
    }
}

impl<T> From<BlOption<T>> for (T, bool) {
    fn from(option: BlOption<T>) -> Self {
        (option.value, option.is_some)
    }
}
