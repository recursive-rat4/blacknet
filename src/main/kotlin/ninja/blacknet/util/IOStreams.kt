/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import java.io.*

fun InputStream.buffered(bufferSize: Int = DEFAULT_BUFFER_SIZE): BufferedInputStream = BufferedInputStream(this, bufferSize)
fun InputStream.data(): DataInputStream = DataInputStream(this)

fun OutputStream.buffered(bufferSize: Int = DEFAULT_BUFFER_SIZE): BufferedOutputStream = BufferedOutputStream(this, bufferSize)
fun OutputStream.data(): DataOutputStream = DataOutputStream(this)
