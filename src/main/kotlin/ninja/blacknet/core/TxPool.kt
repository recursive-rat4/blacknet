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
import ninja.blacknet.api.APIServer
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.WalletDB
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.SerializableByteArray
import kotlin.math.min

private val logger = KotlinLogging.logger {}

object TxPool : MemPool(), Ledger {
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

    override fun checkReferenceChain(hash: Hash): Boolean {
        return LedgerDB.checkReferenceChain(hash)
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

    override fun getOrCreate(key: PublicKey): AccountState {
        return get(key) ?: AccountState.create()
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

    override suspend fun processImpl(hash: Hash, bytes: ByteArray): Status {
        val tx = Transaction.deserialize(bytes)
        val status = processTransactionImpl(tx, hash, bytes.size)
        if (status == Accepted) {
            addImpl(hash, bytes)
            transactions.add(hash)
        }
        return status
    }

    internal suspend fun processImplWithFee(hash: Hash, bytes: ByteArray, time: Long): Pair<Status, Long> {
        val tx = Transaction.deserialize(bytes)
        val status = processTransactionImpl(tx, hash, bytes.size)
        if (status == Accepted) {
            addImpl(hash, bytes)
            transactions.add(hash)
            WalletDB.processTransaction(hash, tx, bytes, time)
            APIServer.txPoolNotify(tx, hash, time, bytes.size)
            logger.debug { "Accepted $hash" }
        }
        return Pair(status, tx.fee)
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
                processImpl(hash, map[hash]!!)
    }
}
