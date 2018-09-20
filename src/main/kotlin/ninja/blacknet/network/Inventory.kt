/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BlacknetOutput

@Serializable
class Inventory(private val list: ArrayList<Pair<DataType, Hash>>) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.Inventory.ordinal
    }

    override fun process(connection: Connection) {
        if (list.size > DataType.MAX_INVENTORY) {
            connection.dos("invalid Inventory size")
            return
        }

        val request = ArrayList<Pair<DataType, Hash>>()

        for (i in list) {
            val type = i.first
            val hash = i.second

            if (type.getDB().isInteresting(hash))
                request.add(Pair(type, hash))

            if (request.size == DataType.MAX_DATA) {
                val getData = GetData(request)
                connection.sendPacket(getData)
                request.clear()
            }
        }

        if (request.size == 0)
            return

        val getData = GetData(request)
        connection.sendPacket(getData)
    }
}