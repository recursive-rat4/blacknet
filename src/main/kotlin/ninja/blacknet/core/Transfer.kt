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
import ninja.blacknet.crypto.Message
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BlacknetOutput

@Serializable
class Transfer(
        val amount: Long,
        val to: PublicKey,
        val message: Message
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetOutput()
        out.write(this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.Transfer.ordinal.toByte()
    }

    override fun processImpl(tx: Transaction, account: AccountState, ledger: Ledger): Boolean {
        if (message.type == Message.ENCRYPTED)
            TODO()
        if (!account.credit(amount))
            return false
        val toAccount = ledger.get(to) ?: AccountState.create()
        toAccount.debit(ledger.height(), amount)
        ledger.set(to, toAccount)
        return true
    }
}