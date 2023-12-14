/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.VarIntSerializer
import ninja.blacknet.serialization.VarLongSerializer

@Serializable
class ChainIndex(
        val previous: Hash,
        var next: Hash,
        @Serializable(with = VarIntSerializer::class)
        var nextSize: Int,
        @Serializable(with = VarIntSerializer::class)
        val height: Int,
        @Serializable(with = VarLongSerializer::class)
        val generated: Long
) {

}
