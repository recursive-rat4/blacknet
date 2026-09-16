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

use blacknet_json_rpc::Method;
use core::assert_matches;
use serde_json::{from_str, to_string};

#[test]
fn method() {
    let method = Method::new("foo").unwrap();
    assert_matches!(Method::new("rpc.foo"), None);
    assert_matches!(to_string(&method).as_deref(), Ok(r#""foo""#));
    assert_matches!(from_str::<Method>(r#""foo""#), Ok(x) if x == method);
    assert_matches!(from_str::<Method>(r#""rpc.foo""#), Err(_));
}
