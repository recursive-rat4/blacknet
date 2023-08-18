/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec.base

import kotlin.test.Test
import kotlin.test.assertFails
import kotlin.test.assertTrue

class Bech32Test {
    @Test
    fun test1() {
        // https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#Test_vectors

        @Suppress("USELESS_ELVIS")
        for (string in arrayOf(
                "A12UEL5L",
                "a12uel5l",
                "an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1tt5tgs",
                "abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw",
                "11qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqc8247j",
                "split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w",
                "?1ezyfcl"
        )) {
            val (hrp, data) = Bech32.decode(string) ?: throw AssertionError("Bech32.decode failed")
            assertTrue(Bech32.encode(hrp, data).compareTo(string, ignoreCase = true) == 0)
        }

        for ((string, reason) in arrayOf(
                Pair('\u0020' + "1nwldj5", "HRP character out of range"),
                Pair('\u007F' + "1axkwrx", "HRP character out of range"),
                Pair('\u0080' + "1eym55h", "HRP character out of range"),
                Pair("an84characterslonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1569pvx", "Overall max length exceeded"),
                Pair("pzry9x0s0muk", "No separator character"),
                Pair("1pzry9x0s0muk", "Empty HRP"),
                Pair("x1b4n0q5v", "Invalid data character"),
                Pair("li1dgmt3", "Too short checksum"),
                Pair("de1lg7wt" + '\u00FF', "Invalid character in checksum"),
                Pair("A1G7SGD8", "Checksum calculated with uppercase form of HRP"),
                Pair("10a06t8", "Empty HRP"),
                Pair("1qzzfhee", "Empty HRP")
        )) {
            try {
                assertFails { Bech32.decode(string) }
            } catch (e: AssertionError) {
                throw AssertionError(reason)
            }
        }

        @Suppress("USELESS_ELVIS")
        for (string in arrayOf(
                "BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4",
                "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7",
                "bc1pw508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7k7grplx",
                "BC1SW50QA3JX3S",
                "bc1zw508d6qejxtdg4y5r3zarvaryvg6kdaj",
                "tb1qqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesrxh6hy"
        )) {
            val (hrp, data) = Bech32.decode(string) ?: throw AssertionError("Bech32.decode failed")
            assertTrue(Bech32.encode(hrp, data).compareTo(string, ignoreCase = true) == 0)
        }

        @Suppress("ControlFlowWithEmptyBody")
        @Suppress("UNUSED_VARIABLE")
        for ((string, reason) in arrayOf(
                Pair("tc1qw508d6qejxtdg4y5r3zarvary0c5xw7kg3g4ty", "Invalid human-readable part"),
                Pair("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t5", "Invalid checksum"),
                Pair("BC13W508D6QEJXTDG4Y5R3ZARVARY0C5XW7KN40WF2", "Invalid witness version"),
                Pair("bc1rw5uspcuh", "Invalid program length"),
                Pair("bc10w508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7kw5rljs90", "Invalid program length"),
                Pair("BC1QR508D6QEJXTDG4Y5R3ZARVARYV98GJ9P", "Invalid program length for witness version 0 (per BIP141)"),
                Pair("tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sL5k7", "Mixed case"),
                Pair("bc1zw508d6qejxtdg4y5r3zarvaryvqyzf3du", "Zero padding of more than 4 bits"),
                Pair("tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3pjxtptv", "Non-zero padding in 8-to-5 conversion"),
                Pair("bc1gmk9yu", "Empty data section")
        )) {
            //assertNull(SegWit.decode(string), reason)
        }
    }

    @Test
    fun test2() {
        @Suppress("USELESS_ELVIS")
        for (string in arrayOf(
                "blacknet1qqqsyqcyq5rqwzqfpg9scrgwpugpzysnzs23v9ccrydpk8qarc0s209j2k",
                "blacknet1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5z5tpwxqergd3c8g7ruszz3fzfj8",
                "blacknet1qgpsgpgxquyqjzstpsxsurcszyfpx9q4zct3sxg6rvwp68slyqsjy02s4zq"
        )) {
            val (hrp, data) = Bech32.decode(string) ?: throw AssertionError("Bech32.decode failed")
            assertTrue(Bech32.encode(hrp, data).compareTo(string, ignoreCase = true) == 0)
        }
    }
}
