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

use blacknet_json_rpc::{Id, Method, Params, Request};
use core::assert_matches;
use serde_json::{from_str, to_string};

#[test]
fn request() {
    let positional_str = r#"{"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1}"#;
    let positional = Request::new(
        Method::new("subtract").unwrap(),
        Some(Params::positional([42, 23])),
        Id::number(1),
    );
    let named_str =
        r#"{"jsonrpc":"2.0","method":"subtract","params":{"subtrahend":23,"minuend":42},"id":3}"#;
    let named = Request::new(
        Method::new("subtract").unwrap(),
        Some(Params::named([("subtrahend", 23), ("minuend", 42)])),
        Id::number(3),
    );
    let notification_str = r#"{"jsonrpc":"2.0","method":"update","params":[1,2,3,4,5]}"#;
    let notification = Request::notification(
        Method::new("update").unwrap(),
        Some(Params::positional([1, 2, 3, 4, 5])),
    );
    let bad_1 = r#"{"jsonrpc":"2.0","method":"foobar,"params":"bar","baz]"#;
    let bad_2 = r#"{"jsonrpc":"2.0","method":1,"params":"bar"}"#;

    assert_matches!(to_string(&positional).as_deref(), Ok(x) if x == positional_str);
    assert_matches!(to_string(&notification).as_deref(), Ok(x) if x == notification_str);

    assert_matches!(from_str::<Request>(positional_str), Ok(x) if x == positional);
    assert_matches!(from_str::<Request>(named_str), Ok(x) if x == named);
    assert_matches!(from_str::<Request>(notification_str), Ok(x) if x == notification);

    assert_matches!(from_str::<Request>(bad_1), Err(_));
    assert_matches!(from_str::<Request>(bad_2), Err(_));
}
