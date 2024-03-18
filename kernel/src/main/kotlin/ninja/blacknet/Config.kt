/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PrivateKeySerializer
import ninja.blacknet.network.Port

@Serializable
class Config(
        val minrelayfeerate: String = "0.001",
        val ipv4: Boolean = true,
        val ipv6: Boolean = true,
        val listen: Boolean = true,
        val port: Port = mode.defaultP2PPort,
        val tor: Boolean = true,
        val i2p: Boolean = true,
        val upnp: Boolean = true,
        val incomingconnections: Int = 128,
        val outgoingconnections: Int = 8,
        val proxyhost: String? = null,
        val proxyport: Port? = null,
        val torhost: String? = null,
        val torport: Port? = null,
        val torcontrol: Port = Port(9051),
        val i2psamhost: String? = null,
        val i2psamport: Port? = null,
        val dbcache: Size = Size.parse("128MiB"),
        var mnemonics: List<@Serializable(PrivateKeySerializer::class) ByteArray>? = null,
        // val master: String? = null,
        // val slave: String? = null,
        val softblocksizelimit: Size = Size(PoS.MAX_BLOCK_SIZE),
        val txpoolsize: Size = Size(128 * 1024 * 1024),
        val logips: Boolean = false,
        // val whitelist: Set<String>? = null,
        // val blacklist: Set<String>? = null,
        val debugcoroutines: Boolean = false,
        val rpcserver: Boolean = true,
        val seqthreshold: Int = Int.MAX_VALUE - 1,
) {
}
