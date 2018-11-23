/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.io.core.readBytes
import kotlinx.serialization.Serializable
import kotlinx.serialization.encode
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Burn(
        val amount: Long,
        val message: SerializableByteArray
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.Burn.ordinal.toByte()
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, account: AccountState, ledger: Ledger, undo: UndoBlock): Boolean {
        if (!account.credit(amount)) {
            return false
        }
        ledger.addSupply(-amount)
        return true
    }
}