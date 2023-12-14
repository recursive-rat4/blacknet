/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.codec.base.decode
import ninja.blacknet.crypto.Address

class AddressTest {
    @Test
    fun account() {
        val string = "blacknet1klnycx794hg9jvuhua0gy75d5v374rrwrlnpg25xpykfxkg30egqq83tj0"
        val bytes = Base16.decode("B7E64C1BC5ADD0593397E75E827A8DA323EA8C6E1FE6142A86092C9359117E50")
        val decoded = Address.decode(string)
        assertEquals(bytes, decoded)
        assertTrue(Address.encode(decoded).compareTo(string, ignoreCase = true) == 0)
    }

    @Test
    fun htlc() {
        val string = "blacknet1q8llal0ul0a0n78h7m6lfulj78cwlmhdan47460gulnwte8ruts7q6rcsw5"
        val address = Address.decode(Address.HTLC, string)
        assertTrue(Address.encode(Address.HTLC, address).compareTo(string, ignoreCase = true) == 0)
    }

    @Test
    fun multisig() {
        val string = "blacknet1qtl0ml8mltul3alk7h608uh37rh7am0va04wn688umj7fclzu8sd7467cge"
        val address = Address.decode(Address.MULTISIG, string)
        assertTrue(Address.encode(Address.MULTISIG, address).compareTo(string, ignoreCase = true) == 0)
    }

    @Test
    fun bapp() {
        val string = "blacknet1q07le7l69mvwv3"
        val address = Address.decode(Address.BAPP, string)
        assertTrue(Address.encode(Address.BAPP, address).compareTo(string, ignoreCase = true) == 0)
    }
}
