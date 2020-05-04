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

class MessageTest {
    init {
        Security.addProvider(Blake2bProvider())
    }

    @Test
    fun sign() {
        val message = "Crab Beat"
        val signature = Message.sign(RegTest.privateKey1, message)

        assertTrue(Message.verify(RegTest.publicKey1, signature, message))
    }

    @Test
    fun verify() {
        val message = "Crab Rave"
        val signature1 = Signature.parse("A64576A3CADFEBC2350542CC22ACF7EE3FF90AA90B0684C3C90A03FA03F67C653BE20DF0DF87A2E205C79A17719D2E1E46E9763DF016A3EE28414AB31DF96A0E")
        val signature2 = Signature.EMPTY

        assertTrue(Message.verify(RegTest.publicKey1, signature1, message))
        assertFalse(Message.verify(RegTest.publicKey1, signature2, message))
    }

    @Test
    fun encrypt() {
        val id = "1000000"
        val obj = Message.encrypted(id, RegTest.privateKey1, RegTest.publicKey2)
        val decrypted = obj.decrypt(RegTest.privateKey2, RegTest.publicKey1)

        assertEquals(decrypted, id)
    }

    @Test
    fun decrypt() {
        val id = "\u0072\u0061\u0074\u0034"
        val encrypted = "E81A8A93583291A7C0472DD6A961B8DE"
        val decrypted = Message.decrypt(RegTest.privateKey2, RegTest.publicKey1, encrypted)

        assertEquals(decrypted, id)
    }
}
