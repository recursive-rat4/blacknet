/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import ninja.blacknet.core.BundlePool
import ninja.blacknet.core.TxPool
import ninja.blacknet.db.BlockDB
import ninja.blacknet.core.DataDB

enum class DataType(val db: DataDB) {
    Block(BlockDB),
    Transaction(TxPool),
    Bundle(BundlePool),
    ;

    companion object {
        const val MAX_INVENTORY = 50000
        const val MAX_DATA = 1000
    }
}