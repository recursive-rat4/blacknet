/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.network.Address
import org.mapdb.DBMaker

object PeerDB {
    private val db = DBMaker.fileDB("peer.db").transactionEnable().fileMmapEnable().closeOnJvmShutdown().make()

    fun getRandom(n: Int): ArrayList<Address> {
        return ArrayList() //TODO
    }

    fun add(peers: ArrayList<Address>, from: Address) {
        //TODO
    }
}