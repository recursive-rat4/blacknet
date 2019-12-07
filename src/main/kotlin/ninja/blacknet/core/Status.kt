/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

sealed class Status

object Accepted : Status() {
    override fun toString() = "Accepted"
}

object AlreadyHave : Status() {
    override fun toString() = "Already have"
}

object InFuture : Status() {
    override fun toString() = "Too far in future"
}

class Invalid(private val message: String) : Status() {
    override fun toString() = message
}

object NotOnThisChain : Status() {
    override fun toString() = "Not on this chain"
}

fun notAccepted(message: String, status: Status): Status {
    return when (status) {
        is Invalid -> Invalid("$message $status")
        Accepted -> throw IllegalArgumentException(status.toString())
        else -> status
    }
}
