/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.security.SecureRandom

object Mnemonic {
    private const val WORDLIST_SIZE = 2048
    private const val WORDS = 12
    private val random = SecureRandom()

    fun generate(wordlist: Array<String>): Pair<String, PrivateKey> {
        require(wordlist.size == WORDLIST_SIZE) { "Wordlist size must be $WORDLIST_SIZE" }

        val builder = StringBuilder(12 * WORDS)

        while (true) {
            for (i in 1..WORDS) {
                val rnd = random.nextInt(WORDLIST_SIZE)
                builder.append(wordlist[rnd])
                if (i < WORDS) builder.append(' ')
            }

            val mnemonic = builder.toString()
            val hash = hash(mnemonic)
            if (checkVersion(hash.bytes))
                return Pair(mnemonic, PrivateKey(hash.bytes))

            builder.setLength(0)
        }
    }

    fun fromString(string: String?): PrivateKey? {
        if (string == null)
            return null
        val hash = hash(string)
        if (checkVersion(hash.bytes))
            return PrivateKey(hash.bytes)
        return null
    }

    private fun checkVersion(bytes: ByteArray): Boolean {
        return bytes[0].toInt() and 0xF0 == 0x10
    }

    private fun hash(string: String): Hash {
        return Blake2b.hash(string)
    }
}
