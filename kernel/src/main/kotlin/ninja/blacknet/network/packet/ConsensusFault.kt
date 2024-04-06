/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import kotlinx.serialization.Serializable
import ninja.blacknet.network.BlockFetcher
import ninja.blacknet.network.Connection

@Serializable
class ConsensusFault(
) : Packet {
    override fun handle(connection: Connection) {

        BlockFetcher.consensusFault(connection, this)
    }
}
