/*
 * Copyright (c) 2018-2019 Pavel Vasin
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
import ninja.blacknet.Config
import ninja.blacknet.Config.torcontrol
import ninja.blacknet.coding.Base32
import ninja.blacknet.util.emptyByteArray
import java.io.File

private val logger = KotlinLogging.logger {}

object TorController {
    private var privateKey = "NEW:RSA1024"

    init {
        try {
            val file = File(Config.dataDir, "privateKey.tor")
            val lastModified = file.lastModified()
            if (lastModified != 0L && lastModified < 1549868177000)
                file.renameTo(File(Config.dataDir, "privateKey.$lastModified.tor"))
            privateKey = File(Config.dataDir, "privateKey.tor").readText()
        } catch (e: Throwable) {
        }
    }

    fun listen(): Pair<Thread, Address> {
        //TODO configure host
        val s = java.net.Socket("localhost", Config[torcontrol].toPort().toPort())
        val tor = TorControlConnection(s)
        val thread = tor.launchThread(true)
        //TODO cookie, password
        tor.authenticate(emptyByteArray())

        val request = HashMap<Int, String?>()
        request[Config.netPort.toPort()] = null

        val response = tor.addOnion(privateKey, request)
        val string = response[TorControlCommands.HS_ADDRESS]!!
        val bytes = Base32.decode(string)!!

        if (bytes.size != Network.TORv2.addrSize)
            throw TorControlError("Unknown KeyType")

        if (privateKey == "NEW:RSA1024")
            savePrivateKey(response[TorControlCommands.HS_PRIVKEY]!!)

        val address = Address(Network.TORv2, Config.netPort, bytes)

        return Pair(thread, address)
    }

    private fun savePrivateKey(privKey: String) {
        privateKey = privKey
        logger.info("Saving Tor private key")
        try {
            File(Config.dataDir, "privateKey.tor").writeText(privateKey)
        } catch (e: Throwable) {
            logger.error(e)
        }
    }
}
