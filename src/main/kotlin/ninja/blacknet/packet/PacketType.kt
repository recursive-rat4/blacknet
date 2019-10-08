/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.serialization.KSerializer

enum class PacketType {
    Version,
    Ping,
    Pong,
    GetPeers,
    Peers,
    InventoryV1,
    GetData,
    Data,
    GetBlocks,
    Blocks,
    ChainAnnounce,
    ChainFork,
    Inventory,
    GetTransactions,
    Transactions,
    ;

    companion object {
        fun getSerializer(type: Int): KSerializer<out Packet> {
            return when (type) {
                Version.ordinal -> ninja.blacknet.packet.Version.serializer()
                Ping.ordinal -> ninja.blacknet.packet.Ping.serializer()
                Pong.ordinal -> ninja.blacknet.packet.Pong.serializer()
                GetPeers.ordinal -> ninja.blacknet.packet.GetPeers.serializer()
                Peers.ordinal -> ninja.blacknet.packet.Peers.serializer()
                InventoryV1.ordinal -> ninja.blacknet.packet.InventoryV1.serializer()
                GetData.ordinal -> ninja.blacknet.packet.GetData.serializer()
                Data.ordinal -> ninja.blacknet.packet.Data.serializer()
                GetBlocks.ordinal -> ninja.blacknet.packet.GetBlocks.serializer()
                Blocks.ordinal -> ninja.blacknet.packet.Blocks.serializer()
                ChainAnnounce.ordinal -> ninja.blacknet.packet.ChainAnnounce.serializer()
                ChainFork.ordinal -> ninja.blacknet.packet.ChainFork.serializer()
                Inventory.ordinal -> ninja.blacknet.packet.Inventory.serializer()
                GetTransactions.ordinal -> ninja.blacknet.packet.GetTransactions.serializer()
                Transactions.ordinal -> ninja.blacknet.packet.Transactions.serializer()
                else -> throw RuntimeException("Unknown packet type $type")
            }
        }
    }
}
