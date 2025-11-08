/*
 * Copyright (c) 2025 Pavel Vasin
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

use core::hash::Hash;
use hashlink::LinkedHashSet;
use std::hash::RandomState;

pub struct RollingHashSet<T> {
    lhs: LinkedHashSet<T, RandomState>,
    max: usize,
}

impl<T: Eq + Hash> RollingHashSet<T> {
    pub fn new(max: usize) -> Self {
        Self {
            lhs: LinkedHashSet::with_capacity_and_hasher(max, RandomState::new()),
            max,
        }
    }

    pub fn clear(&mut self) {
        self.lhs.clear()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.lhs.contains(value)
    }

    pub fn insert(&mut self, value: T) {
        if self.lhs.len() == self.max {
            self.lhs.pop_front();
        }
        self.lhs.replace(value);
    }

    pub fn is_empty(&self) -> bool {
        self.lhs.is_empty()
    }

    pub fn len(&self) -> usize {
        self.lhs.len()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.lhs.remove(value)
    }
}
