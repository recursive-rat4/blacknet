/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2018 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.Serializable
import kotlinx.serialization.internal.HashMapSerializer
import mu.KotlinLogging
import ninja.blacknet.network.Address
import ninja.blacknet.network.Node
import ninja.blacknet.network.Runtime
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.util.SynchronizedHashMap
import ninja.blacknet.util.delay
import kotlin.math.exp
import kotlin.math.min
import kotlin.random.Random

private val logger = KotlinLogging.logger {}

object PeerDB {
    const val DELAY = 60 * 60
    private const val MAX_SIZE = 10000
    private const val VERSION = 2
    private val peers = SynchronizedHashMap<Address, Entry>(MAX_SIZE)
    private val PEER_KEY = "peer".toByteArray()
    private val STATE_KEY = "db".toByteArray()
    private val VERSION_KEY = "version".toByteArray()

    private fun setVersion(batch: LevelDB.WriteBatch) {
        val version = BinaryEncoder()
        version.encodeVarInt(VERSION)
        batch.put(PEER_KEY, VERSION_KEY, version.toBytes())
    }

    init {
        val stateBytes = LevelDB.get(PEER_KEY, STATE_KEY)
        val versionBytes = LevelDB.get(PEER_KEY, VERSION_KEY)

        val version = if (versionBytes != null) {
            BinaryDecoder.fromBytes(versionBytes).decodeVarInt()
        } else {
            1
        }

        val hashMap = if (version == VERSION) {
            if (stateBytes != null) {
                BinaryDecoder.fromBytes(stateBytes).decode(HashMapSerializer(Address.serializer(), Entry.serializer()))
            } else {
                emptyMap<Address, Entry>()
            }
        } else if (version == 1) {
            val batch = LevelDB.createWriteBatch()

            val hashMapV1 = if (stateBytes != null) {
                val stateV1 = BinaryDecoder.fromBytes(stateBytes).decode(HashMapSerializer(Address.serializer(), EntryV1.serializer()))
                logger.info("Importing ${stateV1.size} addresses from PeerDB v1")
                val result = HashMap<Address, Entry>(stateV1.size)
                stateV1.forEach { (address, entryV1) ->
                    result.put(address, Entry.fromV1(entryV1))
                }
                result
            } else {
                emptyMap<Address, Entry>()
            }

            setVersion(batch)

            if (hashMapV1.isEmpty())
                batch.write()
            else
                commitImpl(hashMapV1, batch, false)

            hashMapV1
        } else {
            throw RuntimeException("Unknown database version $version")
        }

        logger.info("Loaded ${hashMap.size} peer addresses")

        peers.map.putAll(hashMap)

        Runtime.addShutdownHook { commit(true) }
        Runtime.launch { oldEntriesRemover() }
    }

    suspend fun size(): Int {
        return peers.size()
    }

    suspend fun isEmpty(): Boolean {
        return peers.isEmpty()
    }

    suspend fun isLow(): Boolean {
        return peers.size() < 100
    }

    suspend fun connected(address: Address, time: Long, userAgent: String) {
        if (address.isLocal()) return
        peers.mutex.withLock {
            val entry = peers.map.get(address)
            if (entry != null)
                entry.connected(time, userAgent)
            else
                peers.map.put(address, Entry.newConnected(time, userAgent))
        }
    }

    suspend fun failed(address: Address, time: Long) {
        if (Node.isOffline()) return
        peers.mutex.withLock {
            peers.map.get(address)?.failed(time)
        }
    }

    suspend fun getAll(): ArrayList<Pair<Address, Entry>> {
        return peers.copyToArray()
    }

    suspend fun getSeed(): List<Address> {
        return peers.filterToKeyList { address, entry -> address.port == Node.DEFAULT_P2P_PORT && entry.isReliable() }
    }

    suspend fun getCandidate(filter: List<Address>): Address? {
        val candidates = peers.filterToKeyList { address, _ -> !filter.contains(address) }
        if (candidates.isEmpty())
            return null
        return candidates[Random.nextInt(candidates.size)]
    }

