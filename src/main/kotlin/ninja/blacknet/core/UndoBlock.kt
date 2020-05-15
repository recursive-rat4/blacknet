/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.Serializable
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.SerializableByteArray

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
        val accounts: ArrayList<Pair<PublicKey, SerializableByteArray>>,
        val htlcs: ArrayList<Pair<HashTimeLockContractId, SerializableByteArray>>,
        val multisigs: ArrayList<Pair<MultiSignatureLockContractId, SerializableByteArray>>,
        val forkV2: Short
) {
    fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)

    fun add(publicKey: PublicKey, account: ByteArray?) {
        val bytes = if (account != null)
            SerializableByteArray(account)
        else
            SerializableByteArray.EMPTY
        accounts.add(Pair(publicKey, bytes))
    }

    fun addHTLC(id: HashTimeLockContractId, htlc: ByteArray?) {
        val bytes = if (htlc != null)
            SerializableByteArray(htlc)
        else
            SerializableByteArray.EMPTY
        htlcs.add(Pair(id, bytes))
    }

    fun addMultisig(id: MultiSignatureLockContractId, multisig: ByteArray?) {
        val bytes = if (multisig != null)
            SerializableByteArray(multisig)
        else
            SerializableByteArray.EMPTY
        multisigs.add(Pair(id, bytes))
    }

    companion object {
        fun deserialize(bytes: ByteArray): UndoBlock = BinaryDecoder(bytes).decode(serializer())
    }
}
