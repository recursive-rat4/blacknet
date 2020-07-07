/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import io.ktor.utils.io.core.ByteReadPacket
import java.math.BigInteger
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.BigIntegerSerializer
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.ChainFetcher
import ninja.blacknet.network.Connection
import ninja.blacknet.serialization.BinaryEncoder

@Serializable
class ChainAnnounce(
        internal val chain: Hash,
        @Serializable(with = BigIntegerSerializer::class)
        internal val cumulativeDifficulty: BigInteger
) : Packet {
    override suspend fun process(connection: Connection) {
        connection.lastChain = this

        ChainFetcher.offer(connection, this)
    }

    companion object {
        val GENESIS = ChainAnnounce(Hash.ZERO, BigInteger.ZERO)
    }
}
