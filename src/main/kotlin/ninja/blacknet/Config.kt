/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.natpryce.konfig.*
import ninja.blacknet.crypto.PoS
import ninja.blacknet.network.toPort
import java.io.File

object Config {
    val dir: File = File(path("config"))
    val htmlDir: String = path("html")

    private val config = ConfigurationProperties.fromFile(File(dir, "blacknet.conf"))

    val mintxfee by stringType
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
    val lowercasehex by booleanType
    val regtest by booleanType

    object apiserver : PropertyGroup() {
        val enabled by booleanType
        val jsonindented by booleanType
        val publicserver by booleanType
    }

    object runtime : PropertyGroup() {
        val debugcoroutines by booleanType
    }

    operator fun <T> get(key: Key<T>): T = config[key]
    fun <T> contains(key: Key<T>): Boolean = config.contains(key)

    val disabledIPv4 = !config[ipv4]
    val disabledIPv6 = !config[ipv6]
    val disabledTOR = !config[tor]
    val disabledI2P = !config[i2p]

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

    fun APIenabled(): Boolean {
        if (contains(apiserver.enabled))
            return get(apiserver.enabled)
        else
            return true
    }

    fun publicAPI(): Boolean {
        if (contains(apiserver.publicserver))
            return get(apiserver.publicserver)
        else
            return false
    }

    var debugCoroutines: Boolean

    val netPort: Short = config[port].toPort()
    val netListen: Boolean = config[listen]

    val incomingConnections: Int = config[incomingconnections]
    val outgoingConnections: Int = config[outgoingconnections]

    val logIPs: Boolean = {
        if (contains(logips))
            get(logips)
        else
            false
    }()

    val lowerCaseHex: Boolean = {
        if (contains(lowercasehex))
            get(lowercasehex)
        else
            false
    }()

    val regTest: Boolean = {
        if (contains(regtest))
            get(regtest)
        else
            false
    }()

    val softBlockSizeLimit: Int = {
        if (contains(softblocksizelimit))
            get(softblocksizelimit)
        else
            PoS.MAX_BLOCK_SIZE
    }()

    val txPoolSize: Int = {
        if (contains(txpoolsize))
            get(txpoolsize) * MiB
        else
            128 * MiB
    }()

    val dataDir: File = {
        var dir = if (portable()) {
            File("db")
        } else if (!contains(datadir)) {
            File(System.getProperty("user.home"), when {
                Runtime.macOS -> "Library/Application Support/Blacknet"
                Runtime.windowsOS -> "AppData\\Roaming\\Blacknet"
                else -> ".blacknet"
            })
        } else {
            File(get(datadir))
        }
        if (regTest) {
            dir = File(dir, "regtest")
        }
        dir.mkdirs()
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

    private fun path(path: String): String {
        if (Runtime.windowsOS) {
            val file = File(path)
            if (file.isFile()) {
                // git symlink
                return file.readText()
            }
        }
        return path
    }
}
