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
import kotlin.test.assertTrue

class AddressTest {
    @Test
    fun htlc() {
        val string = "blacknet1q8llal0ul0a0n78h7m6lfulj78cwlmhdan47460gulnwte8ruts7q6rcsw5"
        val address = HashTimeLockContractIdSerializer.parse(string)
        assertTrue(HashTimeLockContractIdSerializer.stringify(address).compareTo(string, ignoreCase = true) == 0)
    }
    @Test
    fun multisig() {
        val string = "blacknet1qtl0ml8mltul3alk7h608uh37rh7am0va04wn688umj7fclzu8sd7467cge"
        val address = MultiSignatureLockContractIdSerializer.parse(string)
        assertTrue(MultiSignatureLockContractIdSerializer.stringify(address).compareTo(string, ignoreCase = true) == 0)
    }
    @Test
    fun bapp() {
        val string = "blacknet1q07le7l69mvwv3"
        val address = BAppIdSerializer.parse(string)
        assertTrue(BAppIdSerializer.stringify(address).compareTo(string, ignoreCase = true) == 0)
    }
}
