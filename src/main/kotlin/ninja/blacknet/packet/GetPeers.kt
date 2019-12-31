/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.db.PeerDB
import ninja.blacknet.network.AddressV1
import ninja.blacknet.network.Connection
import ninja.blacknet.serialization.BinaryEncoder

@Serializable
class GetPeers : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.GetPeers

    override suspend fun process(connection: Connection) {
        if (connection.version > MAX_VERSION) {
            connection.dos("GetPeers disabled")
            return
        }

        if (connection.state == Connection.State.OUTGOING_CONNECTED) {
            connection.dos("GetPeers from outgoing connection")
            return
        }

        val randomPeers = PeerDB.getRandom(Peers.MAX)

        if (connection.version >= Peers.MIN_VERSION)
            connection.sendPacket(Peers(randomPeers))
        else
            connection.sendPacket(PeersV1(randomPeers.map { AddressV1(it) }))
    }

    companion object {
        const val MAX_VERSION = 11
    }
}
