/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import java.io.BufferedInputStream
import java.io.BufferedOutputStream
import java.io.DataInputStream
import java.io.DataOutputStream
import java.io.InputStream
import java.io.OutputStream
import java.nio.channels.Channels
import java.nio.channels.FileChannel
import java.nio.channels.ReadableByteChannel
import java.nio.channels.WritableByteChannel
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardCopyOption.ATOMIC_MOVE
import java.nio.file.StandardOpenOption.WRITE

fun InputStream.buffered(bufferSize: Int = DEFAULT_BUFFER_SIZE): BufferedInputStream = BufferedInputStream(this, bufferSize)
fun InputStream.data(): DataInputStream = DataInputStream(this)

fun OutputStream.buffered(bufferSize: Int = DEFAULT_BUFFER_SIZE): BufferedOutputStream = BufferedOutputStream(this, bufferSize)
fun OutputStream.data(): DataOutputStream = DataOutputStream(this)

fun ReadableByteChannel.inputStream(): InputStream = Channels.newInputStream(this)
fun WritableByteChannel.outputStream(): OutputStream = Channels.newOutputStream(this)

inline fun replaceFile(dir: Path, name: String, action: DataOutputStream.() -> Unit) {
    val tmpFile = Files.createTempFile(dir, name + '-', null)
    val channel = FileChannel.open(tmpFile, WRITE)
    channel.outputStream().buffered().data().use {
        it.action()
        it.flush()
        channel.force(false)
    }
    Files.move(tmpFile, dir.resolve(name), ATOMIC_MOVE)
}
