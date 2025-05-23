/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2018 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.rfksystems.blake2b.security.Blake2bProvider
import com.sun.security.auth.module.NTSystem
import com.sun.security.auth.module.UnixSystem
import io.github.oshai.kotlinlogging.KotlinLogging
import io.ktor.server.cio.CIO
import io.ktor.server.engine.CommandLineConfig
import io.ktor.server.engine.EmbeddedServer
import java.lang.Thread.UncaughtExceptionHandler
import java.nio.file.Files
import java.security.Security
import java.util.logging.LogManager
import kotlinx.coroutines.debug.DebugProbes
import ninja.blacknet.core.Staker
import ninja.blacknet.core.TxPool
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.CoinDB
import ninja.blacknet.db.Convertor
import ninja.blacknet.db.LevelDB
import ninja.blacknet.db.PeerDB
import ninja.blacknet.db.WalletDB
import ninja.blacknet.network.BlockFetcher
import ninja.blacknet.network.Node
import ninja.blacknet.rpc.RPCServer
import ninja.blacknet.serialization.config.ConfigFormat
import ninja.blacknet.serialization.textModule
import org.bouncycastle.jce.provider.BouncyCastleProvider

private val logger = KotlinLogging.logger {}

object Kernel {
    internal val ktorEngine = CIO

    private var config: Config? = null
    fun config() = config ?: notInitialized()
    private var blockDB: BlockDB? = null
    fun blockDB() = blockDB ?: notInitialized()
    private var txPool: TxPool? = null
    fun txPool() = txPool ?: notInitialized()

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

        logger.info { "Starting up ${Version.name} node ${Version.version}" }
        logger.info { "CPU: ${Runtime.availableProcessors} cores ${System.getProperty("os.arch")}" }
        logger.info { "OS: ${System.getProperty("os.name")} ${System.getProperty("os.version")}" }
        logger.info { "VM: ${System.getProperty("java.vm.name")} ${System.getProperty("java.vm.version")}" }
        logger.info { "Using config directory ${configDir.toAbsolutePath()}" }
        logger.info { "Using data directory ${dataDir.toAbsolutePath()}" }
        logger.info { "Using state directory ${stateDir.toAbsolutePath()}" }

        testAtomicFileMove(dataDir)

        Security.addProvider(Blake2bProvider())
        Security.addProvider(BouncyCastleProvider())

        config = ConfigFormat(serializersModule = textModule).decodeFromFile(Config.serializer(), configDir.resolve("blacknet.conf"))

        if (Runtime.windowsOS && NTSystem().userSID == "S-1-5-18")
            logger.warn { "Running as SYSTEM" }
        else if (UnixSystem().uid == 0L)
            logger.warn { "Running as root" }

        if (config().debugcoroutines) {
            logger.warn { "Installing debug probes..." }
            DebugProbes.enableCreationStackTraces = true
            DebugProbes.ignoreCoroutinesWithEmptyContext = false
            DebugProbes.install()
            logger.warn { "Node may work significally slower" }
        }

        LevelDB
        Convertor
        blockDB = BlockDB(LevelDB)
        WalletDB
        CoinDB
        PeerDB
        txPool = TxPool(config(), blockDB())
        Node
        BlockFetcher
        Staker

        WalletDB.launch()

        /* Launch Blacknet RPC API server using Ktor.
         *
         * Blacknet RPC API server logic is implemented in
         * ninja.blacknet.rpc.RPCServer
         *
         * Ktor is a framework for building asynchronous servers and clients
         * in connected systems using the powerful Kotlin programming language.
         * https://ktor.io/
         *
         * Ktor configuration is stored in rpc.conf
         * https://ktor.io/servers/engine.html
         *
         */
        if (config().rpcserver) {
            RPCServer
            val ktorConfig = CommandLineConfig(arrayOf(
                "-config=${configDir.resolve("rpc.conf")}",
            ))
            val ktorServer = EmbeddedServer(ktorConfig.rootConfig, ktorEngine) {
                takeFrom(ktorConfig.engineConfig)
            }
            ktorServer.start(wait = false)
        }
    }

    private fun notInitialized(): Nothing = throw IllegalStateException("Kernel not initialized")
}
