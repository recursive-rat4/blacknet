/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import io.ktor.server.routing.Route
import java.util.HashMap.newHashMap
import kotlin.concurrent.withLock
import kotlin.math.min
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.MapSerializer
import kotlinx.serialization.builtins.serializer
import ninja.blacknet.Kernel
import ninja.blacknet.core.AccountState
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.*
import ninja.blacknet.db.CoinDB
import ninja.blacknet.db.Genesis
import ninja.blacknet.db.WalletDB
import ninja.blacknet.rpc.requests.*
import ninja.blacknet.rpc.v1.AddressInfo
import ninja.blacknet.rpc.v1.NewMnemonicInfo
import ninja.blacknet.rpc.v1.toHex
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.transaction.TxData
import ninja.blacknet.transaction.TxType

@Serializable
class GenerateAccount(
    val wordlist: String = "english"
) : Request {
    override fun handle(): TextContent {
        val wordlist = Wordlists.get(wordlist)

        return respondJson(NewMnemonicInfo.serializer(), NewMnemonicInfo.new(wordlist))
    }
}

@Serializable
class Address(
    val address: String
) : Request {
    override fun handle(): TextContent {
        val info = AddressInfo.fromString(address)

        return respondJson(AddressInfo.serializer(), info)
    }
}

@Serializable
class Mnemonic(
    val mnemonic: String
) : Request {
    override fun handle(): TextContent {
        val info = MnemonicInfo.fromString(mnemonic)

        return respondJson(MnemonicInfo.serializer(), info)
    }
}

@Serializable
class DecryptPaymentId(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val from: PublicKey,
    val message: String
) : Request {
    override fun handle(): TextContent {
        val decrypted = PaymentId.decrypt(privateKey, from, message)

        return if (decrypted != null)
            respondText(decrypted)
        else
            respondError("Decryption failed")
    }
}

@Serializable
class SignMessage(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val message: String
) : Request {
    override fun handle(): TextContent {
        val signature = Message.sign(privateKey, message)

        return respondText(SignatureSerializer.encode(signature))
    }
}

@Serializable
class VerifyMessage(
    val from: PublicKey,
    @Serializable(with = SignatureSerializer::class)
    val signature: ByteArray,
    val message: String
) : Request {
    override fun handle(): TextContent {
        val result = Message.verify(from, signature, message)

        return respondText(result.toString())
    }
}

@Serializable
class Transactions(
    @SerialName("address")
    val publicKey: PublicKey
) : Request {
    override fun handle(): TextContent = WalletDB.reentrant.readLock().withLock {
        val wallet = WalletDB.getWalletImpl(publicKey)
        val transactions = newHashMap<String, TransactionDataInfo>(wallet.transactions.size)
        wallet.transactions.forEach { (hash, txData) ->
            transactions.put(hash.toString(), TransactionDataInfo(txData))
        }

        return respondJson(MapSerializer(String.serializer(), TransactionDataInfo.serializer()), transactions)
    }
}

@Serializable
class OutLeases(
    @SerialName("address")
    val publicKey: PublicKey
) : Request {
    override fun handle(): TextContent = WalletDB.reentrant.readLock().withLock {
        val wallet = WalletDB.getWalletImpl(publicKey)

        return respondJson(ListSerializer(AccountState.Lease.serializer()), wallet.outLeases)
    }
}

@Serializable
class Sequence(
    @SerialName("address")
    val publicKey: PublicKey
) : Request {
    override fun handle(): TextContent {
        return respondText(WalletDB.getSequence(publicKey).toString())
    }
}

@Serializable
class TransactionRequest(
    @SerialName("address")
    val publicKey: PublicKey,
    val hash: Hash,
    val raw: Boolean = false
) : Request {
    override fun handle(): TextContent = WalletDB.reentrant.readLock().withLock {
        val wallet = WalletDB.getWalletImpl(publicKey)
        val txData = wallet.transactions.get(hash)
        return if (txData != null) {
            val bytes = WalletDB.getTransactionImpl(hash)!!
            if (raw) {
                respondText(@Suppress("DEPRECATION") bytes.toHex())
            } else {
                val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes)
                respondJson(TransactionInfo.serializer(), TransactionInfo(tx, hash, bytes.size, txData.types))
            }
        } else {
            respondError("Transaction not found")
        }
    }
}

