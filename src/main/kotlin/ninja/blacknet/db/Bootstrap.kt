/*
 * Copyright (c) 2019-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlinx.coroutines.runBlocking
import mu.KotlinLogging
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.AlreadyHave
import ninja.blacknet.core.Block
import ninja.blacknet.crypto.HashSerializer
import ninja.blacknet.dataDir
import ninja.blacknet.logging.info
import ninja.blacknet.util.buffered
import ninja.blacknet.util.data
import ninja.blacknet.util.moveFile
import java.io.File

private val logger = KotlinLogging.logger {}

object Bootstrap {
    /**
     * Imports a bootstrap if the file exists.
     */
    fun import() {
        val bootstrap = File(dataDir, "bootstrap.dat")
        if (bootstrap.exists()) {
            runBlocking {
                logger.info("Found bootstrap")
                var n = 0

                try {
                    bootstrap.inputStream().buffered().data().use {
                        while (true) {
                            val size = it.readInt()
                            val bytes = ByteArray(size)
                            it.readFully(bytes)

                            val hash = Block.hash(bytes)
                            val status = BlockDB.processImpl(hash, bytes)
                            if (status == Accepted) {
                                if (++n % 50000 == 0)
                                    logger.info("Processed $n blocks")
                                LedgerDB.pruneImpl()
                            } else if (status !is AlreadyHave) {
                                logger.info("$status block ${HashSerializer.encode(hash)}")
                                break
                            }
                        }
                    }
                } catch (e: Throwable) {
                    logger.info(e)
                }

                moveFile(bootstrap, File(dataDir, "bootstrap.dat.old"))

                logger.info("Imported $n blocks")
            }
        }
    }

    /**
     * @return a [File] or `null` if blockchain is not synchronized
     */
    fun export(): File? {
        val checkpoint = LedgerDB.state().rollingCheckpoint
        if (checkpoint.contentEquals(Genesis.BLOCK_HASH))
            return null

        val file = File(dataDir, "bootstrap.dat.new")
        val stream = file.outputStream().buffered().data()

        var hash = Genesis.BLOCK_HASH
        var index = LedgerDB.getChainIndex(hash)!!
        do {
            hash = index.next
            index = LedgerDB.getChainIndex(hash)!!
            val bytes = BlockDB.blocks.getBytes(hash)!!
            stream.writeInt(bytes.size)
            stream.write(bytes, 0, bytes.size)
        } while (!hash.contentEquals(checkpoint))

        stream.close()

        return file
    }
}
