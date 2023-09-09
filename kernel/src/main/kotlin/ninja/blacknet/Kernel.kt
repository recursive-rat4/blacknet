/*
 * Copyright (c) 2018-2020 Pavel Vasin
 * Copyright (c) 2018 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.rfksystems.blake2b.security.Blake2bProvider
import io.github.oshai.kotlinlogging.KotlinLogging
import io.ktor.server.cio.CIO
import io.ktor.server.engine.commandLineEnvironment
import io.ktor.server.engine.embeddedServer
import java.lang.Thread.UncaughtExceptionHandler
import java.nio.file.Files
import java.security.Security
import java.util.logging.LogManager
import kotlinx.coroutines.debug.DebugProbes
import ninja.blacknet.core.Staker
import ninja.blacknet.core.TxPool
import ninja.blacknet.db.*
import ninja.blacknet.network.ChainFetcher
import ninja.blacknet.network.Node
import org.bouncycastle.jce.provider.BouncyCastleProvider

private val logger = KotlinLogging.logger {}

object Kernel {
    /**
     * Initialize the kernel library: logging, database, network and other essentials.
     *
     * @param args command line arguments.
     * @param uncaughtExceptionHandler is set as a default handler when logging is ready.
     * @throws Throwable if something went wrong.
     */
    fun init(
        @Suppress("UNUSED_PARAMETER")
        args: Array<String>,
        uncaughtExceptionHandler: UncaughtExceptionHandler,
    ) {
        populateConfigDir().let { createdFiles ->
            if (System.getProperty("ninja.blacknet.createConfigAndExit") != null) {
                System.exit(
                    if (createdFiles != 0) {
                        println("Created $createdFiles files in $configDir")
                        0
                    } else {
                        println("Configuration already exists or cannot be created in $configDir")
                        1
                    }
                )
            }
        }

        System.setProperty("java.util.logging.manager", "ninja.blacknet.logging.LogManager")
        Files.newInputStream(configDir.resolve("logging.properties")).let { inputStream ->
            LogManager.getLogManager().readConfiguration(inputStream)
            inputStream.close()
        }
        ShutdownHooks.add {
            logger.info { "Shutting down logger" }
            (LogManager.getLogManager() as ninja.blacknet.logging.LogManager).shutDown()
        }

        // exceptions from coroutines should end up here too
        Thread.setDefaultUncaughtExceptionHandler(uncaughtExceptionHandler)

        logger.info { "Starting up ${Version.name} node ${Version.revision}" }
        logger.info { "CPU: ${Runtime.availableProcessors} cores ${System.getProperty("os.arch")}" }
        logger.info { "OS: ${System.getProperty("os.name")} ${System.getProperty("os.version")}" }
        logger.info { "VM: ${System.getProperty("java.vm.name")} ${System.getProperty("java.vm.version")}" }
        logger.info { "Using config directory ${configDir.toAbsolutePath()}" }
        logger.info { "Using data directory ${dataDir.toAbsolutePath()}" }
        logger.info { "Using state directory ${stateDir.toAbsolutePath()}" }

        testAtomicFileMove(dataDir)

        Security.addProvider(Blake2bProvider())
        Security.addProvider(BouncyCastleProvider())

        Config.instance

        //WINDOWS system user, admin rights, and whatever
        //XXX root may be renamed
        if (!Runtime.windowsOS && System.getProperty("user.name") == "root")
            logger.warn { "Running as root" }

        if (Config.instance.debugcoroutines) {
            logger.warn { "Installing debug probes..." }
            DebugProbes.ignoreCoroutinesWithEmptyContext = false
            DebugProbes.install()
            logger.warn { "Node may work significally slower" }
        }

        LevelDB
        Salt
        BlockDB
        WalletDB
        LedgerDB
        PeerDB
        TxPool
        Node
        ChainFetcher
        Staker

        /* Launch Blacknet RPC API server using Ktor.
         *
         * Blacknet RPC API server logic is implemented in
         * ninja.blacknet.rpc.RPCServer
         *
         * Ktor is a framework for building asynchronous servers and clients
         * in connected systems using the powerful Kotlin programming language.
         * https://ktor.io/
         *
         * Ktor configuration is stored in
         * rpc.conf for main network
         * rpcregtest.conf for regression testing mode
         * https://ktor.io/servers/engine.html
         *
         */
        if (Config.instance.rpcserver) {
            embeddedServer(
                CIO,
                commandLineEnvironment(arrayOf("-config=${configDir.resolve(
                    if (regtest)
                        "rpcregtest.conf"
                    else
                        "rpc.conf"
                )}"))
            ).start(wait = false)
        }
    }
}
