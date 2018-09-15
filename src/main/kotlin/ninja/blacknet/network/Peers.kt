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
import ninja.blacknet.serialization.BlacknetOutput
import ninja.blacknet.db.PeerDB

@Serializable
class Peers(private val peers: ArrayList<Address>) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.Peers.ordinal
    }

    override fun process(connection: Connection) {
        if (peers.size > MAX) {
            connection.dos(1, "invalid peers addrSize")
            return
        }

        for (i in peers) {
            if (!i.checkSize()) {
                connection.dos(1, "invalid address addrSize")
                return
            }
        }

        PeerDB.add(peers, connection.remoteAddress)
    }

    companion object {
        const val MAX = 1000
    }
}