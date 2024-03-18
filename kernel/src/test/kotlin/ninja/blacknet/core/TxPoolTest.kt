/*
 * Copyright (c) 2021-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue
import ninja.blacknet.Config
import ninja.blacknet.crypto.PoS
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.MemDB

class TxPoolTest {
    private val txPool = TxPool(
        Config(),
        BlockDB(MemDB()),
    )

    @Test
    fun setFee() {
        txPool.minFeeRate = 0
        assertEquals(0, txPool.minFeeRate)

        txPool.minFeeRate = 4 * PoS.COIN
        assertEquals(4 * PoS.COIN, txPool.minFeeRate)
    }

    @Test
    fun checkFee() {
        txPool.minFeeRate = 100000

        for ((size, amount) in arrayOf(
                Pair(184, 100000L),
                Pair(216, 100000L),
                Pair(194, 100000L),
                Pair(999, 100000L),
        )) {
            assertTrue(txPool.checkFee(size, amount))
        }

        for ((size, amount) in arrayOf(
                Pair(184, 0L),
                Pair(216, 10000L),
                Pair(194, 50000L),
                Pair(1000, 100000L),
        )) {
            assertFalse(txPool.checkFee(size, amount))
        }
    }
}
