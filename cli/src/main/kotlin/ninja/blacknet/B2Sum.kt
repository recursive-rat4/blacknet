/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.rfksystems.blake2b.Blake2b
import java.nio.file.Files
import java.nio.file.Path
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.codec.base.encode

object B2Sum {
    @JvmStatic
    fun main(args: Array<String>) {
        val threads = ArrayList<Thread>(args.size)

        val DIGEST_SIZE_BITS = 256
        val DIGEST_SIZE_BYTES = DIGEST_SIZE_BITS / Byte.SIZE_BITS

        args.forEach { arg ->
            val vThread = Thread.ofVirtual().start {
                val path = Path.of(arg)
                val stream = try {
                    Files.newInputStream(path)
                } catch (e: Throwable) {
                    println("B2Sum: ${e.message} ${e::class.simpleName}")
                    return@start
                }
                val b2 = Blake2b(DIGEST_SIZE_BITS)
                val buffer = ByteArray(DEFAULT_BUFFER_SIZE)

                while (true) {
                    val n = stream.read(buffer)
                    if (n > 0) {
                        b2.update(buffer, 0, n)
                    } else {
                        break
                    }
                }

                stream.close()
                val bytes = ByteArray(DIGEST_SIZE_BYTES)
                b2.digest(bytes, 0)
                println("${Base16.encode(bytes)} $arg")
            }

            threads.add(vThread)
        }

        threads.forEach(Thread::join)
    }
}
