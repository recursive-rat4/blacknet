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
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.jsonObject
import ninja.blacknet.contract.BAppId
import ninja.blacknet.db.PeerDB.NetworkStat
import ninja.blacknet.db.PeerDB.UptimeStat
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.serialization.json.json
import ninja.blacknet.util.byteArrayOfInts

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
                BAppId(ByteArray(BAppId.SIZE_BYTES)) to Unit,
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
        assertIs<JsonObject>(
            json.encodeToJsonElement(NetworkStat.serializer(), stat).jsonObject.get("subnetworks")
        )
    }
}
