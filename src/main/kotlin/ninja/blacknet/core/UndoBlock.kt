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
import ninja.blacknet.contract.HashTimeLockContractIdSerializer
import ninja.blacknet.contract.MultiSignatureLockContractIdSerializer
import ninja.blacknet.crypto.BigIntegerSerializer
import ninja.blacknet.crypto.HashSerializer
import ninja.blacknet.crypto.PublicKeySerializer
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
        @Serializable(with = HashSerializer::class)
        val nxtrng: ByteArray,
        @Serializable(with = HashSerializer::class)
        val rollingCheckpoint: ByteArray,
        val upgraded: Short,
        val blockSize: Int,
        @Serializable(with = AccountsSerializer::class)
        val accounts: ArrayList<Pair<ByteArray, ByteArray>>,
        @Serializable(with = HTLCsSerializer::class)
        val htlcs: ArrayList<Pair<ByteArray, ByteArray>>,
        @Serializable(with = MultisigsSerializer::class)
        val multisigs: ArrayList<Pair<ByteArray, ByteArray>>,
        val forkV2: Short
) {
    fun add(publicKey: ByteArray, account: ByteArray?) {
        val bytes = if (account != null)
            account
        else
            emptyByteArray()
        accounts.add(Pair(publicKey, bytes))
    }

    fun addHTLC(id: ByteArray, htlc: ByteArray?) {
        val bytes = if (htlc != null)
            htlc
        else
            emptyByteArray()
        htlcs.add(Pair(id, bytes))
    }

    fun addMultisig(id: ByteArray, multisig: ByteArray?) {
        val bytes = if (multisig != null)
            multisig
        else
            emptyByteArray()
        multisigs.add(Pair(id, bytes))
    }
}

private object AccountsSerializer : KSerializer<List<Pair<ByteArray, ByteArray>>>
    by ListSerializer(PairSerializer(PublicKeySerializer, ByteArraySerializer))

private object HTLCsSerializer : KSerializer<List<Pair<ByteArray, ByteArray>>>
    by ListSerializer(PairSerializer(HashTimeLockContractIdSerializer, ByteArraySerializer))

private object MultisigsSerializer : KSerializer<List<Pair<ByteArray, ByteArray>>>
    by ListSerializer(PairSerializer(MultiSignatureLockContractIdSerializer, ByteArraySerializer))
