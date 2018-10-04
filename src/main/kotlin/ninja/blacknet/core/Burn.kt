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
import ninja.blacknet.serialization.BlacknetOutput
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Burn(
        val amount: Long,
        val message: SerializableByteArray
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetOutput()
        out.write(this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.Burn.ordinal.toByte()
    }

    override fun processImpl(tx: Transaction, account: AccountState, ledger: Ledger): Boolean {
        if (!account.credit(amount)) {
            return false
        }
        ledger.addSupply(-amount)
        return true
    }
}