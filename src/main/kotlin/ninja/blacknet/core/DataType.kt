/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.crypto.Blake2b
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.Signature
import ninja.blacknet.db.BlockDB

enum class DataType(
        val db: DataDB,
        val hash: (ByteArray) -> Hash
) {
    Block(BlockDB, BlockHash),
    Transaction(TxPool, TxHash),
    ;

    object BlockHash : (ByteArray) -> Hash {
        override fun invoke(bytes: ByteArray): Hash {
            return Blake2b.hash(bytes, 0, bytes.size - Signature.SIZE)
        }
    }

    object TxHash : (ByteArray) -> Hash {
        override fun invoke(bytes: ByteArray): Hash {
            return Blake2b.hash(bytes, Signature.SIZE, bytes.size - Signature.SIZE)
        }
    }

    companion object {
        const val MAX_INVENTORY = 50000
        const val MAX_DATA = 1000
    }
}