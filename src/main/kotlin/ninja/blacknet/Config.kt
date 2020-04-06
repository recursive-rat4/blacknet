/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.typesafe.config.ConfigFactory
import kotlinx.serialization.Serializable
import kotlinx.serialization.config.ConfigParser
import ninja.blacknet.crypto.PoS
import java.io.File

@Serializable
class Config(
        val mintxfee: String,
        val ipv4: Boolean,
        val ipv6: Boolean,
        val listen: Boolean,
        val port: Int,
        val tor: Boolean,
        val i2p: Boolean,
        val upnp: Boolean,
        val incomingconnections: Int,
        val outgoingconnections: Int,
        val proxyhost: String? = null,
        val proxyport: Int? = null,
        val torhost: String? = null,
        val torport: Int? = null,
        val torcontrol: Int,
        val i2psamhost: String? = null,
        val i2psamport: Int? = null,
        val dbcache: Size,
        var mnemonics: List<String>? = null,
        val softblocksizelimit: Size = Size(PoS.MAX_BLOCK_SIZE),
        val txpoolsize: Size = Size(128 * 1024 * 1024),
        val portable: Boolean = false,
        val datadir: String? = null,
        val logips: Boolean = false,
        val lowercasehex: Boolean = false,
        val regtest: Boolean = false,
        val debugcoroutines: Boolean = false,

        val apiserver: APIServerConfig = APIServerConfig(),
        val wallet: WalletConfig = WalletConfig(),

        val unit: Unit? // 游戏结束
         = kotlin.Unit  // The feature "trailing commas" is only available since language version 1.4
) {
    companion object {
        val instance = ConfigParser.parse(ConfigFactory.parseFile(File(configDir, "blacknet.conf")), serializer())
    }
}

@Serializable
class APIServerConfig(
        val enabled: Boolean = true,
        val jsonindented: Boolean = false,
        val publicserver: Boolean = false,

        val unit: Unit? // 游戏结束
         = kotlin.Unit  // The feature "trailing commas" is only available since language version 1.4
)

@Serializable
class WalletConfig(
        val seqthreshold: Int = Int.MAX_VALUE - 1,

        val unit: Unit? // 游戏结束
         = kotlin.Unit  // The feature "trailing commas" is only available since language version 1.4
)
