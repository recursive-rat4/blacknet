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

use blacknet_rpc::v3::authentication::CookieAuthParams;
use core::{array, assert_matches};
use serde_json::{Map, Value, from_str, from_value, to_string, to_value};

#[test]
fn cookie_auth_params() {
    let string = r#"{"cookie":"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F"}"#;
    let object = CookieAuthParams {
        cookie: array::from_fn(|i| i as u8),
    };
    let mut map = Map::new();
    map.insert(
        "cookie".into(),
        "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F".into(),
    );
    let value = Value::Object(map);

    assert_matches!(from_str::<CookieAuthParams>(string), Ok(x) if x == object);
    assert_matches!(from_value::<CookieAuthParams>(value.clone()), Ok(x) if x == object);
    assert_matches!(to_string(&object), Ok(x) if x == string);
    assert_matches!(to_value(&object), Ok(x) if x == value);
}
