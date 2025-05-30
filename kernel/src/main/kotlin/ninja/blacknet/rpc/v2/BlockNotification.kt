/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import kotlinx.serialization.Serializable
import ninja.blacknet.core.Block
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey

@Serializable
class BlockNotification(
        val hash: Hash,
        val height: Int,
        val size: Int,
        val version: UInt,
        val previous: Hash,
        val time: Long,
        val generator: PublicKey,
        val transactions: Int
) {
    constructor(block: Block, hash: Hash, height: Int, size: Int) : this(
            hash,
            height,
            size,
            block.version,
            block.previous,
            block.time,
            block.generator,
            block.transactions.size
    )
}
