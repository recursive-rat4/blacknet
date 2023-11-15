/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import ninja.blacknet.contract.BAppIdSerializer
import ninja.blacknet.db.PeerDB.NetworkStat
import ninja.blacknet.db.PeerDB.UptimeStat
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.util.byteArrayOfInts
import ninja.blacknet.util.hashMapOf

class PeerDBTest {
    @Test
    fun networkStat() {
        val stat = NetworkStat(
            0,
            "",
            UptimeStat(0f, 0f, 0f),
            UptimeStat(0f, 0f, 0f),
            UptimeStat(0f, 0f, 0f),
            UptimeStat(0f, 0f, 0f),
            UptimeStat(0f, 0f, 0f),
            hashMapOf(
                ByteArray(BAppIdSerializer.SIZE_BYTES) to Unit,
            ),
        )
        val bytes = byteArrayOfInts(
            0, 0, 0, 0, 0, 0, 0, 0,
            128,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            128 + 1, 0, 0, 0, 0,
        )
        assertIs<NetworkStat>(
            binaryFormat.decodeFromByteArray(NetworkStat.serializer(), bytes)
        )
        assertEquals(
            bytes,
            binaryFormat.encodeToByteArray(NetworkStat.serializer(), stat)
        )
    }
}
