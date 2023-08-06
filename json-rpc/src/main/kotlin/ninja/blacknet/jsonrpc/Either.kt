/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

/**
 * A value that can be either [Left] or [Right].
 */
public sealed class Either<out L ,out R>

/**
 * A left value of [Either] usually is an error but may be anything.
 */
public class Left<L>(public val left: L) : Either<L, Nothing>()

/**
 * A right value of [Either] usually is a success but may be anything.
 */
public class Right<R>(public val right: R) : Either<Nothing, R>()
