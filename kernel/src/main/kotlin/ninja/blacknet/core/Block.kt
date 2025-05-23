/*
 * Copyright (c) 2018-2020 Pavel Vasin
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
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.serialization.ByteArraySerializer

@Serializable
class Block(
        val version: UInt,
        val previous: Hash,
        val time: Long,
        val generator: PublicKey,
        var contentHash: Hash,
        @Serializable(with = SignatureSerializer::class)
        var signature: ByteArray,
        val transactions: ArrayList<@Serializable(ByteArraySerializer::class) ByteArray>
) {
    fun sign(privateKey: ByteArray): Pair<Hash, ByteArray> {
        val bytes = binaryFormat.encodeToByteArray(serializer(), this)
        contentHash = contentHash(bytes)
        System.arraycopy(contentHash.bytes, 0, bytes, CONTENT_HASH_POS, Hash.SIZE_BYTES)
        val hash = hash(bytes)
        signature = Ed25519.sign(hash, privateKey)
        System.arraycopy(signature, 0, bytes, SIGNATURE_POS, SignatureSerializer.SIZE_BYTES)
        return Pair(hash, bytes)
    }

    fun verifyContentHash(bytes: ByteArray): Boolean {
        return contentHash == contentHash(bytes)
    }

    fun verifySignature(hash: Hash): Boolean {
        return Ed25519.verify(signature, hash, generator)
    }

    private fun contentHash(bytes: ByteArray) = Hash(
        buildHash {
            encodeByteArray(bytes, HEADER_SIZE_BYTES, bytes.size - HEADER_SIZE_BYTES)
        }
    )

    companion object {
        const val VERSION = 2u
        const val CONTENT_HASH_POS = Int.SIZE_BYTES + Hash.SIZE_BYTES + Long.SIZE_BYTES + PublicKey.SIZE_BYTES
        const val SIGNATURE_POS = CONTENT_HASH_POS + Hash.SIZE_BYTES
        const val HEADER_SIZE_BYTES = SIGNATURE_POS + SignatureSerializer.SIZE_BYTES

        fun hash(bytes: ByteArray) = Hash(
            buildHash {
                encodeByteArray(bytes, 0, HEADER_SIZE_BYTES - SignatureSerializer.SIZE_BYTES)
            }
        )

        fun create(previous: Hash, time: Long, generator: PublicKey): Block {
            return Block(VERSION, previous, time, generator, Hash.ZERO, SignatureSerializer.EMPTY, ArrayList())
        }
    }
}
