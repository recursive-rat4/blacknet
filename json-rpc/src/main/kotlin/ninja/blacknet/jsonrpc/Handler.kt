/*
 * Copyright (c) 2020-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

/**
 * A handler for a JSON-RPC request.
 *
 * @param T a type of result that is serializable to JSON.
 */
public interface Handler<T> {
    /**
     * Handle a JSON-RPC request and return a result to respond if the request is not a notification.
     */
    public fun handle(): T
}
