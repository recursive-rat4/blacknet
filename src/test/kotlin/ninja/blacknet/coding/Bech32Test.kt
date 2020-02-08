/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.coding

import org.testng.Assert.*
import org.testng.annotations.Test

class Bech32Test {
    @Test
    fun test1() {
        // https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#Test_vectors

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

        for (string in arrayOf(
                '\u0020' + "1nwldj5",
                '\u007F' + "1axkwrx",
                '\u0080' + "1eym55h",
                "an84characterslonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1569pvx",
                "pzry9x0s0muk",
                "1pzry9x0s0muk",
                "x1b4n0q5v",
                "li1dgmt3",
                "de1lg7wt",
                "A1G7SGD8",
                "10a06t8",
                "1qzzfhee"
        )) {
            assertNull(Bech32.decode(string))
        }

    }

    @Test
    fun test2() {
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
