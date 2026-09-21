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

use blacknet_compat::feerate::FeeRate;
use blacknet_network::packet::FeeFilter;
use blacknet_serialization::{from_bytes, to_bytes};

#[test]
fn serialization() {
    let fee_rate = FeeRate::from(0x0000000004030201);
    let fee_filter = FeeFilter::new(fee_rate);
    let bytes: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x04, 0x03, 0x02, 0x01];

    let deserialized = from_bytes::<FeeFilter>(bytes, false).unwrap();
    assert_eq!(deserialized.fee_rate(), fee_rate);

    let serialized = to_bytes::<FeeFilter>(&fee_filter).unwrap();
    assert_eq!(serialized, bytes);
}
