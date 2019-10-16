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
import mu.KotlinLogging
import ninja.blacknet.core.Ledger
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json

private val logger = KotlinLogging.logger {}

@Serializable
class WithdrawFromLease(
        val withdraw: Long,
        val amount: Long,
        val to: PublicKey,
        val height: Int
) : TxData {
    override fun getType() = TxType.WithdrawFromLease
    override fun involves(publicKey: PublicKey) = to == publicKey
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger): Boolean {
        if (withdraw <= 0 || withdraw > amount) {
            logger.info("invalid withdraw amount")
            return false
        }
        if (withdraw > amount - PoS.MIN_LEASE) {
            logger.info("can not withdraw more than ${amount - PoS.MIN_LEASE}")
            return false
        }
        val toAccount = ledger.get(to)
        if (toAccount == null) {
            logger.info("account not found")
            return false
        }
        val lease = toAccount.leases.find { it.publicKey == tx.from && it.height == height && it.amount == amount }
        if (lease == null) {
            logger.info("lease not found")
            return false
        }
        lease.amount -= withdraw
        ledger.set(to, toAccount)
        val account = ledger.get(tx.from)!!
        account.debit(ledger.height(), withdraw)
        ledger.set(tx.from, account)
        return true
    }

    companion object {
        fun deserialize(bytes: ByteArray): WithdrawFromLease = BinaryDecoder.fromBytes(bytes).decode(serializer())
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
