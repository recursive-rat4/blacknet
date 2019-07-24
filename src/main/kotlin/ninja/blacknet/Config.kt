/*
 * Copyright (c) 2018 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.natpryce.konfig.*
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.network.Network
import java.io.File

object Config {
    private val config = ConfigurationProperties.fromFile(File("config/blacknet.conf"))

    val mintxfee by stringType
    val dnsseed by booleanType
    val ipv4 by booleanType
    val ipv6 by booleanType
    val listen by booleanType
    val port by intType
    val tor by booleanType
    val i2p by booleanType
    val upnp by booleanType
    val incomingconnections by intType
    val outgoingconnections by intType
    val proxyhost by stringType
    val proxyport by intType
    val torhost by stringType
    val torport by intType
    val torcontrol by intType
    val i2psamhost by stringType
    val i2psamport by intType
    val dbcache by intType
    val mnemonics by listType(stringType)
    val softblocksizelimit by intType
    val txpoolsize by intType
    val portable by booleanType
    val datadir by stringType
    val logips by booleanType

    object apiserver : PropertyGroup() {
        val jsonindented by booleanType
        val publicserver by booleanType
    }

    object runtime : PropertyGroup() {
        val debugcoroutines by booleanType
    }

    operator fun <T> get(key: Key<T>): T = config[key]
    fun <T> contains(key: Key<T>): Boolean = config.contains(key)

    private val disabledIPv4 = !config[ipv4]
    private val disabledIPv6 = !config[ipv6]
    private val disabledTOR = !config[tor]
    private val disabledI2P = !config[i2p]

    fun isDisabled(network: Network): Boolean = when (network) {
        Network.IPv4 -> disabledIPv4
        Network.IPv6 -> disabledIPv6
        Network.TORv2 -> disabledTOR
        Network.TORv3 -> disabledTOR
        Network.I2P -> disabledI2P
    }

    fun jsonIndented(): Boolean {
        if (contains(apiserver.jsonindented))
            return get(apiserver.jsonindented)
        else
            return false
    }

    fun portable(): Boolean {
        if (contains(portable))
            return get(portable)
        else
            return false
    }

    fun publicAPI(): Boolean {
        if (contains(apiserver.publicserver))
            return get(apiserver.publicserver)
        else
            return false
    }

    var debugCoroutines: Boolean

    val incomingConnections: Int = Config[incomingconnections]
    val outgoingConnections: Int = Config[outgoingconnections]

    val logIPs: Boolean = {
        if (contains(logips))
            get(logips)
        else
            false
    }()

    val softBlockSizeLimit: Int = {
        if (contains(softblocksizelimit))
            get(softblocksizelimit)
        else
            LedgerDB.MAX_BLOCK_SIZE
    }()

    val txPoolSize: Int = {
        if (contains(txpoolsize))
            get(txpoolsize) * MiB
        else
            128 * MiB
    }()

    val dataDir: String = {
        val dir = if (portable()) {
            File("db").getAbsolutePath()
        } else if (!contains(datadir)) {
            val userHome = System.getProperty("user.home")
            val osName = System.getProperty("os.name", "generic").toLowerCase()

            userHome +
                    if ((osName.indexOf("mac") >= 0) || (osName.indexOf("darwin") >= 0))
                        "/Library/Application Support/Blacknet"
                    else if (osName.indexOf("win") >= 0)
                        "/AppData/Roaming/Blacknet"
                    else
                        "/.blacknet"
        } else {
            get(datadir)
        }
        File(dir).mkdirs()
        dir
    }()

    private const val MiB = 1024 * 1024

    init {
        debugCoroutines =
                if (contains(runtime.debugcoroutines))
                    get(runtime.debugcoroutines)
                else
                    false

    }
}
