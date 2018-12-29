/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.ktor.util.error
import mu.KotlinLogging
import net.freehaven.tor.control.TorControlCommands
import net.freehaven.tor.control.TorControlConnection
import net.freehaven.tor.control.TorControlError
import net.i2p.data.Base32
import ninja.blacknet.Config
import java.io.File

private val logger = KotlinLogging.logger {}

object TorController {
    private var privateKey = "NEW:RSA1024"

    init {
        try {
            privateKey = File("db/privateKey.tor").readText()
        } catch (e: Throwable) {
        }
    }

    fun listen(): Address? {
        val s = java.net.Socket("localhost", Config[Config.torcontrol])
        val tor = TorControlConnection(s)
        tor.launchThread(true)
        tor.authenticate(ByteArray(0))

        val request = HashMap<Int, String?>()
        request[Config[Config.port]] = null

        val response = tor.addOnion(privateKey, request)
        val string = response[TorControlCommands.HS_ADDRESS]!!
        val bytes = Base32.decode(string)!!

        if (bytes.size != Network.TORv2.addrSize)
            throw TorControlError("Unknown KeyType")

        if (privateKey == "NEW:RSA1024")
            savePrivateKey(response[TorControlCommands.HS_PRIVKEY]!!)

        return Address(Network.TORv2, Config[Config.port], bytes)
    }

    private fun savePrivateKey(privKey: String) {
        privateKey = privKey
        logger.info("Saving Tor private key to db")
        try {
            File("db/privateKey.tor").writeText(privateKey)
        } catch (e: Throwable) {
            logger.error(e)
        }
    }
}
