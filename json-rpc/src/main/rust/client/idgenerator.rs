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

use crate::Id;
use serde_json::Number;

/// Generator of [Id][crate::Id]s for requests.
///
/// It is not thread-safe and has a period of 2³².
#[derive(Default)]
pub struct IdGenerator {
    next: u32,
}

impl IdGenerator {
    /// Construct new generator.
    pub const fn new() -> Self {
        Self { next: 0 }
    }

    /// Generate a next id.
    pub fn generate(&mut self) -> Id {
        let n = self.next;
        self.next = n.wrapping_add(1);
        Id::Number(Number::from(n))
    }
}
