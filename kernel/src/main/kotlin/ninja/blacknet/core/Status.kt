/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

sealed class Status(val message: String) {
    override fun toString() = message
}

object Accepted : Status("Accepted")

class AlreadyHave(message: String) : Status("Already have $message")

class InFuture(message: String) : Status("Too far in future $message")

class Invalid(message: String) : Status(message)

class NotReachableVertex(message: String) : Status("Not reachable vertex $message")

fun notAccepted(message: String, status: Status): Status {
    return when (status) {
        is Invalid -> Invalid("$message ${status.message}")
        is InFuture -> InFuture("$message ${status.message}")
        is NotReachableVertex -> NotReachableVertex("$message ${status.message}")
        is AlreadyHave -> AlreadyHave("$message ${status.message}")
        Accepted -> throw IllegalArgumentException("Already accepted")
    }
}
