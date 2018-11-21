/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey

data class UndoBlock(
        val blockTime: Long,
        val difficulty: BigInt,
        val cumulativeDifficulty: BigInt,
        val supply: Long,
        val nxtrng: Hash,
        val accounts: UndoList,
        val htlcs: UndoHTLCList,
        val multisigs: UndoMultisigList
) {
    fun add(publicKey: PublicKey, state: AccountState) = accounts.add(Pair(publicKey, state))
    fun addHTLC(id: Hash, htlc: HTLC?) = htlcs.add(Pair(id, htlc))
    fun addMultisig(id: Hash, multisig: Multisig?) = multisigs.add(Pair(id, multisig))
}

typealias UndoList = ArrayList<Pair<PublicKey, AccountState>>
typealias UndoHTLCList = ArrayList<Pair<Hash, HTLC?>>
typealias UndoMultisigList = ArrayList<Pair<Hash, Multisig?>>
