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

use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::proofofstake::*;
use blacknet_time::Seconds;

#[test]
fn next_difficulty_v4() {
    let version = Version::V4;
    let difficulty =
        UInt256::from_hex("00000000000000AFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    let prev_block_time = Seconds::new(1545555600);
    let block_time = Seconds::new(1545556624);
    let next =
        UInt256::from_hex("0000000000000175FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD");
    assert_eq!(
        next_difficulty(version, difficulty, prev_block_time, block_time),
        next
    );
}
