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
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BlacknetOutput
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Transfer(
        val amount: Long,
        val to: PublicKey,
        val message: SerializableByteArray
) : TxData {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Byte {
        return TxType.Transfer.ordinal.toByte()
    }

    override fun process(tx: Transaction): Boolean {
        return false //TODO
    }
}