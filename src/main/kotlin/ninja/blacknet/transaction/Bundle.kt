/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Bundle(
        val magic: Int,
        val data: SerializableByteArray
) : TxData {
    override fun serialize() = BlacknetEncoder.toBytes(serializer(), this)

    override fun getType() = TxType.Bundle

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger, undo: UndoBuilder): Boolean {
        return true
    }
}
