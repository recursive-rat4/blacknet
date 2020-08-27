/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.descriptor

import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.SerialKind
import kotlinx.serialization.encoding.CompositeDecoder

/**
 * [SerialDescriptor] for [SerialKind.CONTEXTUAL].
 */
public class ContextualSerialDescriptor(
        override val serialName: String
) : SerialDescriptor {
    override val kind: SerialKind = SerialKind.CONTEXTUAL
    override val isNullable: Boolean = false
    override val elementsCount: Int = 0
    override val annotations: List<Annotation> get() = throw notImplementedError()

    override fun getElementName(index: Int): String {
        throw elementIndexException(index, 0)
    }

    override fun getElementIndex(name: String): Int {
        return CompositeDecoder.UNKNOWN_NAME
    }

    override fun getElementAnnotations(index: Int): List<Annotation> {
        throw elementIndexException(index, 0)
    }

    override fun getElementDescriptor(index: Int): SerialDescriptor {
        throw elementIndexException(index, 0)
    }

    override fun isElementOptional(index: Int): Boolean {
        throw elementIndexException(index, 0)
    }
}
