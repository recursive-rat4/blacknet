/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import com.rfksystems.blake2b.security.Blake2bProvider
import java.security.Security
import kotlin.test.Test
import kotlin.test.assertEquals
import ninja.blacknet.db.Genesis.RegTestGenesis

class PaymentIdTest {
    init {
        Security.addProvider(Blake2bProvider())
    }

    @Test
    fun encrypt() {
        val id = "1000000"
        val obj = PaymentId.encrypted(id, RegTestGenesis.privateKey1, RegTestGenesis.publicKey2)
        val decrypted = obj.decrypt(RegTestGenesis.privateKey2, RegTestGenesis.publicKey1)

        assertEquals(id, decrypted)
    }

    @Test
    fun decrypt() {
        val id = "id"
        val encrypted = "CDCEF8D208A645DB78358859C10F"
        val decrypted = PaymentId.decrypt(RegTestGenesis.privateKey2, RegTestGenesis.publicKey1, encrypted)

        assertEquals(id, decrypted)
    }
}