    suspend fun getCandidates(n: Int, filter: List<Address>): List<Address> {
        val candidates = peers.filterToKeyList { address, _ -> !filter.contains(address) }
        if (candidates.isEmpty())
            return emptyList()
        candidates.shuffle()
        val x = min(candidates.size, n)
        return candidates.take(x)
    }

    suspend fun getRandom(n: Int): ArrayList<Address> {
        val candidates = peers.keys()
        candidates.shuffle()
        val x = min(candidates.size, n)
        val result = ArrayList<Address>(x)
        for (i in 0 until x)
            result.add(candidates[i])
        return result
    }

    suspend fun add(newPeers: List<Address>, from: Address) = peers.mutex.withLock {
        if (peers.map.size >= MAX_SIZE)
            return
        newPeers.forEach {
            addImpl(it, from)
        }
    }

    private fun addImpl(peer: Address, from: Address): Boolean {
        if (peer.network.isDisabled())
            return false
        if (peer.isLocal())
            return false
        if (peer.isPrivate() && !from.isPrivate())
            return false
        if (peers.map.containsKey(peer))
            return false
        peers.map.put(peer, Entry.new(from))
        return true
    }

    suspend fun contains(peer: Address): Boolean {
        return peers.containsKey(peer)
    }

    private suspend fun oldEntriesRemover() {
        while (true) {
            delay(DELAY)
            if (Node.isOffline()) continue

            val toRemove = ArrayList<Address>()
            peers.mutex.withLock {
                val currTime = Runtime.time()
                peers.map.forEach { (address, entry) ->
                    if (entry.isOld(currTime))
                        toRemove.add(address)
                }
                if (!toRemove.isEmpty()) {
                    toRemove.forEach { peers.map.remove(it) }
                    val batch = LevelDB.createWriteBatch()
                    commitImpl(peers.map, batch, false)
                    logger.info("Removed ${toRemove.size} old entries from peer db")
                }
            }
        }
    }

    private suspend fun commit(sync: Boolean = false) = peers.mutex.withLock {
        val batch = LevelDB.createWriteBatch()
        commitImpl(peers.map, batch, sync)
    }

    private fun commitImpl(map: Map<Address, Entry>, batch: LevelDB.WriteBatch, sync: Boolean) {
        val bytes = BinaryEncoder.toBytes(HashMapSerializer(Address.serializer(), Entry.serializer()), map)
        batch.put(PEER_KEY, STATE_KEY, bytes)
        batch.write(sync)
    }

    @Serializable
    class NetworkStat(
            var lastConnected: Long,
            var userAgent: String,
            val stat2H: UptimeStat,
            val stat8H: UptimeStat,
            val stat1D: UptimeStat,
            val stat1W: UptimeStat,
            val stat1M: UptimeStat
    ) {
        internal constructor(lastConnected: Long, userAgent: String) : this(
                lastConnected,
                userAgent,
                UptimeStat(),
                UptimeStat(),
                UptimeStat(),
                UptimeStat(),
                UptimeStat()
        )
    }

    @Serializable
    class Entry(
            val from: Address,
            var attempts: Int,
            var lastTry: Long,
            var stat: NetworkStat?
    ) {
        fun toJson(address: Address) = Json.toJson(Info.serializer(), Info(this, address))

        fun failed(time: Long) {
            stat?.let { updateUptimeStat(it, false, time) }
            attempts += 1
            lastTry = time
        }

        fun connected(time: Long, userAgent: String) {
            if (stat != null) {
                stat!!.lastConnected = time
                stat!!.userAgent = userAgent
            } else {
                stat = NetworkStat(time, userAgent)
            }
            updateUptimeStat(stat!!, true, time)
            attempts = 0
            lastTry = time
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

        companion object {
            internal fun fromV1(v1: EntryV1) = Entry(v1.from, v1.attempts, v1.lastTry, null)
            fun new(from: Address) = Entry(from, 0, 0, null)
            fun newConnected(time: Long, userAgent: String) = Entry(Address.LOOPBACK, 0, 0, NetworkStat(time, userAgent))
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
    internal class EntryV1(val from: Address, val attempts: Int, val lastTry: Long, val lastConnected: Long)
}
