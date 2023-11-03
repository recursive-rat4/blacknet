/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class PoSTest {
    @Suppress("KotlinConstantConditions")
    @Test
    fun sanity() {
        assertTrue(PoS.UPGRADE_THRESHOLD >= PoS.MATURITY, "Not strictly required but seems logical")
        assertTrue(PoS.ROLLBACK_LIMIT <= PoS.UPGRADE_THRESHOLD, "Fork activation shouldn't be reversible")
        assertEquals(1, PoS.BLOCK_SIZE_SPAN % 2, "Used for median")
    }

    @Test
    fun maxBlockSize() {
        val blockSizes = ArrayList<Int>(PoS.BLOCK_SIZE_SPAN)

        for (i in 0 until PoS.BLOCK_SIZE_SPAN)
            blockSizes.add(0)
        assertEquals(
            PoS.DEFAULT_MAX_BLOCK_SIZE,
            PoS.maxBlockSize(blockSizes)
        )

        for (i in 0 until PoS.BLOCK_SIZE_SPAN)
            blockSizes[i] = PoS.DEFAULT_MAX_BLOCK_SIZE * 3 / 4
        assertEquals(
            PoS.DEFAULT_MAX_BLOCK_SIZE * 3 / 2,
            PoS.maxBlockSize(blockSizes)
        )

        for (i in 0 until PoS.BLOCK_SIZE_SPAN)
            blockSizes[i] = PoS.DEFAULT_MAX_BLOCK_SIZE
        assertEquals(
            PoS.DEFAULT_MAX_BLOCK_SIZE * 2,
            PoS.maxBlockSize(blockSizes)
        )
    }
}
