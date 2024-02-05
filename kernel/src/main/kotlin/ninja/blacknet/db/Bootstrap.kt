/*
 * Copyright (c) 2019-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.github.oshai.kotlinlogging.KotlinLogging
import java.io.EOFException
import java.nio.channels.FileChannel
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardCopyOption.ATOMIC_MOVE
import java.nio.file.StandardOpenOption.CREATE
import java.nio.file.StandardOpenOption.READ
import java.nio.file.StandardOpenOption.TRUNCATE_EXISTING
import java.nio.file.StandardOpenOption.WRITE
import kotlinx.coroutines.runBlocking
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.AlreadyHave
import ninja.blacknet.core.Block
import ninja.blacknet.crypto.Hash
import ninja.blacknet.dataDir
import ninja.blacknet.logging.error
import ninja.blacknet.util.buffered
import ninja.blacknet.util.data
import ninja.blacknet.util.inputStream
import ninja.blacknet.util.outputStream

private val logger = KotlinLogging.logger {}

object Bootstrap {
    /**
     * Imports a bootstrap if the file exists.
     */
    fun import() {
        val bootstrap = dataDir.resolve("bootstrap.dat")
        if (Files.exists(bootstrap)) {
            runBlocking {
                logger.info { "Found bootstrap" }
                var n = 0

                try {
                    FileChannel.open(bootstrap, READ).inputStream().buffered().data().use {
                        while (true) {
                            val size = it.readInt()
                            val bytes = ByteArray(size)
                            it.readFully(bytes)

                            val hash = Block.hash(bytes)
                            val status = BlockDB.processImpl(hash, bytes)
                            if (status == Accepted) {
                                if (++n % 50000 == 0)
                                    logger.info { "Processed $n blocks" }
                                LedgerDB.pruneImpl()
                            } else if (status !is AlreadyHave) {
                                logger.info { "$status block $hash" }
                                break
                            }
                        }
                    }
                } catch (e: EOFException) {
                    // DataInputStream reached end of file
                } catch (e: Throwable) {
                    logger.error(e)
                }

                Files.move(bootstrap, dataDir.resolve("bootstrap.dat.old"), ATOMIC_MOVE)

                logger.info { "Imported $n blocks" }
            }
        }
    }

    /**
     * @return a [Path] or `null` if ledger is not synchronized
     */
    fun export(): Path? {
        val checkpoint = LedgerDB.state().rollingCheckpoint
        if (checkpoint == Genesis.BLOCK_HASH)
            return null

        val file = dataDir.resolve("bootstrap.dat.new")
        FileChannel.open(file, CREATE, TRUNCATE_EXISTING, WRITE).outputStream().buffered().data().use { stream ->
            var hash = Genesis.BLOCK_HASH
            var index = LedgerDB.chainIndexes.getOrThrow(hash.bytes)
            do {
                hash = index.next
                index = LedgerDB.chainIndexes.getOrThrow(hash.bytes)
                val bytes = BlockDB.blocks.getBytesOrThrow(hash.bytes)
                stream.writeInt(bytes.size)
                stream.write(bytes, 0, bytes.size)
            } while (hash != checkpoint)
        }

        return file
    }
}
