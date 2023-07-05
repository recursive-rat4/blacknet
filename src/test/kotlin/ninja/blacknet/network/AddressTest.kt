/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import java.security.Security
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertFalse
import kotlin.test.assertTrue
import org.bouncycastle.jce.provider.BouncyCastleProvider

class AddressTest {
    @Test
    fun ipv4() {
        for ((string, local, private) in arrayOf(
            Triple("0.0.0.0", true, false),
            Triple("100.64.0.0", false, true),
            Triple("100.128.0.0", false, false),
            Triple("127.0.1.4", true, false),
            Triple("255.255.255.255", false, false)
        )) {
            val address = Network.parse(string, 0) ?: throw AssertionError("Network.parse failed")
            assertEquals(string, address.getAddressString())
            assertEquals(local, address.isLocal())
            assertEquals(private, address.isPrivate())
            address.getSocketAddress()
        }
    }

    @Test
    fun ipv6() {
        for (string in arrayOf(
            "1001:1001:1001:1001:1001:1001:1001:1001",
            "2001:8db8:8558:8888:1331:8aa8:3789:7337",
            "F00F:F00F:F00F:F00F:F00F:F00F:F00F:F00F"
        )) {
            val address = Network.parse(string, 0) ?: throw AssertionError("Network.parse failed")
            assertTrue(address.getAddressString().compareTo(string, ignoreCase = true) == 0)
            address.getSocketAddress()
        }
    }

    @Test
    fun torv3() {
        Security.addProvider(BouncyCastleProvider())

        // https://gitweb.torproject.org/torspec.git/tree/rend-spec-v3.txt
        for (string in arrayOf(
            "pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion",
            "sp3k262uwy4r2k3ycr5awluarykdpag6a7y33jxop4cs2lu5uz5sseqd.onion",
            "xa4r2iadxm55fbnqgwwi5mymqdcofiu3w6rpbtqn7b2dyn7mgwj64jyd.onion"
        )) {
            val address = Network.parse(string, 0) ?: throw AssertionError("Network.parse failed")
            assertTrue(address.getAddressString().compareTo(string, ignoreCase = true) == 0)
            assertFalse(address.isLocal())
            assertFalse(address.isPrivate())
            assertFails { address.getSocketAddress() }
        }
    }

    @Test
    fun i2p() {
        for (string in arrayOf(
            "y45f23mb2apgywmftrjmfg35oynzfwjed7rxs2mh76pbdeh4fatq.b32.i2p"
        )) {
            val address = Network.parse(string, 0) ?: throw AssertionError("Network.parse failed")
            assertTrue(address.getAddressString().compareTo(string, ignoreCase = true) == 0)
            assertFalse(address.isLocal())
            assertFalse(address.isPrivate())
            assertFails { address.getSocketAddress() }
        }
    }
}
