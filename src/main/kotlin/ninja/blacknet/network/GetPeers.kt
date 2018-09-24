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
class GetPeers : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.GetPeers.ordinal
    }

    override fun process(connection: Connection) {
        if (connection.state == Connection.State.OUTGOING_CONNECTED) {
            connection.dos("GetPeers from outgoing connection")
            return
        }

        val randomPeers = PeerDB.getRandom(Peers.MAX)
        if (randomPeers.size == 0)
            return

        val peers = Peers(randomPeers)
        connection.sendPacket(peers)
    }
}