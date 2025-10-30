/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use crate::connection::Connection;
use crate::packet::{
    Blocks, ConsensusFault, MAX_BLOCKS, MAX_HASHES, PACKET_HEADER_SIZE_BYTES, Packet,
};
use blacknet_kernel::blake2b::Hash;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetBlocks {
    best: Hash,
    checkpoint: Hash,
}

impl Packet for GetBlocks {
    fn handle(self, connection: &mut Connection) {
        let node = connection.node();
        let block_db = node.block_db();
        if let Some((previous_hash, bytes)) = &**block_db.cached_block()
            && self.best == *previous_hash
        {
            connection.send_packet(Blocks::with_block(bytes));
            return;
        }

        if let Some(mut block_index) = block_db.index(self.best) {
            let mut size = PACKET_HEADER_SIZE_BYTES + 2 + 1;
            let max_size = node.min_packet_size(); // actual value is unknown, minimum is assumed
            let mut response = Vec::<Box<[u8]>>::with_capacity(MAX_BLOCKS);

            loop {
                let hash = block_index.next();
                if hash == Hash::default() {
                    break;
                }
                size += block_index.next_size() + 4; //XXX VarInt.size()
                if !response.is_empty() && size >= max_size {
                    break;
                }
                if let Some(bytes) = block_db.get_raw(hash) {
                    response.push(bytes);
                } else {
                    break;
                }
                if response.len() == MAX_BLOCKS {
                    break;
                }
                if let Some(next_index) = block_db.index(hash) {
                    block_index = next_index;
                } else {
                    break;
                }
            }

            connection.send_packet(Blocks::with_blocks(response));
        } else if let Some(next_block_hashes) =
            block_db.next_block_hashes(self.checkpoint, MAX_HASHES)
        {
            connection.send_packet(Blocks::with_hashes(next_block_hashes));
        } else {
            connection.send_packet(ConsensusFault);
            connection.dos("Consensus fault");
        }
    }
}
