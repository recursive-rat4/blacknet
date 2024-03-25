/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2018 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.github.oshai.kotlinlogging.KotlinLogging
import java.nio.channels.FileChannel
import java.nio.file.NoSuchFileException
import java.nio.file.StandardOpenOption.READ
import java.util.HashMap.newHashMap
import java.util.concurrent.ConcurrentHashMap
import kotlin.math.exp
import kotlin.math.max
import kotlin.math.min
import kotlin.math.pow
import kotlin.random.Random
import kotlinx.atomicfu.atomic
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.Serializable
import kotlinx.serialization.Transient
import kotlinx.serialization.builtins.*
import ninja.blacknet.Config
import ninja.blacknet.Kernel
import ninja.blacknet.Mode.*
import ninja.blacknet.Runtime
import ninja.blacknet.ShutdownHooks
import ninja.blacknet.contract.BAppId
import ninja.blacknet.dataDir
import ninja.blacknet.io.buffered
import ninja.blacknet.io.data
import ninja.blacknet.io.inputStream
import ninja.blacknet.io.replaceFile
import ninja.blacknet.logging.error
import ninja.blacknet.mode
import ninja.blacknet.network.Address
import ninja.blacknet.network.AddressV1
import ninja.blacknet.network.Network
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.VarIntSerializer
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.time.currentTimeSeconds
import ninja.blacknet.util.Resources
import ninja.blacknet.util.rotate

private val logger = KotlinLogging.logger {}

object PeerDB {
    const val MAX_SIZE = 8192
    private const val VERSION = 4
    private const val FILENAME = "peers.dat"
    private val LOCALHOST = Network.LOOPBACK(Kernel.config().port)
    private val peers = ConcurrentHashMap<Address, Entry>(MAX_SIZE)
    private val STATE_KEY = DBKey(0x80.toByte(), 0)
    private val VERSION_KEY = DBKey(0x81.toByte(), 0)
    private val writeToDiskMutex = Mutex()

    init {
        val decodedMap = loadFromFile() ?: extractFromLevelDB()

        if (decodedMap != null) {
            logger.info { "Loaded ${decodedMap.size} peer addresses" }
            peers.putAll(decodedMap)
        }

        if (peers.size < 100) {
            val added = add(listBuiltinPeers(), LOCALHOST)
            if (added > 0) {
                logger.info { "Added $added built-in peer addresses to db" }
            }
        }

        Runtime.rotate(::prober)

        ShutdownHooks.add {
            if (writeToDiskMutex.tryLock()) {
                logger.info { "Saving PeerDB" }
                saveToFile()
            } else runBlocking {
                logger.info { "Waiting PeerDB saver" }
                writeToDiskMutex.lock()
            }
            // hodl mutex so others don't acquire it
        }
    }

    private fun loadFromFile(): Map<Address, Entry>? {
        try {
            FileChannel.open(dataDir.resolve(FILENAME), READ).inputStream().buffered().data().use { stream ->
                val version = stream.readInt()
                if (version == VERSION) {
                    return binaryFormat.decodeFromStream(MapSerializer(Address.serializer(), Entry.serializer()), stream)
                } else {
                    throw Error("Unknown database version $version")
                }
            }
        } catch (e: NoSuchFileException) {
            // first run or unlinked file
        }
        return null
    }

