/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import java.math.BigInteger
import kotlinx.serialization.Serializable
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.crypto.BigIntegerSerializer
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.ByteArraySerializer

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
        val accounts: ArrayList<Pair<
            PublicKey,
            @Serializable(with = ByteArraySerializer::class) ByteArray?
        >>,
        val htlcs: ArrayList<Pair<
            HashTimeLockContractId,
            @Serializable(with = ByteArraySerializer::class) ByteArray?
        >>,
        val multisigs: ArrayList<Pair<
            MultiSignatureLockContractId,
            @Serializable(with = ByteArraySerializer::class) ByteArray?
        >>,
        val forkV2: Short,
        val bapps: ArrayList<Pair<
                @Serializable(with = ByteArraySerializer::class) ByteArray,
                @Serializable(with = ByteArraySerializer::class) ByteArray?
        >>
) {
    fun add(publicKey: PublicKey, account: ByteArray?) {
        accounts.add(Pair(publicKey, account))
    }

    fun addHTLC(id: HashTimeLockContractId, htlc: ByteArray?) {
        htlcs.add(Pair(id, htlc))
    }

    fun addMultisig(id: MultiSignatureLockContractId, multisig: ByteArray?) {
        multisigs.add(Pair(id, multisig))
    }

    fun addBApp(key: ByteArray, data: ByteArray?) {
        bapps.add(Pair(key, data))
    }
}
