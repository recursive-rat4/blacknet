/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import java.math.BigInteger
import java.util.concurrent.CompletableFuture
import ninja.blacknet.core.Status
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.packet.BlockAnnounce
import ninja.blacknet.network.packet.Blocks

sealed class BlockSource : Comparable<BlockSource>

class Remote(
    val connection: Connection,
    val announce: BlockAnnounce,
) : BlockSource() {
    override fun compareTo(other: BlockSource): Int = when (other) {
        is Remote -> 0
        is Deferred -> 1
        is Staked -> 1
    }
}

class Deferred(
    val connection: Connection,
    val answer: Blocks,
    val requestedDifficulty: BigInteger,
) : BlockSource() {
    override fun compareTo(other: BlockSource): Int = when (other) {
        is Remote -> -1
        is Deferred -> 0
        is Staked -> 1
    }
}

class Staked(
    val hash: Hash,
    val bytes: ByteArray,
    val future: CompletableFuture<Pair<Status, Int>>,
) : BlockSource() {
    override fun compareTo(other: BlockSource): Int = when (other) {
        is Remote -> -1
        is Deferred -> -1
        is Staked -> 0
    }
}
