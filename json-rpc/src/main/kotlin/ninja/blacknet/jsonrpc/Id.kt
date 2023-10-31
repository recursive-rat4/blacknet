/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("INLINE_CLASS_DEPRECATED")

package ninja.blacknet.jsonrpc

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.booleanOrNull

/**
 * A type for the `id` field in a [Request] or [Response].
 */
@Serializable
internal inline class Id private constructor(
    private val value: JsonPrimitive
) {
    /**
     * Use a string as the id.
     */
    constructor(string: String) : this(JsonPrimitive(string))

    /**
     * Use a number as the id.
     */
    constructor(number: Int) : this(JsonPrimitive(number))

    /**
     * Use `null` as the id.
     */
    constructor() : this(JsonNull)

    init {
        // https://github.com/Kotlin/kotlinx.serialization/issues/1298
        require(value.booleanOrNull == null) { "$value is not string, number or null" }
    }
}
