/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.SystemService
import kotlin.random.Random

@SystemService
object Salt {
    private val salt = Random.nextInt()

    /**
     * Builds a hash code with given [init] builder.
     */
    fun hashCode(init: HashCoder.() -> Unit): Int {
        val coder = HashCoder()
        coder.init()
        return coder.result()
    }

    /**
     * DSL builder for a hash code.
     */
    class HashCoder(private var x: Int) {
        internal constructor() : this(salt)

        /**
         * Adds [Byte] value.
         */
        fun x(byte: Byte) {
            f(byte.toInt())
        }

        /**
         * Adds [Short] value.
         */
        fun x(short: Short) {
            f(short.toInt())
        }

        /**
         * Adds [Int] value.
         */
        fun x(int: Int) {
            f(int)
        }

        /**
         * Adds [Long] value.
         */
        fun x(long: Long) {
            f((long / 4294967296L + long).toInt())
        }

        /**
         * Adds [ByteArray] value.
         */
        fun x(bytes: ByteArray) {
            for (i in 0 until bytes.size)
                f(bytes[i].toInt())
        }

        internal fun result(): Int {
            return x
        }

        private fun f(x: Int) {
            this.x = 31 * this.x + x;
        }
    }
}
