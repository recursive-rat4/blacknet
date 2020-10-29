/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.descriptor

public fun elementIndexException(index: Int, size: Int): Throwable = IndexOutOfBoundsException("Descriptor element index $index is out of size $size")

public fun notImplementedError(): Throwable = NotImplementedError("Serial annotations are not implemented")
