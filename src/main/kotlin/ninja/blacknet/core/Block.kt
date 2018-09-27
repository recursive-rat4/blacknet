/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.io.core.ByteReadPacket
import kotlinx.io.core.IoBuffer
import kotlinx.io.core.readBytes
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Ed25519
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.crypto.Signature
import ninja.blacknet.serialization.BlacknetInput
import ninja.blacknet.serialization.BlacknetOutput
import ninja.blacknet.serialization.SerializableByteArray
import java.nio.ByteBuffer

@Serializable
class Block(
        val version: Int,
        val previous: Hash,
        val time: Long,
        val generator: PublicKey,
        val sizeVote: Int,
        val transactions: ArrayList<SerializableByteArray>,
        val signature: Signature
) {
    fun serialize(): ByteArray {
        val out = BlacknetOutput()
        out.write(this)
        return out.build().readBytes()
    }

    fun verifySignature(hash: Hash): Boolean {
        return Ed25519.verify(signature, hash, generator)
    }

    companion object {
        fun deserialize(bytes: ByteArray): Block? {
            val buf = IoBuffer(ByteBuffer.wrap(bytes))
            buf.resetForRead()
            val input = ByteReadPacket(buf, IoBuffer.NoPool)
            val block = BlacknetInput(input).read<Block>()
            if (input.remaining > 0) {
                input.release()
                return null
            }
            return block
        }
    }
}