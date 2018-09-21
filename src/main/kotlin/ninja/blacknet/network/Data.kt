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
import ninja.blacknet.crypto.Blake2b
import ninja.blacknet.serialization.BlacknetOutput
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Data(private val list: ArrayList<Pair<DataType, SerializableByteArray>>) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.Data.ordinal
    }

    override fun process(connection: Connection) {
        if (list.size > DataType.MAX_DATA) {
            connection.dos("invalid Data size")
            return
        }

        for (i in list) {
            val type = i.first
            val bytes = i.second

            val hash = Blake2b.hash(bytes.array)

            DataFetcher.fetched(hash)

            if (!type.db.process(hash, bytes.array))
                connection.dos("invalid " + type.name + " " + hash)
        }
    }
}