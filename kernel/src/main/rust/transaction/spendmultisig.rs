/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use crate::amount::Amount;
use crate::transaction::{MultiSignatureLockContractId, Sig};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SpendMultisig {
    id: MultiSignatureLockContractId,
    amounts: Box<[Amount]>,
    signatures: Box<[Sig]>,
}
