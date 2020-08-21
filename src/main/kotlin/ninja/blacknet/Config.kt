/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PrivateKeySerializer
import ninja.blacknet.serialization.ConfigDecoderImpl
import ninja.blacknet.serialization.ConfigReader
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
        var mnemonics: List<@Serializable(PrivateKeySerializer::class) ByteArray>? = null,
        // 主從模式
        val softblocksizelimit: Size = Size(PoS.MAX_BLOCK_SIZE),
        val txpoolsize: Size = Size(128 * 1024 * 1024),
        val logips: Boolean = false,
        // 白名單
        // val blacklist: Set<String>? = null,
        val lowercasehex: Boolean = false,
        val regtest: Boolean = false,
        val debugcoroutines: Boolean = false,

        @SerialName("apiserver.enabled")
        val apiserver_enabled: Boolean = true,
        @SerialName("apiserver.jsonindented")
        val apiserver_jsonindented: Boolean = false,

        @SerialName("wallet.seqthreshold")
        val wallet_seqthreshold: Int = Int.MAX_VALUE - 1,

        val unit: Unit? = kotlin.Unit /* XXX 1.4 */
) {
    companion object {
        val instance = ConfigDecoderImpl(ConfigReader(File(configDir, "blacknet.conf"))).decode(serializer()).also {
            if (it.dbcache.bytes < 1024 * 1024) throw ConfigError("dbcache ${it.dbcache.hrp(false)} is unrealistically low")
            if (it.txpoolsize.bytes < 1024 * 1024) throw ConfigError("txpoolsize ${it.txpoolsize.hrp(false)} is unrealistically low")
        }
    }
}

private class ConfigError(message: String) : Error(message)
