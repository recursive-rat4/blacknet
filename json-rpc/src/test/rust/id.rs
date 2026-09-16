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

use blacknet_json_rpc::Id;
use core::assert_matches;
use serde_json::{Number, from_str, to_string};

#[test]
fn id() {
    assert_matches!(to_string(&Id::Null).as_deref(), Ok("null"));
    assert_matches!(to_string(&Id::Number(Number::from(1))).as_deref(), Ok("1"));
    assert_matches!(
        to_string(&Id::String(String::from("a"))).as_deref(),
        Ok(r#""a""#)
    );

    assert_matches!(from_str::<Id>("null"), Ok(Id::Null));
    assert_matches!(
        from_str::<Id>("1"),
        Ok(Id::Number(x)) if x == Number::from(1)
    );
    assert_matches!(
        from_str::<Id>(r#""a""#),
        Ok(Id::String(x)) if x == "a"
    );
    assert_matches!(from_str::<Id>("true"), Err(_));
}
