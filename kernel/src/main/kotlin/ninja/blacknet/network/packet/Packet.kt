/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import ninja.blacknet.network.Connection

/**
 * Packet length is used for delimiting, and as such doesn't count towards packet size.
 */
const val PACKET_LENGTH_SIZE_BYTES = 4
const val PACKET_HEADER_SIZE_BYTES = 4

interface Packet {
    fun handle(connection: Connection)
}
