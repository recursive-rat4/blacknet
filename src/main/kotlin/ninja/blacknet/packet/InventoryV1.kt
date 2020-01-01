/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.core.DataType
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.Connection
import ninja.blacknet.network.TxFetcher
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.BinaryEncoder

@Serializable
internal class InventoryV1(private val list: List<Pair<Byte, Hash>>) : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.InventoryV1

    override suspend fun process(connection: Connection) {
        if (Node.isInitialSynchronization())
            return

        if (list.size > Inventory.MAX) {
            connection.dos("invalid Inventory size")
            return
        }

        for ((type, _) in list) {
            if (type != DataType.Transaction) {
                connection.dos("Inv type $type")
                return
            }
        }

        TxFetcher.offer(connection, list.map { (_, hash) -> hash })
    }
}
