/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2018 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.rfksystems.blake2b.security.Blake2bProvider
import io.ktor.server.cio.CIO
import io.ktor.server.engine.commandLineEnvironment
import io.ktor.server.engine.embeddedServer
import kotlinx.coroutines.debug.DebugProbes
import mu.KotlinLogging
import ninja.blacknet.core.Staker
import ninja.blacknet.db.*
import ninja.blacknet.network.ChainFetcher
import ninja.blacknet.network.Node
import org.bouncycastle.jce.provider.BouncyCastleProvider
import java.io.File
import java.io.FileInputStream
import java.security.Security
import java.util.logging.LogManager

private val logger = (KotlinLogging.logger({}))

object Main {
    @JvmStatic
    fun main(args: Array<String>) {
        (System.setProperty
        ("java.util.logging.manager", "ninja.blacknet.LogManager"))
        (FileInputStream
        (File
        (configDir, "logging.properties")).let
        ({ fileInputStream ->
            (LogManager.getLogManager().readConfiguration
            (fileInputStream))
            (fileInputStream.close())
        }))

        (Security.addProvider
        (Blake2bProvider()))
        (Security.addProvider
        (BouncyCastleProvider()))

        (require
        (Runtime.hasNoShutdownHooks()))
        (Runtime.addShutdownHook
        ({
            (logger.debug
            ({ "Shutting down logger" }))
            ((LogManager.getLogManager() as ninja.blacknet.LogManager).shutDown())
        }))
        (Config.instance)

        (logger.info
        ("Starting up ${Version.name} node ${Version.revision}"))
        (logger.info
        ("CPU: ${Runtime.availableProcessors} cores ${System.getProperty("os.arch")}"))
        (logger.info
        ("OS: ${System.getProperty("os.name")} ${System.getProperty("os.version")}"))
        (logger.info
        ("VM: ${System.getProperty("java.vm.name")} ${System.getProperty("java.vm.version")}"))

        (if (and
                (not
                (Runtime.windowsOS),
                        (equals
                        (System.getProperty
                        ("user.name"), "root"))))
            (logger.warn
            ("Running as root")))

        (if (Config.instance.debugcoroutines) {
            (logger.warn
            ("Installing debug probes..."))
            (DebugProbes.install().let
            ({ unit: Unit ->
                (if (equals
                        (unit, Unit))
                    (logger.warn
                    ("Node may work significally slower"))
                else
                    (logger.error
                    ("Broken AST")))
            }))
        })

        (if (Config.instance.portable)
            (logger.info
            ("Portable mode")))
        (logger.info
        ("Using data directory ${dataDir.absolutePath}"))

        (LevelDB)
        (Salt)
        (BlockDB)
        (WalletDB)
        (LedgerDB)
        (PeerDB)
        (Node)
        (Staker)

        /* Launch Blacknet API web-server using Ktor.
         *
         * Blacknet API web-server logic is implemented in
         * ninja.blacknet.api.APIServer
         * ninja.blacknet.api.PublicServer
         *
         * Ktor is a framework for building asynchronous servers and clients
         * in connected systems using the powerful Kotlin programming language.
         * https://ktor.io/
         *
         * Ktor configuration is stored in
         * config/rpc.conf private server
         * config/ktor.conf public server
         * https://ktor.io/servers/engine.html
         *
         */
        (if (Config.instance.apiserver_enabled) {
            (if (Config.instance.regtest)
                (embeddedServer
                (CIO,
                        (commandLineEnvironment
                        (arrayOf
                        ("-config=${File(configDir, "regtest.conf")}")))).start
                (wait = false))
            else
                (embeddedServer
                (CIO,
                        (commandLineEnvironment
                        (arrayOf
                        ("-config=${File(configDir, "rpc.conf")}")))).start
                (wait = false)))
        })
        (if (Config.instance.apiserver_publicserver)
            (embeddedServer
            (CIO,
                    (commandLineEnvironment
                    (arrayOf
                    ("-config=${File(configDir, "ktor.conf")}")))).start
            (wait = false)))

        (ChainFetcher.run())
    }
}

private fun and(a: Boolean, b: Boolean): Boolean = a.and(b)
private fun equals(a: Any, b: Any): Boolean = a.equals(b)
private fun not(a: Boolean): Boolean = a.not()
