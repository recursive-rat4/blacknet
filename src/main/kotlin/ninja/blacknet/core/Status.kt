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

class AlreadyHave(private val message: String) : Status() {
    override fun toString() = "Already have $message"
}

class InFuture(private val message: String) : Status() {
    override fun toString() = "Too far in future $message"
}

class Invalid(private val message: String) : Status() {
    override fun toString() = message
}

class NotOnThisChain(private val message: String) : Status() {
    override fun toString() = "Not on this chain $message"
}

fun notAccepted(message: String, status: Status): Status {
    return when (status) {
        is Invalid -> Invalid("$message $status")
        is InFuture -> InFuture("$message $status")
        is NotOnThisChain -> NotOnThisChain("$message $status")
        is AlreadyHave -> AlreadyHave("$message $status")
        Accepted -> throw IllegalArgumentException("Already accepted")
    }
}
