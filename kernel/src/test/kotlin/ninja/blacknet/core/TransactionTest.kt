/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlin.test.Test
import kotlin.test.assertNotEquals
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.crypto.SignatureSerializer
import ninja.blacknet.db.Genesis

class TransactionTest {
    @Test
    fun sign() {
        val newTx = Transaction.create(
            PublicKey(ByteArray(PublicKey.SIZE_BYTES)),
            0,
            Hash.ZERO,
            0,
            0u,
            ByteArray(0),
        )
        newTx.sign(Genesis.RegTestGenesis.privateKey1)
        assertNotEquals(SignatureSerializer.EMPTY, newTx.signature)
    }
}
