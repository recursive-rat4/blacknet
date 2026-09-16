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

use blacknet_json_rpc::Params;
use core::assert_matches;
use serde_json::{Map, Value, from_str, to_string};

#[test]
fn params() {
    let one_two_three: Vec<Value> = [1, 2, 3].into_iter().map(Into::into).collect();
    let foo_bar: Map<String, Value> = [("foo", "bar")]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect();

    assert_matches!(
        from_str::<Params>("[1,2,3]"),
        Ok(Params::Positional(x)) if x == one_two_three
    );
    assert_matches!(
        from_str::<Params>(r#"{"foo":"bar"}"#),
        Ok(Params::Named(x)) if x == foo_bar
    );
    assert_matches!(from_str::<Params>(r#""baz""#), Err(_));

    assert_matches!(
        to_string(&Params::Positional(one_two_three)).as_deref(),
        Ok("[1,2,3]")
    );
    assert_matches!(
        to_string(&Params::Named(foo_bar)).as_deref(),
        Ok(r#"{"foo":"bar"}"#)
    );
}