    private fun extractFromLevelDB(): Map<Address, Entry>? {
        val stateBytes = LevelDB.get(STATE_KEY)

        if (stateBytes == null)
            return null

        val versionBytes = LevelDB.get(VERSION_KEY)

        val version = if (versionBytes != null) {
            binaryFormat.decodeFromByteArray(VarIntSerializer, versionBytes)
        } else {
            1
        }

        val decodedMap = if (version == VERSION) {
            binaryFormat.decodeFromByteArray(MapSerializer(Address.serializer(), Entry.serializer()), stateBytes)
        } else if (version in 1 until VERSION) {
            val updatedMap = run {
                logger.info { "Upgrading PeerDB..." }
                val result = newHashMap<Address, Entry>(MAX_SIZE)
                try {
                    if (version == 3) {
                        val stateV3 = binaryFormat.decodeFromByteArray(MapSerializer(Address.serializer(), EntryV3.serializer()), stateBytes)
                        stateV3.forEach { (address, entryV3) ->
                            result.put(address, Entry(entryV3))
                        }
                    } else if (version == 2) {
                        val stateV2 = binaryFormat.decodeFromByteArray(MapSerializer(AddressV1.serializer(), EntryV2.serializer()), stateBytes)
                        stateV2.forEach { (addressV1, entryV2) ->
                            result.put(Address(addressV1), Entry(entryV2))
                        }
                    } else if (version == 1) {
                        val stateV1 = binaryFormat.decodeFromByteArray(MapSerializer(AddressV1.serializer(), EntryV1.serializer()), stateBytes)
                        stateV1.forEach { (addressV1, entryV1) ->
                            result.put(Address(addressV1), Entry(entryV1))
                        }
                    }
                } catch (e: Throwable) {
                    logger.error(e)
                }
                result
            }

            updatedMap
        } else {
            throw Error("Unknown database version $version")
        }

        val batch = LevelDB.createWriteBatch()
        batch.delete(VERSION_KEY)
        batch.delete(STATE_KEY)
        batch.write()

        return decodedMap
    }

    private fun listBuiltinPeers(): List<Address> = when (mode) {
        MainNet -> {
            Resources.lines(PeerDB::class.java, "peers.txt", Charsets.UTF_8)
                .map {
                    Network.parse(it, mode.defaultP2PPort) ?: throw RuntimeException("Failed to parse $it")
                }
        }
        TestNet -> {
            throw NotImplementedError("$mode peers.txt is missing")
        }
        SigNet -> {
            throw NotImplementedError("$mode peers.txt is missing")
        }
        RegTest -> {
            emptyList()
        }
    }

    fun size(): Int {
        return peers.size
    }

    fun isEmpty(): Boolean {
        return peers.isEmpty()
    }

    fun connected(address: Address, time: Long, userAgent: String, prober: Boolean) {
        if (address.isLocal() || address.isPrivate()) return
        val entry = peers.get(address)
        if (entry != null)
            entry.connected(time, userAgent, prober)
        else
            peers.put(address, Entry(time, userAgent))
    }

    fun tryContact(address: Address): Boolean {
        if (address.isLocal() || address.isPrivate()) return false
        // ignore max size
        val entry = peers.computeIfAbsent(address) { Entry(LOCALHOST) }
        return entry.inContact.compareAndSet(false, true)
    }

    fun contacted(address: Address) {
        if (address.isLocal() || address.isPrivate()) return
        // ignore max size
        val entry = peers.computeIfAbsent(address) { Entry(LOCALHOST) }
        if (entry.inContact.compareAndSet(false, true))
            return
        else
            logger.error { "Inconsistent PeerDB.Entry of ${address.debugName()}" }
    }

    fun discontacted(address: Address) {
        if (address.isLocal() || address.isPrivate()) return
        val entry = peers.get(address)
        if (entry != null) {
            if (entry.inContact.compareAndSet(true, false))
                return
            else
                logger.error { "Inconsistent PeerDB.Entry of ${address.debugName()}" }
        } else {
            logger.error { "Not found PeerDB.Entry of ${address.debugName()}" }
        }
    }

    fun failed(address: Address, time: Long) {
        if (Node.isOffline()) return
        peers.get(address)?.failed(time)
    }

    fun subnetworksAnnounce(address: Address, announce: List<BAppId>): Unit {
        peers.get(address)?.stat?.subnetworks?.let { subnetworks ->
            announce.forEach { id ->
                if (BAppDB.isInteresting(id)) {
                    subnetworks.put(id, Unit)
                }
            }
        }
    }

    fun getSubnetwork(id: BAppId): List<Address> {
        val result = ArrayList<Address>(peers.size)
        peers.forEach { (address, entry) -> if (entry.stat?.subnetworks?.contains(id) == true) result.add(address) }
        return result
    }

    fun getAll(): List<Pair<Address, Entry>> {
        return peers.toList()
    }

