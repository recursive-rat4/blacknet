/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.crypto.Hash
import org.mapdb.DBMaker
import org.mapdb.Serializer

object BlockDB {
    private val db = DBMaker.fileDB("block.db").transactionEnable().fileMmapEnableIfSupported().closeOnJvmShutdown().make()
    private val map = db.hashMap("blocks", HashSerializer, Serializer.BYTE_ARRAY).createOrOpen()

    fun contains(hash: Hash): Boolean {
        return map.contains(hash)
    }
}