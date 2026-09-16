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

use blacknet_json_rpc::{BatchRequest, BatchResponse, Id, Method, Request, Response};
use core::assert_matches;
use serde_json::{from_str, to_string};

#[test]
fn batch_request() {
    let object_str = r#"{"jsonrpc":"2.0","method":"foo","id":1}"#;
    let object = BatchRequest::Object(Request::new(
        Method::new("foo").unwrap(),
        None,
        Id::number(1),
    ));
    let array_str =
        r#"[{"jsonrpc":"2.0","method":"foo","id":1},{"jsonrpc":"2.0","method":"bar","id":2}]"#;
    let array = BatchRequest::Array(vec![
        Request::new(Method::new("foo").unwrap(), None, Id::number(1)),
        Request::new(Method::new("bar").unwrap(), None, Id::number(2)),
    ]);
    let bad_str = "[1,2,3]";

    assert_matches!(to_string(&object).as_deref(), Ok(x) if x == object_str);
    assert_matches!(to_string(&array).as_deref(), Ok(x) if x == array_str);
    assert_matches!(from_str::<BatchRequest>(object_str), Ok(x) if x == object);
    assert_matches!(from_str::<BatchRequest>(array_str), Ok(x) if x == array);
    assert_matches!(from_str::<BatchRequest>(bad_str), Err(_));
}

#[test]
fn batch_response() {
    let object_str = r#"{"jsonrpc":"2.0","result":2,"id":1}"#;
    let object = BatchResponse::Object(Response::success(2, Id::number(1)));
    let array_str = r#"[{"jsonrpc":"2.0","result":2,"id":1},{"jsonrpc":"2.0","result":3,"id":2}]"#;
    let array = BatchResponse::Array(vec![
        Response::success(2, Id::number(1)),
        Response::success(3, Id::number(2)),
    ]);
    let bad_str = "[1,2,3]";

    assert_matches!(to_string(&object).as_deref(), Ok(x) if x == object_str);
    assert_matches!(to_string(&array).as_deref(), Ok(x) if x == array_str);
    assert_matches!(from_str::<BatchResponse>(object_str), Ok(x) if x == object);
    assert_matches!(from_str::<BatchResponse>(array_str), Ok(x) if x == array);
    assert_matches!(from_str::<BatchResponse>(bad_str), Err(_));
}
