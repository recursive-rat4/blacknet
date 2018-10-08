/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import kotlinx.serialization.Serializable
import ninja.blacknet.network.Connection

@Serializable
class PeerInfo(
        val remoteAddress: String,
        val localAddress: String,
        val connectedat: Long,
        val timeoffset: Long,
        val ping: Long,
        val version: Int,
        val agent: String,
        val state: String,
        val dosscore: Int
) {
    constructor(connection: Connection) : this(
            connection.remoteAddress.toString(),
            connection.localAddress.toString(),
            connection.connectedAt,
            connection.timeOffset,
            connection.ping,
            connection.version,
            connection.agent,
            connection.state.name,
            connection.dosScore
    )
}