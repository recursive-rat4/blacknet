/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.io.core.readBytes
import kotlinx.serialization.Serializable
import kotlinx.serialization.encode
import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray

private val logger = KotlinLogging.logger {}

@Serializable
class CreateHTLC(
        val amount: Long,
        val to: PublicKey,
        val timeLockType: Byte,
        val timeLock: Long,
        val hashType: Byte,
        val hashLock: SerializableByteArray
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.CreateHTLC.type
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, account: AccountState, ledger: Ledger, undo: UndoBlock): Boolean {
        if (!HTLC.isValidTimeLockType(timeLockType)) {
            logger.info("unknown timelock type $timeLockType")
            return false
        }
        if (!HTLC.isValidHashType(hashType)) {
            logger.info("unknown hash type $hashType")
            return false
        }

        if (amount == 0L) {
            logger.info("invalid amount")
            return false
        }

        if (!account.credit(amount))
            return false

        undo.addHTLC(hash, null)

        val htlc = HTLC(ledger.height(), ledger.blockTime(), amount, tx.from, to, timeLockType, timeLock, hashType, hashLock)
        ledger.addHTLC(hash, htlc)
        return true
    }
}
