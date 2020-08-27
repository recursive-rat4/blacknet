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
import kotlinx.serialization.descriptors.StructureKind
import kotlinx.serialization.encoding.CompositeDecoder

/**
 * [SerialDescriptor] for [StructureKind.MAP].
 */
public class MapSerialDescriptor(
        override val serialName: String,
        private val keyDescriptor: SerialDescriptor,
        private val valueDescriptor: SerialDescriptor
) : SerialDescriptor {
    override val kind: SerialKind = StructureKind.MAP
    override val isNullable: Boolean = false
    override val elementsCount: Int = 2
    override val annotations: List<Annotation> get() = throw notImplementedError()

    override fun getElementName(index: Int): String {
        return when (index) {
            0 -> "key"
            1 -> "value"
            else -> throw elementIndexException(index, 2)
        }
    }

    override fun getElementIndex(name: String): Int {
        return when (name) {
            "key" -> 0
            "value" -> 1
            else -> CompositeDecoder.UNKNOWN_NAME
        }
    }

    override fun getElementAnnotations(index: Int): List<Annotation> {
        @Suppress("UNREACHABLE_CODE")
        return when (index) {
            0, 1 -> throw notImplementedError()
            else -> throw elementIndexException(index, 2)
        }
    }

    override fun getElementDescriptor(index: Int): SerialDescriptor {
        return when (index) {
            0 -> keyDescriptor
            1 -> valueDescriptor
            else -> throw elementIndexException(index, 2)
        }
    }

    override fun isElementOptional(index: Int): Boolean {
        return when (index) {
            0, 1 -> false
            else -> throw elementIndexException(index, 2)
        }
    }
}
