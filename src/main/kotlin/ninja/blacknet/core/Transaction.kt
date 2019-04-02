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
import kotlinx.serialization.json.JsonElement
import ninja.blacknet.crypto.*
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.transaction.TxData

@Serializable
class Transaction(
        var signature: Signature,
        val from: PublicKey,
        val seq: Int,
        val blockHash: Hash,
        val fee: Long,
        val type: Byte,
        val data: SerializableByteArray
) {
    fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)
    fun toJson(hash: Hash, size: Int) = Json.toJson(Info.serializer(), Info(this, hash, size))

    fun sign(privateKey: PrivateKey): Pair<Hash, ByteArray> {
        val bytes = serialize()
        val hash = Hasher(bytes)
        signature = Ed25519.sign(hash, privateKey)
        System.arraycopy(signature.bytes, 0, bytes, 0, Signature.SIZE)
        return Pair(hash, bytes)
    }

    fun verifySignature(hash: Hash): Boolean {
        return Ed25519.verify(signature, hash, from)
    }

    object Hasher : (ByteArray) -> Hash {
        override fun invoke(bytes: ByteArray): Hash {
            return Blake2b.hash(bytes, Signature.SIZE, bytes.size - Signature.SIZE)
        }
    }

    companion object {
        fun deserialize(bytes: ByteArray): Transaction? = BinaryDecoder.fromBytes(bytes).decode(serializer())

        fun create(from: PublicKey, seq: Int, blockHash: Hash?, fee: Long, type: Byte, data: ByteArray): Transaction {
            return Transaction(Signature.EMPTY, from, seq, blockHash ?: Hash.ZERO, fee, type, SerializableByteArray(data))
        }
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val hash: String,
            val size: Int,
            val signature: String,
            val from: String,
            val seq: Int,
            val blockHash: String,
            val fee: String,
            val type: Byte,
            val data: JsonElement
    ) {
        constructor(tx: Transaction, hash: Hash, size: Int) : this(
                hash.toString(),
                size,
                tx.signature.toString(),
                Address.encode(tx.from),
                tx.seq,
                tx.blockHash.toString(),
                tx.fee.toString(),
                tx.type,
                data(tx.type, tx.data.array)
        )

        companion object {
            private fun data(type: Byte, bytes: ByteArray): JsonElement {
                val txData = TxData.deserialize(type, bytes) ?: throw RuntimeException("Deserialization error")
                return txData.toJson()
            }
        }
    }
}
