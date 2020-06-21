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
import io.ktor.application.ApplicationCall
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import kotlin.math.min
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.MapSerializer
import kotlinx.serialization.builtins.list
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.json.JsonElement
import ninja.blacknet.coding.toHex
import ninja.blacknet.core.AccountState
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.*
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.WalletDB
import ninja.blacknet.ktor.requests.Request
import ninja.blacknet.ktor.requests.get
import ninja.blacknet.ktor.requests.post
import ninja.blacknet.transaction.TxType
import ninja.blacknet.serialization.BinaryDecoder

fun Route.wallet() {
    @Serializable
    class GenerateAccount(
            val wordlist: String = "english"
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val wordlist = Wordlists.get(wordlist)

            return call.respondJson(NewMnemonicInfo.serializer(), NewMnemonicInfo.new(wordlist))
        }
    }

    //get(GenerateAccount.serializer(), "/api/v2/generateaccount")
    get(GenerateAccount.serializer(), "/api/v2/generateaccount/{wordlist?}")

    @Serializable
    class Address(
            val address: String
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val info = AddressInfo.fromString(address)

            return call.respondJson(AddressInfo.serializer(), info)
        }
    }

    get(Address.serializer(), "/api/v2/address")
    get(Address.serializer(), "/api/v2/address/{address}")

    @Serializable
    class Mnemonic(
            val mnemonic: String
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val info = MnemonicInfo.fromString(mnemonic)

            return call.respondJson(MnemonicInfo.serializer(), info)
        }
    }

    post(Mnemonic.serializer(), "/api/v2/mnemonic")

    @Serializable
    class DecryptPaymentId(
            val mnemonic: PrivateKey,
            val from: PublicKey,
            val message: String
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val privateKey = mnemonic
            val decrypted = PaymentId.decrypt(privateKey, from, message)

            return if (decrypted != null)
                call.respond(decrypted)
            else
                call.respond(HttpStatusCode.BadRequest, "Decryption failed")
        }
    }

    post(DecryptPaymentId.serializer(), "/api/v2/decryptpaymentid")
    post(DecryptPaymentId.serializer(), "/api/v2/decryptmessage")

    @Serializable
    class SignMessage(
            val mnemonic: PrivateKey,
            val message: String
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val privateKey = mnemonic
            val signature = Message.sign(privateKey, message)

            return call.respond(signature.toString())
        }
    }

    post(SignMessage.serializer(), "/api/v2/signmessage")

    @Serializable
    class VerifyMessage(
            val from: PublicKey,
            val signature: Signature,
            val message: String
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val result = Message.verify(from, signature, message)

            return call.respond(result.toString())
        }
    }

    get(VerifyMessage.serializer(), "/api/v2/verifymessage")
    get(VerifyMessage.serializer(), "/api/v2/verifymessage/{from}/{signature}/{message}")

    @Serializable
    class Transactions(
            val address: PublicKey
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = WalletDB.mutex.withLock {
            val publicKey = address
            val wallet = WalletDB.getWalletImpl(publicKey)
            val transactions = newHashMapWithExpectedSize<String, JsonElement>(wallet.transactions.size)
            wallet.transactions.forEach { (hash, txData) ->
                transactions.put(hash.toString(), txData.toJson())
            }

            return call.respondJson(MapSerializer(String.serializer(), JsonElement.serializer()), transactions)
        }
    }

    //get(Transactions.serializer(), "/api/v2/wallet/transactions")
    //get(Transactions.serializer(), "/api/v2/wallet/transactions/{address}")
    get(Transactions.serializer(), "/api/v2/wallet/{address}/transactions")

    @Serializable
    class OutLeases(
            val address: PublicKey
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = WalletDB.mutex.withLock {
            val publicKey = address
            val wallet = WalletDB.getWalletImpl(publicKey)

            return call.respondJson(AccountState.Lease.serializer().list, wallet.outLeases)
        }
    }

    //get(OutLeases.serializer(), "/api/v2/wallet/outleases")
    //get(OutLeases.serializer(), "/api/v2/wallet/outleases/{address}")
    get(OutLeases.serializer(), "/api/v2/wallet/{address}/outleases")

    @Serializable
    class Sequence(
            val address: PublicKey
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val publicKey = address

            return call.respond(WalletDB.getSequence(publicKey).toString())
        }
    }

    //get(Sequence.serializer(), "/api/v2/wallet/sequence")
    //get(Sequence.serializer(), "/api/v2/wallet/sequence/{address}")
    get(Sequence.serializer(), "/api/v2/wallet/{address}/sequence")

    @Serializable
    class TransactionRequest(
            val address: PublicKey,
            val hash: Hash,
            val raw: Boolean = false
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = WalletDB.mutex.withLock {
            val publicKey = address
            val wallet = WalletDB.getWalletImpl(publicKey)
            val txData = wallet.transactions.get(hash)
            return if (txData != null) {
                val bytes = WalletDB.getTransactionImpl(hash)!!
                if (raw) {
                    call.respond(bytes.toHex())
                } else {
                    val tx = BinaryDecoder(bytes).decode(Transaction.serializer())
                    call.respondJson(TransactionInfo.serializer(), TransactionInfo(tx, hash, bytes.size, txData.types))
                }
            } else {
                call.respond(HttpStatusCode.BadRequest, "Transaction not found")
            }
        }
    }

    //get(TransactionRequest.serializer(), "/api/v2/wallet/transaction")
    //get(TransactionRequest.serializer(), "/api/v2/wallet/transaction/{address}/{hash}/{raw?}")
    get(TransactionRequest.serializer(), "/api/v2/wallet/{address}/transaction/{hash}/{raw?}")

    @Serializable
    class Confirmations(
            val address: PublicKey,
            val hash: Hash
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val publicKey = address
            val result = WalletDB.getConfirmations(publicKey, hash)
            return if (result != null)
                call.respond(result.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction not found")
        }
    }

    //get(Confirmations.serializer(), "/api/v2/wallet/confirmations")
    //get(Confirmations.serializer(), "/api/v2/wallet/confirmations/{address}/{hash}")
    get(Confirmations.serializer(), "/api/v2/wallet/{address}/confirmations/{hash}")

    @Serializable
    class ReferenceChain(
            val address: PublicKey
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            @Suppress("UNUSED_VARIABLE")
            val publicKey = address
            val result = WalletDB.referenceChain()
            return call.respond(result.toString())
        }
    }

    //get(ReferenceChain.serializer(), "/api/v2/wallet/referencechain")
    //get(ReferenceChain.serializer(), "/api/v2/wallet/referencechain/{address}")
    get(ReferenceChain.serializer(), "/api/v2/wallet/{address}/referencechain")

    @Serializable
    class TxCount(
            val address: PublicKey
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = WalletDB.mutex.withLock {
            val publicKey = address
            val wallet = WalletDB.getWalletImpl(publicKey)
            val count = wallet.transactions.size
            return call.respond(count.toString())
        }
    }

    //get(TxCount.serializer(), "/api/v2/wallet/txcount")
    //get(TxCount.serializer(), "/api/v2/wallet/txcount/{address}")
    get(TxCount.serializer(), "/api/v2/wallet/{address}/txcount")

    @Serializable
    class ListTransactions(
            val address: PublicKey,
            val offset: Int = 0,
            val max: Int = 100,
            val type: Int? = null
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = WalletDB.mutex.withLock {
            val publicKey = address
            val wallet = WalletDB.getWalletImpl(publicKey)
            BlockDB.mutex.withLock {
                val size = wallet.transactions.size
                if (offset < 0 || offset > size)
                    return call.respond(HttpStatusCode.BadRequest, "Invalid offset")
                if (max < 0 || max > Int.MAX_VALUE)
                    return call.respond(HttpStatusCode.BadRequest, "Invalid max")
                val toIndex = min(offset + max, size)
                val transactions = ArrayList<WalletTransactionInfo>(min(max, size))
                val state = LedgerDB.state()
                val list = wallet.transactions.entries.sortedByDescending { (_, txData) -> txData.time }
                if (type == null) {
                    for (index in offset until toIndex) {
                        val (hash, txData) = list[index]
                        val bytes = WalletDB.getTransactionImpl(hash)!!
                        val tx = BinaryDecoder(bytes).decode(Transaction.serializer())
                        transactions.add(WalletTransactionInfo(
                                TransactionInfo(tx, hash, bytes.size, txData.types),
                                txData.confirmationsImpl(state),
                                txData.time
                        ))
                    }
                } else {
                    require(offset >= 0) { "偏移不能为负数" }
                    var offsetNumber = offset
                    val type = type.toUByte().toByte().also { TxType.getSerializer(it) /* 请校验请求引数 */ }
                    for (index in 0 until list.size) {
                        val (hash, txData) = list[index]
                        val filter = txData.types.filter { it.type == type }
                        if (filter.size == 0)
                            continue
                        if (offsetNumber != 0) {
                            offsetNumber -= 1
                            continue
                        }
                        val bytes = WalletDB.getTransactionImpl(hash)!!
                        val tx = BinaryDecoder(bytes).decode(Transaction.serializer())
                        transactions.add(WalletTransactionInfo(
                            TransactionInfo(tx, hash, bytes.size, filter),
                            txData.confirmationsImpl(state),
                            txData.time
                        ))
                        if (transactions.size == max)
                            break
                    }
                }
                return call.respondJson(WalletTransactionInfo.serializer().list, transactions)
            }
        }
    }

    //get(ListTransactions.serializer(), "/api/v2/wallet/listtransactions")
    //get(ListTransactions.serializer(), "/api/v2/wallet/listtransactions/{address}/{offset?}/{max?}/{type?}")
    get(ListTransactions.serializer(), "/api/v2/wallet/{address}/listtransactions/{offset?}/{max?}/{type?}")

    @Serializable
    class ListSinceBlockInfo(
            val transactions: List<WalletTransactionInfo>,
            val lastBlockHash: Hash
    )

    @Serializable
    class ListSinceBlock(
            val address: PublicKey,
            val hash: Hash = Hash.ZERO
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = WalletDB.mutex.withLock {
            val publicKey = address
            val wallet = WalletDB.getWalletImpl(publicKey)
            BlockDB.mutex.withLock {
                val height = LedgerDB.getChainIndex(hash)?.height ?: return call.respond(HttpStatusCode.BadRequest, "Block not found")
                val state = LedgerDB.state()
                if (height >= state.height - PoS.MATURITY)
                    return call.respond(HttpStatusCode.BadRequest, "Block not finalized")
                val transactions = ArrayList<WalletTransactionInfo>()
                wallet.transactions.forEach { (hash, txData) ->
                    if (txData.height != 0 && height >= txData.height) {
                        val bytes = WalletDB.getTransactionImpl(hash)!!
                        val tx = BinaryDecoder(bytes).decode(Transaction.serializer())
                        transactions.add(WalletTransactionInfo(
                                TransactionInfo(tx, hash, bytes.size, txData.types),
                                txData.confirmationsImpl(state),
                                txData.time
                        ))
                    }
                }
                return call.respondJson(ListSinceBlockInfo.serializer(), ListSinceBlockInfo(transactions, state.rollingCheckpoint))
            }
        }
    }

    //get(ListSinceBlock.serializer(), "/api/v2/wallet/listsinceblock")
    //get(ListSinceBlock.serializer(), "/api/v2/wallet/listsinceblock/{address}/{hash?}")
    get(ListSinceBlock.serializer(), "/api/v2/wallet/{address}/listsinceblock/{hash?}")
}
