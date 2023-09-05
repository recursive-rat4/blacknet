/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.codec.base.Base16
import ninja.blacknet.codec.base.decode
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.HashSerializer
import ninja.blacknet.crypto.SignatureSerializer
import ninja.blacknet.serialization.bbf.binaryFormat
import kotlin.test.Test
import kotlin.test.assertEquals

class BlockTest {
    val block = Block(
        0u,
        HashSerializer.ZERO,
        1545556624L,
        Address.decode("blacknet1klnycx794hg9jvuhua0gy75d5v374rrwrlnpg25xpykfxkg30egqq83tj0"),
        HashSerializer.decode("45B0CFC220CEEC5B7C1C62C4D4193D38E4EBA48E8815729CE75F9C0AB0E4C1C0"),
        SignatureSerializer.decode("0BD14B678ED7C9C5E44E4C2EF6307416B44CFE3315D17345DAF80EF60CD684A5AABDFD0DA0983ED1EC8B3797E49D89053BE49FA2149597FB3E14AAA48DE02505"),
        ArrayList()
    )

    val raw =
        Base16.decode("000000000000000000000000000000000000000000000000000000000000000000000000000000005C1F5290B7E64C1BC5ADD0593397E75E827A8DA323EA8C6E1FE6142A86092C9359117E5045B0CFC220CEEC5B7C1C62C4D4193D38E4EBA48E8815729CE75F9C0AB0E4C1C00BD14B678ED7C9C5E44E4C2EF6307416B44CFE3315D17345DAF80EF60CD684A5AABDFD0DA0983ED1EC8B3797E49D89053BE49FA2149597FB3E14AAA48DE0250580")

    @Test
    fun serialize() {
        assertEquals(
            raw,
            binaryFormat.encodeToByteArray(Block.serializer(), block)
        )
    }

    @Test
    fun deserialize() {
        binaryFormat.decodeFromByteArray(Block.serializer(), raw)
    }
}
