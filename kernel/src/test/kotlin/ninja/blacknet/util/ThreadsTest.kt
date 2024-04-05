/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import java.lang.Thread.sleep
import java.util.concurrent.CountDownLatch
import kotlin.test.Test
import kotlin.test.assertFailsWith
import kotlin.test.assertIs

class ThreadsTest {
    @Test
    fun interruptible() {
        assertIs<Unit>(
            interruptible {
                @Suppress("UNUSED_EXPRESSION")
                0
            }
        )
        assertIs<Unit>(
            interruptible {
                throw InterruptedException()
            }
        )
        assertFailsWith<CustomThrowable> {
            interruptible {
                throw CustomThrowable()
            }
        }
    }

    @Test
    fun startInterruptible() {
        startInterruptible("ThreadsTest::startInterruptible") {
            sleep(Long.MAX_VALUE)
        }.run {
            interrupt()
            join()
        }
    }

    @Test
    fun rotate() {
        val latch = CountDownLatch(4)
        val vt = rotate("ThreadsTest::rotate") {
            latch.countDown()
            if (latch.getCount() == 0L) sleep(Long.MAX_VALUE)
        }
        latch.await()
        vt.interrupt()
        vt.join()
    }

    private class CustomThrowable : Throwable()
}
