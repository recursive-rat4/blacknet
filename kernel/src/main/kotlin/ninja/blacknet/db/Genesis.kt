/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import java.math.BigInteger
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import ninja.blacknet.Mode.*
import ninja.blacknet.crypto.*
import ninja.blacknet.mode
import ninja.blacknet.serialization.json.json
import ninja.blacknet.util.Resources

object Genesis {
    const val TIME: Long = 1545555600
    val BLOCK_HASH = Hash.ZERO
    val CUMULATIVE_DIFFICULTY = BigInteger.ZERO

    val balances by lazy {
        val map = HashMap<PublicKey, Long>()

        when (mode) {
            MainNet -> {
                val genesis = Resources.string(Genesis::class.java, "genesis.json", Charsets.UTF_8)
                val entries = json.decodeFromString(ListSerializer(GenesisJsonEntry.serializer()), genesis)
                entries.forEach { entry ->
                    map.put(entry.publicKey, entry.balance)
                }
            }
            TestNet -> {
                throw NotImplementedError("$mode genesis.json is missing")
            }
            SigNet -> {
                throw NotImplementedError("$mode genesis.json is missing")
            }
            RegTest -> {
                map.put(RegTestGenesis.publicKey1, 1000000000 * PoS.COIN)
                map.put(RegTestGenesis.publicKey2, 10101010 * PoS.COIN)
            }
        }

        map
    }

    @Serializable
    private class GenesisJsonEntry(
            val publicKey: PublicKey,
            val balance: Long
    )

    object RegTestGenesis {
        // rblacknet1y73v0n57axhsgkyrypusz7jlhwclz4gextzvhyqnj6awjhmapu9qklf7u2
        val mnemonic1 = "疗 昨 示 穿 偏 贷 五 袁 色 烂 撒 殖"
        val privateKey1 = Mnemonic.fromString(mnemonic1)
        val publicKey1 = Ed25519.toPublicKey(privateKey1)

        // rblacknet15edw70jp9qp39pdlqdncxtpc45fkdg0g6h3et0xu0gtu8v5t4vwspmsgfx
        val mnemonic2 = "胡 允 空 桥 料 状 纱 角 钠 灌 绝 件"
        val privateKey2 = Mnemonic.fromString(mnemonic2)
        val publicKey2 = Ed25519.toPublicKey(privateKey2)
    }
}
