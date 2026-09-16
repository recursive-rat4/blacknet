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

use blacknet_json_rpc::Error;
use core::assert_matches;
use serde_json::{from_str, to_string};

#[test]
fn error() {
    let parse_str = r#"{"code":-32700,"message":"Parse error"}"#;
    let application_str =
        r#"{"code":-1,"message":"Application error","data":"Something went wrong"}"#;
    let parse = Error::parse_error(Option::<()>::None);
    let application = Error::application_error(Some("Something went wrong"));
    let bad_str = r#"{"code":0.777,"message":"Custom error"}"#;

    assert_matches!(to_string(&parse).as_deref(), Ok(x) if x == parse_str);
    assert_matches!(to_string(&application).as_deref(), Ok(x) if x == application_str);

    assert_matches!(from_str::<Error>(parse_str), Ok(x) if x == parse);
    assert_matches!(from_str::<Error>(application_str), Ok(x) if x == application);
    assert_matches!(from_str::<Error>(bad_str), Err(_));
}
