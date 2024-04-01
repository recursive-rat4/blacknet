/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import java.io.DataOutputStream
import java.io.OutputStream

public fun DataOutputStream.writeByte(value: Byte): Unit = writeByte(value.toInt())

public fun DataOutputStream.writeShort(value: Short): Unit = writeShort(value.toInt())

public fun DataOutputStream.writeUByte(value: UByte): Unit = writeByte(value.toInt())

public fun DataOutputStream.writeUShort(value: UShort): Unit = writeShort(value.toInt())

public fun DataOutputStream.writeUInt(value: UInt): Unit = writeInt(value.toInt())

public fun DataOutputStream.writeULong(value: ULong): Unit = writeLong(value.toLong())

public fun OutputStream.data(): DataOutputStream = DataOutputStream(this)
