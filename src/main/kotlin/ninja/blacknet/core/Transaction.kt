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
import ninja.blacknet.crypto.*
import ninja.blacknet.serialization.BlacknetInput
import ninja.blacknet.serialization.BlacknetOutput
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Transaction(
        private var signature: Signature,
        val from: PublicKey,
        val seq: Int,
        val blochHash: Hash,
        val fee: Long,
        val type: Byte,
        val data: SerializableByteArray
) {
    fun serialize(): ByteArray {
        val out = BlacknetOutput()
        out.write(this)
        return out.build().readBytes()
    }

    fun sign(privateKey: PrivateKey): Pair<Hash, ByteArray> {
        val bytes = serialize()
        val hash = DataType.Transaction.hash(bytes)
        signature = Ed25519.sign(hash, privateKey)
        System.arraycopy(signature.bytes.array, 0, bytes, 0, Signature.SIZE)
        return Pair(hash, bytes)
    }

    fun verifySignature(hash: Hash): Boolean {
        return Ed25519.verify(signature, hash, from)
    }

    companion object {
        fun deserialize(bytes: ByteArray): Transaction? {
            return BlacknetInput.fromBytes(bytes).deserialize(Transaction.serializer())
        }

        fun create(from: PublicKey, seq: Int, fee: Long, type: Byte, data: ByteArray): Transaction {
            return Transaction(Signature.EMPTY, from, seq, Hash.ZERO, fee, type, SerializableByteArray(data))
        }
    }
}