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
import ninja.blacknet.network.Node

@Serializable
class NodeInfo(
        val agent: String,
        val version: Int,
        val outgoing: Int,
        val incoming: Int,
        val listening: List<String>
) {
    companion object {
        suspend fun get(): NodeInfo {
            val listening = Node.listenAddress.map { it.toString() }
            return NodeInfo(Node.agent, Node.version, Node.outgoing(), Node.incoming(), listening)
        }
    }
}
