/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import net.i2p.crypto.eddsa.math.Curve
import net.i2p.crypto.eddsa.math.Field
import net.i2p.crypto.eddsa.math.ed25519.Ed25519LittleEndianEncoding
import net.i2p.crypto.eddsa.math.ed25519.Ed25519ScalarOps
import net.i2p.crypto.eddsa.spec.EdDSANamedCurveSpec
import net.i2p.crypto.eddsa.spec.EdDSANamedCurveTable
import net.i2p.crypto.eddsa.spec.EdDSAPrivateKeySpec
import net.i2p.crypto.eddsa.EdDSAEngine
import net.i2p.crypto.eddsa.EdDSAPrivateKey
import net.i2p.crypto.eddsa.EdDSAPublicKey
import net.i2p.crypto.eddsa.spec.EdDSAPublicKeySpec
import java.security.MessageDigest

object Ed25519 {
    private val spec: EdDSANamedCurveSpec

    init {
        val field = Field(256,
                byteArrayOfInts(0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f),
                Ed25519LittleEndianEncoding())
        val curve = Curve(field,
                byteArrayOfInts(0xa3, 0x78, 0x59, 0x13, 0xca, 0x4d, 0xeb, 0x75, 0xab, 0xd8, 0x41, 0x41, 0x4d, 0x0a, 0x70, 0x00, 0x98, 0xe8, 0x79, 0x77, 0x79, 0x40, 0xc7, 0x8c, 0x73, 0xfe, 0x6f, 0x2b, 0xee, 0x6c, 0x03, 0x52),
                field.fromByteArray(byteArrayOfInts(0xb0, 0xa0, 0x0e, 0x4a, 0x27, 0x1b, 0xee, 0xc4, 0x78, 0xe4, 0x2f, 0xad, 0x06, 0x18, 0x43, 0x2f, 0xa7, 0xd7, 0xfb, 0x3d, 0x99, 0x00, 0x4d, 0x2b, 0x0b, 0xdf, 0xc1, 0x4f, 0x80, 0x24, 0x83, 0x2b)))
        spec = EdDSANamedCurveSpec(EdDSANamedCurveTable.ED_25519,
                curve,
                Blake2b.BLAKE2_B_512,
                Ed25519ScalarOps(),
                curve.createPoint(byteArrayOfInts(0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66),
                        true))
    }

    class PublicKey(val bytes: ByteArray)
    class PrivateKey(val bytes: ByteArray)
    class Signature(val bytes: ByteArray)

    fun publicKey(privateKey: PrivateKey): PublicKey {
        val key = EdDSAPrivateKeySpec(privateKey.bytes, spec)
        return PublicKey(key.getA().toByteArray())
    }

    fun sign(hash: Blake2b.Hash, privateKey: PrivateKey): Signature {
        val edDSAEngine = EdDSAEngine(MessageDigest.getInstance(Blake2b.BLAKE2_B_512))
        val edDSAPrivateKeySpec = EdDSAPrivateKeySpec(privateKey.bytes, spec)
        val edDSAPrivateKey = EdDSAPrivateKey(edDSAPrivateKeySpec)
        edDSAEngine.initSign(edDSAPrivateKey)
        edDSAEngine.setParameter(EdDSAEngine.ONE_SHOT_MODE)
        edDSAEngine.update(hash.bytes)
        return Signature(edDSAEngine.sign())
    }

    fun verify(signature: Signature, hash: Blake2b.Hash, publicKey: PublicKey): Boolean {
        val edDSAEngine = EdDSAEngine(MessageDigest.getInstance(Blake2b.BLAKE2_B_512))
        val edDSAPublicKeySpec = EdDSAPublicKeySpec(publicKey.bytes, spec)
        val edDSAPublicKey = EdDSAPublicKey(edDSAPublicKeySpec)
        edDSAEngine.initVerify(edDSAPublicKey)
        edDSAEngine.setParameter(EdDSAEngine.ONE_SHOT_MODE)
        edDSAEngine.update(hash.bytes)
        return edDSAEngine.verify(signature.bytes)
    }

    private fun byteArrayOfInts(vararg ints: Int) = ByteArray(ints.size) { pos -> ints[pos].toByte() }
}