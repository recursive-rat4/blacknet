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
import kotlin.test.assertFalse
import kotlin.test.assertTrue
import ninja.blacknet.db.Genesis.RegTest

class MessageTest {
    init {
        Security.addProvider(Blake2bProvider())
    }

    @Test
    fun signAndVerify() {
        val message = "Blacknet test message 1"
        val signature = Message.sign(RegTest.privateKey1, message)

        assertTrue(Message.verify(RegTest.publicKey1, signature, message))
    }

    @Test
    fun testVectors() {
        val magic = "Blacknet Signed Message:\n" // don't break test in forks
        val message = "Blacknet test message 2"
        val signature1 = SignatureSerializer.decode("6D5D4F6A81C601B1834701BDE84785470F92DFA517975BED9AAEA035FBDB0072327EFD207195B7202B5A72BB9CC37443A011C35137E1DF1C11BB5E9C60125B04")
        val signature2 = SignatureSerializer.EMPTY

        assertTrue(Message.verifyWithMagic(RegTest.publicKey1, signature1, message, magic))
        assertFalse(Message.verifyWithMagic(RegTest.publicKey1, signature2, message, magic))
    }
}
