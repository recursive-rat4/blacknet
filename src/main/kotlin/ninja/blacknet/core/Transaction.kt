/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.io.core.readBytes
import kotlinx.serialization.Serializable
import kotlinx.serialization.encode
import ninja.blacknet.crypto.*
import ninja.blacknet.serialization.BlacknetDecoder
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray

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
    fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    fun sign(privateKey: PrivateKey): Pair<Hash, ByteArray> {
        val bytes = serialize()
        val hash = Hasher(bytes)
        signature = Ed25519.sign(hash, privateKey)
        System.arraycopy(signature.bytes.array, 0, bytes, 0, Signature.SIZE)
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
        fun deserialize(bytes: ByteArray): Transaction? {
            return BlacknetDecoder.fromBytes(bytes).decode(Transaction.serializer())
        }

        fun create(from: PublicKey, seq: Int, fee: Long, type: Byte, data: ByteArray): Transaction {
            return Transaction(Signature.EMPTY, from, seq, Hash.ZERO, fee, type, SerializableByteArray(data))
        }
    }
}