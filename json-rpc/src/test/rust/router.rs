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

use blacknet_json_rpc::{Method, peer::Route, server::Router};
use core::assert_matches;
use serde::Deserialize;

#[derive(Deserialize)]
struct Ping;

impl Route for Ping {
    type Result = &'static str;
    type Error = ();

    const METHOD: Method = Method::new("ping");
}

#[derive(Deserialize)]
struct Subtract {
    minuend: i32,
    subtrahend: i32,
}

impl Route for Subtract {
    type Result = i32;
    type Error = ();

    const METHOD: Method = Method::new("subtract");
}

#[derive(Deserialize)]
struct Sum(Vec<i32>);

impl Route for Sum {
    type Result = i32;
    type Error = ();

    const METHOD: Method = Method::new("sum");
}

#[derive(Deserialize)]
struct Baz([String; 1]);

impl Route for Baz {
    type Result = ();
    type Error = String;

    const METHOD: Method = Method::new("baz");
}

#[test]
fn router() {
    let state = 0i32;
    let mut router = Router::new();
    router.route::<Ping>(|_, _| Ok("pong")).unwrap();
    router
        .route::<Subtract>(|this, _| Ok(this.minuend - this.subtrahend))
        .unwrap();
    router
        .route::<Sum>(|this, _| Ok(this.0.into_iter().sum()))
        .unwrap();
    router
        .route::<Baz>(|this, state| Err(format!("{}baz {}", this.0[0], state)))
        .unwrap();
    assert!(router.route::<Baz>(|_, _| Ok(())).is_err());

    let request_1_str = r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#;
    let request_2_str = r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"minuend": 42, "subtrahend": 23}, "id": 2}"#;
    let request_3_str = r#"{"jsonrpc": "2.0", "method": "sum", "params": [1, 2, 3, 4], "id": 3}"#;
    let request_4_str = r#"{"jsonrpc": "2.0", "method": "baz", "params": ["foo"], "id": 4}"#;

    let response_1_str = r#"{"jsonrpc":"2.0","result":"pong","id":1}"#;
    let response_2_str = r#"{"jsonrpc":"2.0","result":19,"id":2}"#;
    let response_3_str = r#"{"jsonrpc":"2.0","result":10,"id":3}"#;
    let response_4_str = r#"{"jsonrpc":"2.0","error":{"code":-1,"message":"Application error","data":"foobaz 0"},"id":4}"#;

    assert_matches!(router.handle_str(request_1_str, &state), Ok(Some(x)) if x == response_1_str);
    assert_matches!(router.handle_str(request_2_str, &state), Ok(Some(x)) if x == response_2_str);
    assert_matches!(router.handle_str(request_3_str, &state), Ok(Some(x)) if x == response_3_str);
    assert_matches!(router.handle_str(request_4_str, &state), Ok(Some(x)) if x == response_4_str);
}
