/*
 * Copyright (c) 2020-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("SpellCheckingInspection")

package ninja.blacknet.codec.base

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertTrue

// https://github.com/bitcoin/bips/blob/master/bip-0350.mediawiki#user-content-Test_vectors

class Bech32mTest {
    @Test
    fun validBech32m() {
        @Suppress("USELESS_ELVIS")
        for (string in arrayOf(
            "A1LQFN3A",
            "a1lqfn3a",
            "an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11sg7hg6",
            "abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx",
            "11llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllludsr8",
            "split1checkupstagehandshakeupstreamerranterredcaperredlc445v",
            "?1v759aa",
        )) {
            val (hrp, data) = Bech32m.decode(string) ?: throw AssertionError("Bech32m.decode failed")
            assertTrue(Bech32m.encode(hrp, data).compareTo(string, ignoreCase = true) == 0)
        }
    }

    @Test
    fun notValidBech32m() {
        for ((string, reason) in arrayOf(
            Pair('\u0020' + "1xj0phk", "HRP character out of range"),
            Pair('\u007F' + "1g6xzxy", "HRP character out of range"),
            Pair('\u0080' + "1vctc34", "HRP character out of range"),
            Pair("an84characterslonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11d6pts4", "Overall max length exceeded"),
            Pair("qyrz8wqd2c9m", "No separator character"),
            Pair("1qyrz8wqd2c9m", "Empty HRP"),
            Pair("y1b0jsk6g", "Invalid data character"),
            Pair("lt1igcx5c0", "Invalid data character"),
            Pair("in1muywd", "Too short checksum"),
            Pair("mm1crxm3i", "Invalid character in checksum"),
            Pair("au1s5cgom", "Invalid character in checksum"),
            Pair("M1VUXWEZ", "Checksum calculated with uppercase form of HRP"),
            Pair("16plkw9", "Empty HRP"),
            Pair("1p2gdwpf", "Empty HRP"),
        )) {
            try {
                assertFails { Bech32m.decode(string) }
            } catch (e: AssertionError) {
                throw AssertionError(reason)
            }
        }
    }

    @Test
    fun validSegwitAddresses() {
        @Suppress("USELESS_ELVIS")
        for ((string, hex, m) in arrayOf(
            Triple("BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4", "0014751e76e8199196d454941c45d1b3a323f1433bd6", false),
            Triple("tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7", "00201863143c14c5166804bd19203356da136c985678cd4d27a1b8c6329604903262", false),
            Triple("bc1pw508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7kt5nd6y", "5128751e76e8199196d454941c45d1b3a323f1433bd6751e76e8199196d454941c45d1b3a323f1433bd6", true),
            Triple("BC1SW50QGDZ25J", "6002751e", true),
            Triple("bc1zw508d6qejxtdg4y5r3zarvaryvaxxpcs", "5210751e76e8199196d454941c45d1b3a323", true),
            Triple("tb1qqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesrxh6hy", "0020000000c4a5cad46221b2a187905e5266362b99d5e91c6ce24d165dab93e86433", false),
            Triple("tb1pqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesf3hn0c", "5120000000c4a5cad46221b2a187905e5266362b99d5e91c6ce24d165dab93e86433", true),
            Triple("bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqzk5jj0", "512079be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", true),
        )) {
            if (m) {
                val (_, data) = Bech32m.decode(string) ?: throw AssertionError("Bech32m.decode failed")
                // strip out segwit details
                val strippedData = data.drop(1).toByteArray()
                val strippedHex = hex.drop(4)
                val hexDecoded = Base16.decode(strippedHex)
                val hexConverted = Bech32m.convertBits(hexDecoded, 8, 5, true)
                assertEquals(hexConverted, strippedData)
            }
        }
    }

    @Test
    fun invalidSegwitAddresses() {
        @Suppress("USELESS_ELVIS")
        for ((string, reason, segwit) in arrayOf(
            Triple("tc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vq5zuyut", "Invalid human-readable part", true),
            Triple("bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqh2y7hd", "Invalid checksum (Bech32 instead of Bech32m)", false),
            Triple("tb1z0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqglt7rf", "Invalid checksum (Bech32 instead of Bech32m)", false),
            Triple("BC1S0XLXVLHEMJA6C4DQV22UAPCTQUPFHLXM9H8Z3K2E72Q4K9HCZ7VQ54WELL", "Invalid checksum (Bech32 instead of Bech32m)", false),
            Triple("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kemeawh", "Invalid checksum (Bech32m instead of Bech32)", true),
            Triple("tb1q0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vq24jc47", "Invalid checksum (Bech32m instead of Bech32)", true),
            Triple("bc1p38j9r5y49hruaue7wxjce0updqjuyyx0kh56v8s25huc6995vvpql3jow4", "Invalid character in checksum", false),
            Triple("BC130XLXVLHEMJA6C4DQV22UAPCTQUPFHLXM9H8Z3K2E72Q4K9HCZ7VQ7ZWS8R", "Invalid witness version", true),
            Triple("bc1pw5dgrnzv", "Invalid program length", true),
            Triple("bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7v8n0nx0muaewav253zgeav", "Invalid program length (41 bytes)", true),
            Triple("BC1QR508D6QEJXTDG4Y5R3ZARVARYV98GJ9P", "Invalid program length for witness version 0 (per BIP141)", false), // the reason is segwit, but actually Bech32
            Triple("tb1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vq47Zagq", "Mixed case", false),
            Triple("bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7v07qwwzcrf", "Zero padding of more than 4 bits", true),
            Triple("tb1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vpggkg4j", "Non-zero padding in 8-to-5 conversion", true),
            Triple("bc1gmk9yu", "Empty data section", false),
        )) {
            if (segwit) {
                val (hrp, data) = Bech32m.decode(string) ?: throw AssertionError("Bech32m.decode failed")
                assertTrue(Bech32m.encode(hrp, data).compareTo(string, ignoreCase = true) == 0)
            } else {
                try {
                    assertFails { Bech32m.decode(string) }
                } catch (e: AssertionError) {
                    throw AssertionError(reason)
                }
            }
        }
    }

    @Test
    fun blacknet() {
        // Bech32
        for (string in arrayOf(
            "blacknet1qqqsyqcyq5rqwzqfpg9scrgwpugpzysnzs23v9ccrydpk8qarc0s209j2k",
            "blacknet1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5z5tpwxqergd3c8g7ruszz3fzfj8",
            "blacknet1qgpsgpgxquyqjzstpsxsurcszyfpx9q4zct3sxg6rvwp68slyqsjy02s4zq"
        )) {
            assertFails { Bech32m.decode(string) }
        }
    }
}
