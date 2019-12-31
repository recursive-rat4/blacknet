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
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Blake2b
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.LedgerDB.forkV2
import ninja.blacknet.serialization.*

@Serializable
class CreateHTLC(
        val amount: Long,
        val to: PublicKey,
        val timeLockType: Byte,
        val timeLock: Long,
        val hashType: Byte,
        val hashLock: SerializableByteArray
) : TxData {
    override fun getType() = TxType.CreateHTLC
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    fun id(hash: Hash, dataIndex: Int) = if (forkV2()) Blake2b.hasher { this + hash + dataIndex } else hash

    override suspend fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        if (!HTLC.isValidTimeLockType(timeLockType)) {
            return Invalid("Unknown time lock type $timeLockType")
        }
        if (forkV2()) {
            if (!HTLC.isValidHashLock(hashType, hashLock)) {
                return Invalid("Invalid hash lock type $hashType size ${hashLock.array.size}")
            }
        } else {
            if (!HTLC.isValidHashType(hashType)) {
                return Invalid("Unknown hash type $hashType")
            }
        }

        if (amount == 0L) {
            return Invalid("Invalid amount")
        }

        val account = ledger.get(tx.from)!!
        val status = account.credit(amount)
        if (status != Accepted) {
            return status
        }

        val id = id(hash, dataIndex)
        val htlc = HTLC(ledger.height(), ledger.blockTime(), amount, tx.from, to, timeLockType, timeLock, hashType, hashLock)
        ledger.set(tx.from, account)
        ledger.addHTLC(id, htlc)
        return Accepted
    }

    fun involves(publicKey: PublicKey) = to == publicKey

    companion object {
        fun deserialize(bytes: ByteArray): CreateHTLC = BinaryDecoder.fromBytes(bytes).decode(serializer())
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val amount: String,
            val to: String,
            val timeLockType: Int,
            val timeLock: Long,
            val hashType: Int,
            val hashLock: String
    ) {
        constructor(data: CreateHTLC) : this(
                data.amount.toString(),
                Address.encode(data.to),
                data.timeLockType.toUByte().toInt(),
                data.timeLock,
                data.hashType.toUByte().toInt(),
                data.hashLock.array.toHex()
        )
    }
}
