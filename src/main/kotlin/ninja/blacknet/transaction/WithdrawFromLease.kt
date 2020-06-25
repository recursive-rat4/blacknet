/*
 * Copyright (c) 2019 Pavel Vasin
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
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.VarLong

/**
 * 提款金額
 */
@Serializable
class WithdrawFromLease(
        val withdraw: Long,
        val amount: Long,
        val to: PublicKey,
        val height: Int
) : TxData {
    override fun getType() = TxType.WithdrawFromLease
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        if (withdraw <= 0 || withdraw > amount) {
            return Invalid("Invalid withdraw amount")
        }
        if (withdraw > amount - PoS.MIN_LEASE) {
            return Invalid("Can not withdraw more than ${amount - PoS.MIN_LEASE}")
        }
        val toAccount = ledger.get(to)
        if (toAccount == null) {
            return Invalid("Account not found")
        }
        val lease = toAccount.leases.find { it.publicKey == tx.from && it.height.int == height && it.amount.long == amount }
        if (lease == null) {
            return Invalid("Lease not found")
        }
        lease.amount = VarLong(lease.amount.long - withdraw)
        ledger.set(to, toAccount)
        val account = ledger.get(tx.from)!!
        account.debit(ledger.height(), withdraw)
        ledger.set(tx.from, account)
        return Accepted
    }

    fun involves(publicKey: PublicKey) = to == publicKey

    companion object {
        fun deserialize(bytes: ByteArray): WithdrawFromLease = BinaryDecoder(bytes).decode(serializer())
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val withdraw: String,
            val amount: String,
            val to: String,
            val height: Int
    ) {
        constructor(data: WithdrawFromLease) : this(
                data.withdraw.toString(),
                data.amount.toString(),
                Address.encode(data.to),
                data.height
        )
    }
}
