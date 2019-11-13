/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder

@Serializable
class UndoBlock(
        val blockTime: Long,
        val difficulty: BigInt,
        val cumulativeDifficulty: BigInt,
        val supply: Long,
        val nxtrng: Hash,
        val rollingCheckpoint: Hash,
        val upgraded: Short,
        val blockSize: Int,
        val accounts: ArrayList<Pair<PublicKey, AccountState>>,
        val htlcs: ArrayList<Pair<Hash, HTLC?>>,
        val multisigs: ArrayList<Pair<Hash, Multisig?>>,
        val forkV2: Short
) {
    fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)

    companion object {
        fun deserialize(bytes: ByteArray): UndoBlock = BinaryDecoder.fromBytes(bytes).decode(serializer())
    }
}

class UndoBuilder(
        val blockTime: Long,
        val difficulty: BigInt,
        val cumulativeDifficulty: BigInt,
        val supply: Long,
        val nxtrng: Hash,
        val rollingCheckpoint: Hash,
        val upgraded: Short,
        val blockSize: Int,
        val forkV2: Short,
        private val accounts: HashMap<PublicKey, AccountState> = HashMap(),
        private val htlcs: HashMap<Hash, HTLC?> = HashMap(),
        private val multisigs: HashMap<Hash, Multisig?> = HashMap()
) {
    fun add(publicKey: PublicKey, state: AccountState) {
        if (!accounts.containsKey(publicKey))
            accounts.put(publicKey, state.copy())
    }

    fun addHTLC(id: Hash, htlc: HTLC?) {
        if (!htlcs.containsKey(id))
            htlcs.put(id, htlc)
    }

    fun addMultisig(id: Hash, multisig: Multisig?) {
        if (!multisigs.containsKey(id))
            multisigs.put(id, multisig)
    }

    fun build(): UndoBlock {
        return UndoBlock(
                blockTime,
                difficulty,
                cumulativeDifficulty,
                supply,
                nxtrng,
                rollingCheckpoint,
                upgraded,
                blockSize,
                accounts.toArrayList(),
                htlcs.toArrayList(),
                multisigs.toArrayList(),
                forkV2)
    }
}

private fun <K, V> HashMap<K, V>.toArrayList(): ArrayList<Pair<K, V>> {
    return mapTo(ArrayList(size)) { Pair(it.key, it.value) }
}