@Serializable
class Confirmations(
    @SerialName("address")
    val publicKey: PublicKey,
    val hash: Hash
) : Request {
    override fun handle(): TextContent {
        val result = WalletDB.getConfirmations(publicKey, hash)
        return if (result != null)
            respondText(result.toString())
        else
            respondError("Transaction not found")
    }
}

@Serializable
class Anchor(
    @SerialName("address")
    val publicKey: PublicKey
) : Request {
    override fun handle(): TextContent {
        val result = WalletDB.anchor()
        return respondText(result.toString())
    }
}

@Serializable
class TxCount(
    @SerialName("address")
    val publicKey: PublicKey
) : Request {
    override fun handle(): TextContent = WalletDB.reentrant.readLock().withLock {
        val wallet = WalletDB.getWalletImpl(publicKey)
        val count = wallet.transactions.size
        return respondText(count.toString())
    }
}

@Serializable
class ListTransactions(
    @SerialName("address")
    val publicKey: PublicKey,
    val offset: Int = 0,
    val max: Int = 100,
    val type: UByte? = null
) : Request {
    override fun handle(): TextContent = Kernel.blockDB().reentrant.readLock().withLock {
        WalletDB.reentrant.readLock().withLock<TextContent> {
            val wallet = WalletDB.getWalletImpl(publicKey)
            val size = wallet.transactions.size
            if (offset < 0 || offset > size)
                return respondError("Invalid offset")
            if (max < 0 || max > Int.MAX_VALUE)
                return respondError("Invalid max")
            val toIndex = min(offset + max, size)
            val transactions = ArrayList<WalletTransactionInfo>(min(max, size))
            val state = CoinDB.state()
            val list = wallet.transactions.entries.sortedByDescending { (_, txData) -> txData.time }
            if (type == null) {
                for (index in offset until toIndex) {
                    val (hash, txData) = list[index]
                    val bytes = WalletDB.getTransactionImpl(hash)!!
                    val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes)
                    transactions.add(WalletTransactionInfo(
                        TransactionInfo(tx, hash, bytes.size, txData.types),
                        txData.confirmationsImpl(state),
                        txData.time
                    ))
                }
            } else {
                require(offset >= 0) { "偏移不能为负数" }
                var offsetNumber = offset
                val type = type.also { if (it != TxType.Generated.type) TxType.getSerializer<TxData>(it) /* 请校验请求引数 */ }
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
                    val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes)
                    transactions.add(WalletTransactionInfo(
                        TransactionInfo(tx, hash, bytes.size, filter),
                        txData.confirmationsImpl(state),
                        txData.time
                    ))
                    if (transactions.size == max)
                        break
                }
            }
            return respondJson(ListSerializer(WalletTransactionInfo.serializer()), transactions)
        }
    }
}

@Serializable
class ListSinceBlockInfo(
    val transactions: List<WalletTransactionInfo>,
    val lastBlockHash: Hash
)

@Serializable
class ListSinceBlock(
    @SerialName("address")
    val publicKey: PublicKey,
    val hash: Hash = Genesis.BLOCK_HASH
) : Request {
    override fun handle(): TextContent = Kernel.blockDB().reentrant.readLock().withLock {
        WalletDB.reentrant.readLock().withLock<TextContent> {
            val wallet = WalletDB.getWalletImpl(publicKey)
            val height = CoinDB.blockIndexes.get(hash.bytes)?.height ?: return respondError("Block not found")
            val state = CoinDB.state()
            if (height >= state.height - PoS.ROLLBACK_LIMIT)
                return respondError("Block not checkpointed")
            val transactions = ArrayList<WalletTransactionInfo>()
            wallet.transactions.forEach { (hash, txData) ->
                if (txData.height != 0 && height >= txData.height) {
                    val bytes = WalletDB.getTransactionImpl(hash)!!
                    val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes)
                    transactions.add(WalletTransactionInfo(
                        TransactionInfo(tx, hash, bytes.size, txData.types),
                        txData.confirmationsImpl(state),
                        txData.time
                    ))
                }
            }
            return respondJson(ListSinceBlockInfo.serializer(), ListSinceBlockInfo(transactions, state.rollingCheckpoint))
        }
    }
}

