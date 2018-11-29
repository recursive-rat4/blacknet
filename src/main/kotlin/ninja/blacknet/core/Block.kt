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
class Block(
        val version: Int,
        val previous: Hash,
        val time: Long,
        val generator: PublicKey,
        var contentHash: Hash,
        var signature: Signature,
        val transactions: ArrayList<SerializableByteArray>
) {
    fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    fun sign(privateKey: PrivateKey): Pair<Hash, ByteArray> {
        val bytes = serialize()
        contentHash = contentHash(bytes)
        System.arraycopy(contentHash.bytes.array, 0, bytes, CONTENT_HASH_POS, Hash.SIZE)
        val hash = Hasher(bytes)
        signature = Ed25519.sign(hash, privateKey)
        System.arraycopy(signature.bytes.array, 0, bytes, SIGNATURE_POS, Signature.SIZE)
        return Pair(hash, bytes)
    }

    fun verifyContentHash(bytes: ByteArray): Boolean {
        return contentHash == contentHash(bytes)
    }

    fun verifySignature(hash: Hash): Boolean {
        return Ed25519.verify(signature, hash, generator)
    }

    private fun contentHash(bytes: ByteArray): Hash {
        return Blake2b.hash(bytes, HEADER_SIZE, bytes.size - HEADER_SIZE)
    }

    object Hasher : (ByteArray) -> Hash {
        override fun invoke(bytes: ByteArray): Hash {
            return Blake2b.hash(bytes, 0, HEADER_SIZE - Signature.SIZE)
        }
    }

    companion object {
        const val CONTENT_HASH_POS = 4 + Hash.SIZE + 8 + PublicKey.SIZE
        const val SIGNATURE_POS = CONTENT_HASH_POS + Hash.SIZE
        const val HEADER_SIZE = SIGNATURE_POS + Signature.SIZE

        fun deserialize(bytes: ByteArray): Block? {
            return BlacknetDecoder.fromBytes(bytes).decode(Block.serializer())
        }
    }
}