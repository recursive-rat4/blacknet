/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlin.test.Test
import kotlin.test.assertEquals
import ninja.blacknet.db.WalletDB.TransactionDataType
import ninja.blacknet.db.WalletDB.Wallet
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.serialization.json.json
import ninja.blacknet.util.byteArrayOfInts

class WalletDBTest {
    @Test
    fun transactionDataTypeSerialization() {
        val txDataType = TransactionDataType(
            254u,
            0u,
        )
        val bytes = byteArrayOfInts(
            254,
            0,
        )
        val string = """{"type":254,"dataIndex":0}"""
        binaryFormat.decodeFromByteArray(TransactionDataType.serializer(), bytes)
        assertEquals(
            bytes,
            binaryFormat.encodeToByteArray(TransactionDataType.serializer(), txDataType)
        )
        json.decodeFromString(TransactionDataType.serializer(), string)
        assertEquals(
            string,
            json.encodeToString(TransactionDataType.serializer(), txDataType)
        )
    }

    @Test
    fun walletSerialization() {
        val wallet = Wallet()
        val bytes = byteArrayOfInts(
            0x80,
            0x80,
            0x80,
            0x80,
            0x80,
        )
        binaryFormat.decodeFromByteArray(Wallet.serializer(), bytes)
        assertEquals(
            bytes,
            binaryFormat.encodeToByteArray(Wallet.serializer(), wallet)
        )
    }

}
