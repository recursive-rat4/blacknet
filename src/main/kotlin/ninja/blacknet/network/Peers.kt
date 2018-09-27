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
import ninja.blacknet.db.PeerDB
import ninja.blacknet.serialization.BlacknetOutput

@Serializable
class Peers(private val list: List<Address>) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.Peers.ordinal
    }

    override suspend fun process(connection: Connection) {
        if (list.size > MAX) {
            connection.dos("invalid Peers size")
            return
        }

        for (i in list) {
            if (!i.checkSize() || i.isLocal()) {
                connection.dos("invalid Address")
                return
            }
        }

        PeerDB.add(list, connection.remoteAddress)
        PeerDB.commit()
    }

    companion object {
        const val MAX = 1000
    }
}