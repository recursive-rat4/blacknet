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
import ninja.blacknet.crypto.*
import ninja.blacknet.crypto.Blake2b.buildHash
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.transaction.TxData
import ninja.blacknet.transaction.TxType

@Serializable
class Transaction(
        var signature: Signature,
        val from: PublicKey,
        val seq: Int,
        val referenceChain: Hash,
        val fee: Long,
        val type: Byte,
        val data: SerializableByteArray
) {
    fun data(): TxData {
        return TxData.deserialize(type, data.array)
    }

    fun sign(privateKey: PrivateKey): Pair<Hash, ByteArray> {
        val bytes = BinaryEncoder.toBytes(serializer(), this)
        val hash = hash(bytes)
        signature = Ed25519.sign(hash, privateKey)
        System.arraycopy(signature.bytes, 0, bytes, 0, Signature.SIZE_BYTES)
        return Pair(hash, bytes)
    }

    fun verifySignature(hash: Hash): Boolean {
        return Ed25519.verify(signature, hash, from)
    }

    companion object {
        fun hash(bytes: ByteArray): Hash {
            return buildHash {
                encodeByteArray(bytes, Signature.SIZE_BYTES, bytes.size - Signature.SIZE_BYTES)
            }
        }

        fun create(from: PublicKey, seq: Int, referenceChain: Hash, fee: Long, type: Byte, data: ByteArray): Transaction {
            return Transaction(Signature.EMPTY, from, seq, referenceChain, fee, type, SerializableByteArray(data))
        }

        /**
         * Returns a new transaction of the [TxType.Generated].
         *
         * [Transaction.signature] the empty signature
         *
         * [Transaction.from] generator of the block
         *
         * [Transaction.seq] height of the block
         *
         * [Transaction.referenceChain] hash of the block
         *
         * [Transaction.fee] the amount
         *
         * [Transaction.type] 254
         *
         * [Transaction.data] the empty object
         *
         * @return the constructed [Transaction]
         */
        fun generated(from: PublicKey, height: Int, referenceChain: Hash, amount: Long): Transaction {
            return Transaction(Signature.EMPTY, from, height, referenceChain, amount, TxType.Generated.type, SerializableByteArray.EMPTY)
        }
    }
}
