/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import java.io.DataInputStream
import java.io.InputStream

public fun DataInputStream.readUByte(): UByte = readByte().toUByte()

public fun DataInputStream.readUShort(): UShort = readShort().toUShort()

public fun DataInputStream.readUInt(): UInt = readInt().toUInt()

public fun DataInputStream.readULong(): ULong = readLong().toULong()

public fun InputStream.data(): DataInputStream = DataInputStream(this)
