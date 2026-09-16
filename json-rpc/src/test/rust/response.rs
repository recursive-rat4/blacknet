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

use blacknet_json_rpc::{Error, Id, Response};
use core::assert_matches;
use serde_json::{from_str, to_string};

#[test]
fn response() {
    let success_str = r#"{"jsonrpc":"2.0","result":19,"id":1}"#;
    let success = Response::success(19, Id::number(1));
    let error_str =
        r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid Request"},"id":null}"#;
    let error = Response::error(Error::invalid_request(Option::<()>::None), Id::Null);
    let bad_1 = r#"{"jsonrpc":"2.0","result":19,"error":{"code":-32600,"message":"Invalid Request"},"id":1}"#;
    let bad_2 = r#"{"jsonrpc":"2.0","id":1}"#;

    assert_matches!(to_string(&success).as_deref(), Ok(x) if x == success_str);
    assert_matches!(to_string(&error).as_deref(), Ok(x) if x == error_str);

    assert_matches!(from_str::<Response>(success_str), Ok(x) if x == success);
    assert_matches!(from_str::<Response>(error_str), Ok(x) if x == error);

    assert_matches!(from_str::<Response>(bad_1), Err(_));
    assert_matches!(from_str::<Response>(bad_2), Err(_));
}
