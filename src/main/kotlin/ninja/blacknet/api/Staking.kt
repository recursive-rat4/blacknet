/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.request.receiveParameters
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import io.ktor.routing.post
import ninja.blacknet.core.Staker
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Mnemonic

fun Route.staking() {
    post("/api/v2/startstaking") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

        call.respond(Staker.startStaking(privateKey).toString())
    }

    post("/api/v2/stopstaking") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")

        call.respond(Staker.stopStaking(privateKey).toString())
    }

    post("/api/v2/isstaking") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")

        call.respond(Staker.isStaking(privateKey).toString())
    }

    get("/api/v2/staking/{address?}") {
        val publicKey = call.parameters["address"]?.let { Address.decode(it) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address") }

        call.respondJson(StakingInfo.serializer(), Staker.info(publicKey))
    }
}
