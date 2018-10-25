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
import ninja.blacknet.core.Block
import ninja.blacknet.core.DataType

@Serializable
class BlockInfo(
        val size: Int,
        val version: Int,
        val previous: String,
        val time: Long,
        val generator: String,
        val transactions: List<String>,
        val signature: String
) {
    constructor(block: Block, size: Int) : this(
            size,
            block.version,
            block.previous.toString(),
            block.time,
            block.generator.toString(),
            block.transactions.map { DataType.Transaction.hash(it.array).toString() },
            block.signature.toString()
    )
}