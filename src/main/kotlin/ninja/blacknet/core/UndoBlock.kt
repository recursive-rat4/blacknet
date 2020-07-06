/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import java.math.BigInteger
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.PairSerializer
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BigIntegerSerializer
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.util.emptyByteArray

@Serializable
class UndoBlock(
        val blockTime: Long,
        @Serializable(with = BigIntegerSerializer::class)
        val difficulty: BigInteger,
        @Serializable(with = BigIntegerSerializer::class)
        val cumulativeDifficulty: BigInteger,
        val supply: Long,
        val nxtrng: Hash,
        val rollingCheckpoint: Hash,
        val upgraded: Short,
        val blockSize: Int,
        @Serializable(with = AccountsSerializer::class)
        val accounts: ArrayList<Pair<PublicKey, ByteArray>>,
        @Serializable(with = HTLCsSerializer::class)
        val htlcs: ArrayList<Pair<HashTimeLockContractId, ByteArray>>,
        @Serializable(with = MultisigsSerializer::class)
        val multisigs: ArrayList<Pair<MultiSignatureLockContractId, ByteArray>>,
        val forkV2: Short
) {
    fun add(publicKey: PublicKey, account: ByteArray?) {
        val bytes = if (account != null)
            account
        else
            emptyByteArray()
        accounts.add(Pair(publicKey, bytes))
    }

    fun addHTLC(id: HashTimeLockContractId, htlc: ByteArray?) {
        val bytes = if (htlc != null)
            htlc
        else
            emptyByteArray()
        htlcs.add(Pair(id, bytes))
    }

    fun addMultisig(id: MultiSignatureLockContractId, multisig: ByteArray?) {
        val bytes = if (multisig != null)
            multisig
        else
            emptyByteArray()
        multisigs.add(Pair(id, bytes))
    }
}

private object AccountsSerializer : KSerializer<List<Pair<PublicKey, ByteArray>>>
    by ListSerializer(PairSerializer(PublicKey.Companion, ByteArraySerializer))

private object HTLCsSerializer : KSerializer<List<Pair<HashTimeLockContractId, ByteArray>>>
    by ListSerializer(PairSerializer(HashTimeLockContractId.Companion, ByteArraySerializer))

private object MultisigsSerializer : KSerializer<List<Pair<MultiSignatureLockContractId, ByteArray>>>
    by ListSerializer(PairSerializer(MultiSignatureLockContractId.Companion, ByteArraySerializer))
