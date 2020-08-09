/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.list
import ninja.blacknet.Config
import ninja.blacknet.crypto.Ed25519
import ninja.blacknet.crypto.Mnemonic
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKeySerializer
import ninja.blacknet.serialization.Json
import ninja.blacknet.util.HashMap
import ninja.blacknet.util.Resources

object Genesis {
    const val TIME: Long = 1545555600

    val balances by lazy {
        val map = HashMap<ByteArray, Long>()

        if (Config.instance.regtest) {
            map.put(RegTest.publicKey1, 1000000000 * PoS.COIN)
            map.put(RegTest.publicKey2, 10101010 * PoS.COIN)
        } else {
            val genesis = Resources.string(this, "genesis.json", Charsets.UTF_8)
            val entries = Json.parse(GenesisJsonEntry.serializer().list, genesis)
            entries.forEach { entry ->
                map.put(entry.publicKey, entry.balance)
            }
        }

        map
    }

    @Serializable
    private class GenesisJsonEntry(
            @Serializable(with = PublicKeySerializer::class)
            val publicKey: ByteArray,
            val balance: Long
    )

    object RegTest {
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
