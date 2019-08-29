/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.coroutines.sync.withLock
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.WalletDB
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import ninja.blacknet.network.Runtime
import ninja.blacknet.serialization.SerializableByteArray
import kotlin.math.min

private val logger = KotlinLogging.logger {}

object TxPool : MemPool(), Ledger {
    internal const val INVALID = -1L
    internal const val IN_FUTURE = -2L
    private val accounts = HashMap<PublicKey, AccountState>()
    private val htlcs = HashMap<Hash, HTLC?>()
    private val multisigs = HashMap<Hash, Multisig?>()
    private val transactions = ArrayList<Hash>()

    suspend fun fill(block: Block) = mutex.withLock {
        val poolSize = sizeImpl()
        var freeSize = min(LedgerDB.maxBlockSize(), Config.softBlockSizeLimit) - 176
        var i = 0
        while (freeSize > 0 && i < poolSize) {
            val hash = transactions.get(i++)
            val bytes = getImpl(hash)
            if (bytes == null) {
                logger.error("inconsistent mempool")
                continue
            }
            if (bytes.size + 4 > freeSize)
                break

            freeSize -= bytes.size + 4
            block.transactions.add(SerializableByteArray(bytes))
        }
    }

    suspend fun getSequence(key: PublicKey): Int = mutex.withLock {
        val account = accounts.get(key)
        if (account != null)
            return account.seq
        return LedgerDB.get(key)?.seq ?: 0
    }

    override fun addSupply(amount: Long) {}

    override fun checkBlockHash(hash: Hash): Boolean {
        return LedgerDB.checkBlockHash(hash)
    }

    override fun checkFee(size: Int, amount: Long): Boolean {
        return amount >= Node.minTxFee * (1 + size / 1000)
    }

    override fun blockTime(): Long {
        return LedgerDB.blockTime()
    }

    override fun height(): Int {
        return LedgerDB.height()
    }

    override fun get(key: PublicKey): AccountState? {
        val account = accounts.get(key)
        if (account != null)
            return account
        return LedgerDB.get(key)
    }

    override fun set(key: PublicKey, state: AccountState) {
        accounts.put(key, state)
    }

    override fun addHTLC(id: Hash, htlc: HTLC) {
        htlcs.put(id, htlc)
    }

    override fun getHTLC(id: Hash): HTLC? {
        if (!htlcs.containsKey(id))
            return LedgerDB.getHTLC(id)
        return htlcs.get(id)
    }

    override fun removeHTLC(id: Hash) {
        htlcs.put(id, null)
    }

    override fun addMultisig(id: Hash, multisig: Multisig) {
        multisigs.put(id, multisig)
    }

    override fun getMultisig(id: Hash): Multisig? {
        if (!multisigs.containsKey(id))
            return LedgerDB.getMultisig(id)
        return multisigs.get(id)
    }

    override fun removeMultisig(id: Hash) {
        multisigs.put(id, null)
    }

    override suspend fun processImpl(hash: Hash, bytes: ByteArray, connection: Connection?): Status {
        val tx = Transaction.deserialize(bytes)
        if (tx == null) {
            logger.info("deserialization failed")
            return Status.INVALID
        }
        val status = processTransactionImpl(tx, hash, bytes.size, TxUndoBuilder())
        if (status == Status.ACCEPTED) {
            addImpl(hash, bytes)
            transactions.add(hash)
        }
        return status
    }

    internal suspend fun processImplWithFee(hash: Hash, bytes: ByteArray, connection: Connection?): Long {
        val tx = Transaction.deserialize(bytes)
        if (tx == null) {
            logger.info("deserialization failed")
            return INVALID
        }
        val status = processTransactionImpl(tx, hash, bytes.size, TxUndoBuilder())
        if (status == Status.ACCEPTED) {
            addImpl(hash, bytes)
            transactions.add(hash)
            val currTime = connection?.lastPacketTime ?: Runtime.time()
            connection?.lastTxTime = currTime
            WalletDB.processTransaction(hash, tx, bytes, currTime)
            logger.debug { "Accepted $hash" }
            return tx.fee
        } else if (status == Status.IN_FUTURE) {
            return IN_FUTURE
        } else {
            return INVALID
        }
    }

    private class TxUndoBuilder : UndoBuilder(0, BigInt.ZERO, BigInt.ZERO, 0, Hash.ZERO, Hash.ZERO, 0, 0, ArrayList()) {
        override fun add(publicKey: PublicKey, state: AccountState) {}
        override fun addHTLC(id: Hash, htlc: HTLC?) {}
        override fun addMultisig(id: Hash, multisig: Multisig?) {}
    }

    internal suspend fun removeImpl(hashes: ArrayList<Hash>) {
        if (hashes.isEmpty() || transactions.isEmpty())
            return

        val txs = ArrayList(transactions)
        val map = copyImpl()
        accounts.clear()
        htlcs.clear()
        multisigs.clear()
        transactions.clear()
        clearImpl()
        for (hash in txs)
            if (!hashes.contains(hash))
                processImpl(hash, map[hash]!!, null)
    }
}