fun Route.wallet() {
    //get(GenerateAccount.serializer(), "/api/v2/generateaccount")
    get(GenerateAccount.serializer(), "/api/v2/generateaccount/{wordlist?}")

    get(Address.serializer(), "/api/v2/address")
    get(Address.serializer(), "/api/v2/address/{address}")

    post(Mnemonic.serializer(), "/api/v2/mnemonic")

    post(DecryptPaymentId.serializer(), "/api/v2/decryptpaymentid")
    post(DecryptPaymentId.serializer(), "/api/v2/decryptmessage")

    post(SignMessage.serializer(), "/api/v2/signmessage")

    get(VerifyMessage.serializer(), "/api/v2/verifymessage")
    get(VerifyMessage.serializer(), "/api/v2/verifymessage/{from}/{signature}/{message}")

    //get(Transactions.serializer(), "/api/v2/wallet/transactions")
    //get(Transactions.serializer(), "/api/v2/wallet/transactions/{address}")
    get(Transactions.serializer(), "/api/v2/wallet/{address}/transactions")

    //get(OutLeases.serializer(), "/api/v2/wallet/outleases")
    //get(OutLeases.serializer(), "/api/v2/wallet/outleases/{address}")
    get(OutLeases.serializer(), "/api/v2/wallet/{address}/outleases")

    //get(Sequence.serializer(), "/api/v2/wallet/sequence")
    //get(Sequence.serializer(), "/api/v2/wallet/sequence/{address}")
    get(Sequence.serializer(), "/api/v2/wallet/{address}/sequence")

    //get(TransactionRequest.serializer(), "/api/v2/wallet/transaction")
    //get(TransactionRequest.serializer(), "/api/v2/wallet/transaction/{address}/{hash}/{raw?}")
    get(TransactionRequest.serializer(), "/api/v2/wallet/{address}/transaction/{hash}/{raw?}")

    //get(Confirmations.serializer(), "/api/v2/wallet/confirmations")
    //get(Confirmations.serializer(), "/api/v2/wallet/confirmations/{address}/{hash}")
    get(Confirmations.serializer(), "/api/v2/wallet/{address}/confirmations/{hash}")

    //get(Anchor.serializer(), "/api/v2/wallet/referencechain")
    //get(Anchor.serializer(), "/api/v2/wallet/referencechain/{address}")
    get(Anchor.serializer(), "/api/v2/wallet/{address}/referencechain")

    //get(TxCount.serializer(), "/api/v2/wallet/txcount")
    //get(TxCount.serializer(), "/api/v2/wallet/txcount/{address}")
    get(TxCount.serializer(), "/api/v2/wallet/{address}/txcount")

    //get(ListTransactions.serializer(), "/api/v2/wallet/listtransactions")
    //get(ListTransactions.serializer(), "/api/v2/wallet/listtransactions/{address}/{offset?}/{max?}/{type?}")
    get(ListTransactions.serializer(), "/api/v2/wallet/{address}/listtransactions/{offset?}/{max?}/{type?}")

    //get(ListSinceBlock.serializer(), "/api/v2/wallet/listsinceblock")
    //get(ListSinceBlock.serializer(), "/api/v2/wallet/listsinceblock/{address}/{hash?}")
    get(ListSinceBlock.serializer(), "/api/v2/wallet/{address}/listsinceblock/{hash?}")
}