    fun getSeed(): List<Address> {
        val result = ArrayList<Address>(peers.size)
        peers.forEach { (address, entry) -> address.port == mode.defaultP2PPort && entry.isReliable() }
        return result
    }

    fun getCandidate(predicate: (Address, Entry) -> Boolean): Address? {
        val candidates = ArrayList<Triple<Address, Entry, Float>>(peers.size)
        val currTime = currentTimeSeconds()
        peers.forEach { (address, entry) ->
            if (predicate(address, entry))
                candidates.add(Triple(address, entry, entry.chance(currTime)))
        }
        while (candidates.isNotEmpty()) {
            val random = Random.nextInt(candidates.size)
            val (address, entry, chance) = candidates[random]
            if (chance > Random.nextFloat()) {
                if (entry.inContact.compareAndSet(false, true))
                    return address
                else
                    candidates.removeAt(random)
            }
        }
        return null
    }

    fun getRandom(n: Int): ArrayList<Address> {
        val candidates = ArrayList<Address>(peers.size)
        peers.forEach { (address, _) -> candidates.add(address) }
        candidates.shuffle()
        val x = min(candidates.size, n)
        val result = ArrayList<Address>(x)
        for (i in 0 until x)
            result.add(candidates[i])
        return result
    }

    fun add(newPeers: List<Address>, from: Address, force: Boolean = false): Int {
        var added = 0
        var i = 0
        val newPeersSize = newPeers.size
        val nToAdd = if (!force) {
            val freeSlots = max(MAX_SIZE - peers.size, 0)
            min(newPeersSize, freeSlots)
        } else {
            newPeersSize
        }
        while (i < newPeersSize && added < nToAdd) {
            if (addImpl(newPeers[i], from))
                added += 1
            i += 1
        }
        return added
    }

    private fun addImpl(peer: Address, from: Address): Boolean {
        if (peer.isLocal() || peer.isPrivate())
            return false
        if (peer.network == Network.TORv2) // obsolete
            return false
        if (peers.containsKey(peer))
            return false
        peers.put(peer, Entry(from))
        return true
    }

    fun contains(peer: Address): Boolean {
        return peers.containsKey(peer)
    }

    private suspend fun prober() {
        delay(1 * 60 * 60 * 1000L)

        // Await while node is offline
        if (Node.isOffline())
            return

        var removed = 0
        val currTime = currentTimeSeconds()
        peers.forEach { (address, entry) ->
            if (entry.isOld(currTime)) {
                peers.remove(address)
                ++removed
            }
        }
        if (removed != 0) {
            writeToDiskMutex.withLock {
                saveToFile()
            }
            logger.debug { "Probed $removed entries" }
        }
    }

    private fun saveToFile() {
        replaceFile(dataDir, FILENAME) {
            writeInt(VERSION)
            binaryFormat.encodeToStream(MapSerializer(Address.serializer(), Entry.serializer()), peers, this)
        }
    }

    @Serializable
    class NetworkStat(
            var lastConnected: Long,
            var userAgent: String,
            val stat2H: UptimeStat,
            val stat8H: UptimeStat,
            val stat1D: UptimeStat,
            val stat1W: UptimeStat,
            val stat1M: UptimeStat,
            val subnetworks: HashMap<BAppId, Unit>,
    ) {
        constructor(lastConnected: Long, userAgent: String) : this(
                lastConnected,
                userAgent,
                UptimeStat(),
                UptimeStat(),
                UptimeStat(),
                UptimeStat(),
                UptimeStat(),
                newHashMap(0),
        )
        internal constructor(stat: NetworkStatV1) : this(stat.lastConnected, stat.userAgent, stat.stat2H, stat.stat8H, stat.stat1D, stat.stat1W, stat.stat1M, newHashMap(0))
    }

