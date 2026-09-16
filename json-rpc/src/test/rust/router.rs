/*
 * Copyright (c) 2023-2026 Pavel Vasin
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

#![cfg(feature = "server")]

use blacknet_json_rpc::{
    Method, Request, Response,
    server::{Handler, Router},
};
use core::{assert_matches, fmt::Display};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
struct Ping;

impl<T> Handler<T> for Ping {
    type Ok = &'static str;
    type Error = ();

    fn handle(self, _: &T) -> Result<Self::Ok, Self::Error> {
        Ok("pong")
    }
}

#[derive(Deserialize)]
struct Subtract {
    minuend: i32,
    subtrahend: i32,
}

impl<T> Handler<T> for Subtract {
    type Ok = i32;
    type Error = ();

    fn handle(self, _: &T) -> Result<Self::Ok, Self::Error> {
        Ok(self.minuend - self.subtrahend)
    }
}

#[derive(Deserialize)]
struct Sum(Vec<i32>);

impl<T> Handler<T> for Sum {
    type Ok = i32;
    type Error = ();

    fn handle(self, _: &T) -> Result<Self::Ok, Self::Error> {
        Ok(self.0.into_iter().sum())
    }
}

#[derive(Deserialize)]
struct Baz([String; 1]);

impl<T: Display> Handler<T> for Baz {
    type Ok = ();
    type Error = String;

    fn handle(self, state: &T) -> Result<Self::Ok, Self::Error> {
        Err(format!("{}baz {}", self.0[0], state))
    }
}

#[test]
fn router() {
    let mut router = Router::new(0i32);
    router.route::<Ping>(Method::new("ping").unwrap()).unwrap();
    router
        .route::<Subtract>(Method::new("subtract").unwrap())
        .unwrap();
    router.route::<Sum>(Method::new("sum").unwrap()).unwrap();
    router.route::<Baz>(Method::new("baz").unwrap()).unwrap();

    let request_1_str = r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#;
    let request_2_str = r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"minuend": 42, "subtrahend": 23}, "id": 2}"#;
    let request_3_str = r#"{"jsonrpc": "2.0", "method": "sum", "params": [1, 2, 3, 4], "id": 3}"#;
    let request_4_str = r#"{"jsonrpc": "2.0", "method": "baz", "params": ["foo"], "id": 4}"#;

    let request_1 = from_str::<Request>(request_1_str).unwrap();
    let request_2 = from_str::<Request>(request_2_str).unwrap();
    let request_3 = from_str::<Request>(request_3_str).unwrap();
    let request_4 = from_str::<Request>(request_4_str).unwrap();

    let response_1_str = r#"{"jsonrpc": "2.0", "result": "pong", "id": 1}"#;
    let response_2_str = r#"{"jsonrpc": "2.0", "result": 19, "id": 2}"#;
    let response_3_str = r#"{"jsonrpc": "2.0", "result": 10, "id": 3}"#;
    let response_4_str = r#"{"jsonrpc": "2.0", "error": { "code": -1, "message": "Application error", "data": "foobaz 0" }, "id": 4}"#;

    let response_1 = from_str::<Response>(response_1_str).unwrap();
    let response_2 = from_str::<Response>(response_2_str).unwrap();
    let response_3 = from_str::<Response>(response_3_str).unwrap();
    let response_4 = from_str::<Response>(response_4_str).unwrap();

    assert_matches!(router.handle(request_1), Some(x) if x == response_1);
    assert_matches!(router.handle(request_2), Some(x) if x == response_2);
    assert_matches!(router.handle(request_3), Some(x) if x == response_3);
    assert_matches!(router.handle(request_4), Some(x) if x == response_4);
}
