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
import kotlinx.serialization.encode
import ninja.blacknet.core.DataType
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Data(private val list: DataList) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.Data.ordinal
    }

    override suspend fun process(connection: Connection) {
        if (list.size > DataType.MAX_DATA) {
            connection.dos("invalid Data size")
            return
        }

        val inv = InvList()

        for (i in list) {
            val type = i.first
            val bytes = i.second

            val hash = type.hash(bytes.array)

            DataFetcher.fetched(hash)

            if (type.db.process(hash, bytes.array, connection))
                inv.add(Pair(type, hash))
            else
                connection.dos("invalid " + type.name + " " + hash)
        }

        //TODO don't announce to sender
        if (!inv.isEmpty())
            Node.broadcastInv(inv)
    }
}

typealias DataList = ArrayList<Pair<DataType, SerializableByteArray>>
