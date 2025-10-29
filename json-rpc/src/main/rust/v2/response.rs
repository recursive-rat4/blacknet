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

use axum::response::Response;
use data_encoding::HEXUPPER;
use serde::Serialize;
use serde_json::to_string;

pub fn respond_hex(message: &[u8]) -> Response<String> {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(HEXUPPER.encode(message))
        .unwrap()
}

pub fn respond_text(message: &str) -> Response<String> {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(message.to_owned())
        .unwrap()
}

pub fn respond_json<T: Serialize>(message: &T) -> Response<String> {
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(to_string(message).unwrap())
        .unwrap()
}

pub fn respond_error(message: &str) -> Response<String> {
    Response::builder()
        .status(400)
        .header("Content-Type", "text/plain")
        .body(message.to_owned())
        .unwrap()
}
