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
import ninja.blacknet.db.Genesis.RegTest
import org.testng.Assert.*
import org.testng.annotations.Test
import java.security.Security

class PaymentIdTest {
    init {
        Security.addProvider(Blake2bProvider())
    }

    @Test
    fun encrypt() {
        val id = "1000000"
        val obj = PaymentId.encrypted(id, RegTest.privateKey1, RegTest.publicKey2)
        val decrypted = obj.decrypt(RegTest.privateKey2, RegTest.publicKey1)

        assertEquals(decrypted, id)
    }

    @Test
    fun decrypt() {
        val id = "\u0072\u0061\u0074\u0034"
        val encrypted = "E81A8A93583291A7C0472DD6A961B8DE"
        val decrypted = PaymentId.decrypt(RegTest.privateKey2, RegTest.publicKey1, encrypted)

        assertEquals(decrypted, id)
    }
}
