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

import com.google.common.collect.Maps.newHashMapWithExpectedSize
import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.request.receiveParameters
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import io.ktor.routing.post
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.builtins.*
import kotlinx.serialization.json.JsonElement
import ninja.blacknet.coding.toHex
import ninja.blacknet.core.AccountState
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.*
import ninja.blacknet.db.WalletDB

fun Route.wallet() {
    get("/api/v2/generateaccount/{wordlist?}") {
        val wordlist = Wordlists.get(call.parameters["wordlist"] ?: "english") ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid wordlist")

        call.respondJson(NewMnemonicInfo.serializer(), NewMnemonicInfo.new(wordlist))
    }

    get("/api/v2/address/{address}") {
        val info = AddressInfo.fromString(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")

        call.respondJson(AddressInfo.serializer(), info)
    }

    post("/api/v2/mnemonic") {
        val parameters = call.receiveParameters()
        val info = MnemonicInfo.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")

        call.respondJson(MnemonicInfo.serializer(), info)
    }

    post("/api/v2/decryptmessage") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")
        val publicKey = Address.decode(parameters["from"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid from")
        val message = parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid message")

        val decrypted = Message.decrypt(privateKey, publicKey, message)
        if (decrypted != null)
            call.respond(decrypted)
        else
            call.respond(HttpStatusCode.BadRequest, "Decryption failed")
    }

    post("/api/v2/signmessage") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")
        val message = parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid message")

        val signature = Message.sign(privateKey, message)

        call.respond(signature.toString())
    }

    get("/api/v2/verifymessage/{from}/{signature}/{message}") {
        val publicKey = Address.decode(call.parameters["from"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid from")
        val signature = Signature.fromString(call.parameters["signature"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid signature")
        val message = call.parameters["message"] ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid message")

        val result = Message.verify(publicKey, signature, message)

        call.respond(result.toString())
    }

    get("/api/v2/wallet/{address}/transactions") {
        val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")

        val transactions = WalletDB.mutex.withLock {
            val wallet = WalletDB.getWalletImpl(publicKey)
            val transactions = newHashMapWithExpectedSize<String, JsonElement>(wallet.transactions.size)
            wallet.transactions.forEach { (hash, txData) ->
                transactions.put(hash.toString(), txData.toJson())
            }
            transactions
        }
        call.respondJson(MapSerializer(String.serializer(), JsonElement.serializer()), transactions)
    }

    get("/api/v2/wallet/{address}/outleases") {
        val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")

        WalletDB.mutex.withLock {
            val wallet = WalletDB.getWalletImpl(publicKey)
            call.respondJson(AccountState.Lease.serializer().list, wallet.outLeases)
        }
    }

    get("/api/v2/wallet/{address}/sequence") {
        val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")

        call.respond(WalletDB.getSequence(publicKey).toString())
    }

    get("/api/v2/wallet/{address}/transaction/{hash}/{raw?}") {
        val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")
        val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hash")
        val raw = call.parameters["raw"]?.toBoolean() ?: false

        WalletDB.mutex.withLock {
            val wallet = WalletDB.getWalletImpl(publicKey)
            val txData = wallet.transactions.get(hash)
            if (txData != null) {
                val bytes = WalletDB.getTransactionImpl(hash)
                if (bytes != null) {
                    if (raw) {
                        call.respond(bytes.toHex())
                    } else {
                        val tx = Transaction.deserialize(bytes)
                        call.respondJson(TransactionInfo.serializer(), TransactionInfo(tx, hash, bytes.size, txData.types))
                    }
                } else {
                    call.respond(HttpStatusCode.BadRequest, "Transaction not found")
                }
            } else {
                call.respond(HttpStatusCode.BadRequest, "Transaction not found")
            }
        }
    }

    get("/api/v2/wallet/{address}/confirmations/{hash}") {
        val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")
        val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hash")

        val result = WalletDB.getConfirmations(publicKey, hash)
        if (result != null)
            call.respond(result.toString())
        else
            call.respond(HttpStatusCode.BadRequest, "Transaction not found")
    }

    get("/api/v2/wallet/{address}/referencechain") {
        @Suppress("UNUSED_VARIABLE")
        val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")

        val result = WalletDB.referenceChain()
        call.respond(result.toString())
    }
}
