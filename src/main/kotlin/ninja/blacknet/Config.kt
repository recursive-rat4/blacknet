/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.natpryce.konfig.*

object Config {
    private val config = ConfigurationProperties.fromResource("blacknet.conf")

    val mintxfee by stringType
    val dnsseed by booleanType
    val p2pport by intType
    val listen by booleanType
    val upnp by booleanType
    val incomingconnections by intType
    val outgoingconnections by intType
    val proxyhost by stringType
    val proxyport by intType
    val torhost by stringType
    val torport by intType
    val torcontrol by intType

    operator fun <T> get(key: Key<T>): T = config[key]
    fun <T> contains(key: Key<T>): Boolean = config.contains(key)
}