    @Serializable
    class Entry(
            val from: Address,
            var attempts: Int,
            var lastTry: Long,
            var stat: NetworkStat?
    ) {
        internal constructor(entry: EntryV1) : this(Address(entry.from), entry.attempts, entry.lastTry, null)
        internal constructor(entry: EntryV2) : this(Address(entry.from), entry.attempts, entry.lastTry, entry.stat?.let { NetworkStat(it) })
        internal constructor(entry: EntryV3) : this(entry.from, entry.attempts, entry.lastTry, entry.stat?.let { NetworkStat(it) })

        constructor(from: Address) : this(from, 0, 0, null)
        constructor(time: Long, userAgent: String) : this(LOCALHOST, 0, 0, NetworkStat(time, userAgent))

        @Transient
        internal val inContact = atomic(false)

        fun failed(time: Long) {
            stat?.let { updateUptimeStat(it, false, time) }
            attempts += 1
            lastTry = time
        }

        fun connected(time: Long, userAgent: String, prober: Boolean) {
            val stat = stat
            if (stat != null) {
                stat.lastConnected = time
                stat.userAgent = userAgent
                if (!prober)
                    stat.subnetworks.clear()
                updateUptimeStat(stat, true, time)
            } else {
                @Suppress("NAME_SHADOWING")
                val stat = NetworkStat(time, userAgent)
                updateUptimeStat(stat, true, time)
                this.stat = stat
            }
            attempts = 0
            lastTry = time
        }

        fun chance(time: Long): Float {
            val age = time - lastTry
            val chance = 0.66f.pow(min(attempts, 8))
            return if (age > 15 * 60)
                chance
            else
                chance * 0.01f
        }

        fun isNew(): Boolean {
            return stat == null && attempts == 0
        }

        fun isOld(currTime: Long): Boolean {
            val lastConnected = stat?.lastConnected ?: 0L

            if (lastConnected == 0L && attempts > 15)
                return true
            if (lastConnected != 0L && currTime - lastConnected > 15 * 24 * 60 * 60)
                return true

            return false
        }

        fun isReliable(): Boolean {
            val stat = stat ?: return false

            if (stat.stat2H.reliability > 0.85f && stat.stat2H.count > 2f) return true
            if (stat.stat8H.reliability > 0.70f && stat.stat8H.count > 4f) return true
            if (stat.stat1D.reliability > 0.55f && stat.stat1D.count > 8f) return true
            if (stat.stat1W.reliability > 0.45f && stat.stat1W.count > 16f) return true
            if (stat.stat1M.reliability > 0.35f && stat.stat1M.count > 32f) return true

            return false
        }

        private fun updateUptimeStat(stat: NetworkStat, good: Boolean, time: Long) {
            val age = time - lastTry
            stat.stat2H.update(good, age, 3600.0f * 2)
            stat.stat8H.update(good, age, 3600.0f * 8)
            stat.stat1D.update(good, age, 3600.0f * 24)
            stat.stat1W.update(good, age, 3600.0f * 24 * 7)
            stat.stat1M.update(good, age, 3600.0f * 24 * 30)
        }

        @Suppress("unused")
        @Serializable
        class Info(
                val address: String,
                val from: String,
                val attempts: Int,
                var lastTry: Long,
                val stat: NetworkStat?
        ) {
            constructor(entry: Entry, address: Address) : this(
                    address.toString(),
                    entry.from.toString(),
                    entry.attempts,
                    entry.lastTry,
                    entry.stat
            )
        }
    }

    @Serializable
    class UptimeStat(var weight: Float, var count: Float, var reliability: Float) {
        constructor() : this(0.0f, 0.0f, 0.0f)

        fun update(good: Boolean, age: Long, tau: Float) {
            val f: Float = exp(-age / tau)
            reliability = reliability * f + if (good) 1.0f - f else 0.0f
            count = count * f + 1.0f
            weight = weight * f + (1.0f - f)
        }
    }

    @Serializable
    internal class EntryV1(val from: AddressV1, val attempts: Int, val lastTry: Long, val lastConnected: Long)

    @Serializable
    internal class EntryV2(val from: AddressV1, val attempts: Int, val lastTry: Long, val stat: NetworkStatV1?)

    @Serializable
    internal class EntryV3(val from: Address, val attempts: Int, val lastTry: Long, val stat: NetworkStatV1?)

    @Serializable
    internal class NetworkStatV1(var lastConnected: Long, var userAgent: String, val stat2H: UptimeStat, val stat8H: UptimeStat, val stat1D: UptimeStat, val stat1W: UptimeStat, val stat1M: UptimeStat)
}
