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

use crate::v2::BlockNotification;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};

#[derive(Deserialize, Serialize)]
pub struct WebSocketNotification {
    route: String,
    message: Value,
}

impl WebSocketNotification {
    pub fn with_block(notification: BlockNotification) -> Self {
        Self {
            route: "block".to_owned(),
            message: to_value(notification).expect("BlockNotification"),
        }
    }
}
