/*
 * Copyright (c) 2019-2025 Pavel Vasin
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

use crate::transaction::TxKind;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

pub const MIN_SIZE: usize = 2;
pub const MAX_SIZE: usize = 20;

#[derive(Deserialize, Serialize)]
pub struct Batchee {
    kind: TxKind,
    data: Box<[u8]>,
}

impl Batchee {
    pub const fn kind(&self) -> TxKind {
        self.kind
    }

    pub const fn raw_data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Deserialize, Serialize)]
pub struct Batch {
    multi_data: Box<[Batchee]>,
}

impl Batch {
    pub fn len(&self) -> usize {
        self.multi_data.len()
    }

    pub fn multi_data(&self) -> &[Batchee] {
        &self.multi_data
    }
}
