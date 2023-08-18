/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.requests

import io.ktor.server.application.Application
import io.ktor.server.application.BaseApplicationPlugin
import io.ktor.util.AttributeKey

object Requests : BaseApplicationPlugin<Application, Unit, Unit> {
    override val key: AttributeKey<Unit> = AttributeKey("請求功能")

    override fun install(pipeline: Application, configure: Unit.() -> Unit): Unit {
        return Unit
    }
}
