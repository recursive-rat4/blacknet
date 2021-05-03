/*
 * Copyright (c) 2021 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlin.test.Test
import kotlin.test.assertEquals
import ninja.blacknet.crypto.PoS

class TxPoolTest {
    @Test
    fun initialization() {
        TxPool
    }

    @Test
    fun setFee() {
        TxPool.minFeeRate = 0
        assertEquals(0, TxPool.minFeeRate)

        TxPool.minFeeRate = 4 * PoS.COIN
        assertEquals(4 * PoS.COIN, TxPool.minFeeRate)
    }
}
