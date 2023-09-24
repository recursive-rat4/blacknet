/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.security.SecureRandom
import javax.crypto.Cipher
import javax.crypto.Cipher.DECRYPT_MODE
import javax.crypto.Cipher.ENCRYPT_MODE
import javax.crypto.spec.ChaCha20ParameterSpec
import javax.crypto.spec.SecretKeySpec

/**
 * ChaCha20 stream cipher.
 */
object ChaCha20 {
    private const val ALGORITHM = "ChaCha20"
    private const val IV_SIZE = 12
    private val random = SecureRandom()

    internal fun encrypt(key: ByteArray, iv: ByteArray, bytes: ByteArray): ByteArray {
        val encrypted = ByteArray(bytes.size + IV_SIZE)
        System.arraycopy(iv, 0, encrypted, 0, IV_SIZE)
        val cipher = Cipher.getInstance(ALGORITHM)
        cipher.init(ENCRYPT_MODE, SecretKeySpec(key, ALGORITHM), ChaCha20ParameterSpec(iv, 0))
        return if (cipher.doFinal(bytes, 0, bytes.size, encrypted, IV_SIZE) == bytes.size)
            encrypted
        else
            throw IllegalStateException("Unexpected number of bytes")
    }

    /**
     * Returns a [ByteArray] encrypted with [key].
     */
    fun encrypt(key: ByteArray, bytes: ByteArray): ByteArray {
        val iv = random.nextBytes(IV_SIZE)
        return encrypt(key, iv, bytes)
    }

    /**
     * Returns a [ByteArray] decrypted with [key] or `null`.
     */
    fun decrypt(key: ByteArray, bytes: ByteArray): ByteArray? {
        val size = bytes.size - IV_SIZE
        if (size < 0) return null
        val iv = bytes.copyOf(IV_SIZE)
        val decrypted = ByteArray(size)
        val cipher = Cipher.getInstance(ALGORITHM)
        cipher.init(DECRYPT_MODE, SecretKeySpec(key, ALGORITHM), ChaCha20ParameterSpec(iv, 0))
        return if (cipher.doFinal(bytes, IV_SIZE, size, decrypted, 0) == size)
            decrypted
        else
            null
    }

    /**
     * Returns an encrypted [string] with [key] using the UTF-8 charset.
     */
    fun encryptUtf8(key: ByteArray, string: String): ByteArray {
        return encrypt(key, string.toByteArray(Charsets.UTF_8))
    }

    /**
     * Returns a decrypted [String] with [key] using the UTF-8 charset or `null`.
     */
    fun decryptUtf8(key: ByteArray, bytes: ByteArray): String? {
        return decrypt(key, bytes)?.let { decrypted ->
            try {
                String(decrypted, Charsets.UTF_8)
            } catch (e: Throwable) {
                null
            }
        }
    }
}

internal fun SecureRandom.nextBytes(n: Int): ByteArray {
    val bytes = ByteArray(n)
    nextBytes(bytes)
    return bytes
}
