/*
 * Copyright (c) 2025 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef BLACKNET_WALLET_WALLET_H
#define BLACKNET_WALLET_WALLET_H

#include "blacknet-config.h"

#include <cstddef>
#include <exception>
#include <span>
#include <string>
#include <vector>
#include <fmt/format.h>

#include "mode.h"
#include "sqlite.h"

namespace blacknet::wallet {

class Exception : public std::exception {
    std::string message;
public:
    Exception(const std::string& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

class Wallet {
    sqlite::Connection connection;
    sqlite::Statement select_transaction;
    sqlite::Statement insert_transaction;

    Wallet(sqlite::Connection&& connection) :
        connection(std::move(connection)),
        select_transaction(this->
            connection.prepare("SELECT bytes FROM transactions WHERE id = ?;")
        ),
        insert_transaction(this->
            connection.prepare("INSERT INTO transactions VALUES(?, ?);")
        ) {}
public:
    constexpr Wallet() = delete;
    constexpr Wallet(const Wallet&) = delete;
    constexpr Wallet(Wallet&&) noexcept = default;

    constexpr Wallet& operator = (const Wallet&) = delete;
    constexpr Wallet& operator = (Wallet&&) noexcept = default;

    std::vector<std::byte> transaction(
        const std::span<const std::byte>& id
    ) {
        auto binder = select_transaction.binder();
        binder.blob(1, id);
        for (auto&& row : select_transaction.evaluate()) {
            auto&& bytes = row.blob(0);
            return { bytes.cbegin(), bytes.cend() };
        }
        throw Exception("Transaction not found");
    }

    void transaction(
        const std::span<const std::byte>& id,
        const std::span<const std::byte>& bytes
    ) {
        auto binder = insert_transaction.binder();
        binder.blob(1, id);
        binder.blob(2, bytes);
        insert_transaction.execute();
    }

    static Wallet create(const char* filename) {
        return initialize(sqlite::Connection::create(filename));
    }

    static Wallet open(const char* filename) {
        return attach(sqlite::Connection::open(filename));
    }

    static Wallet ephemeral() {
        return initialize(sqlite::Connection::memory());
    }

    static Wallet attach(sqlite::Connection&& connection) {
        checkMagic(connection);
        configure(connection);
        return connection;
    }
private:
    static void configure(sqlite::Connection& connection) {
        connection.execute("PRAGMA locking_mode = EXCLUSIVE;");
#ifdef BLACKNET_HAVE_FULLFSYNC
        connection.execute("PRAGMA fullfsync = TRUE;");
#endif
        connection.execute("PRAGMA synchronous = FULL;");
        connection.execute("PRAGMA journal_mode = DELETE;");
    }

    static void checkMagic(sqlite::Connection& connection) {
        int64_t magic = 0;
        auto rows = connection.evaluate("PRAGMA application_id;");
        for (auto&& row : rows) {
            magic = row.integer(0);
        }
        if (magic == compat::mode()->network_magic())
            return;
        throw Exception(
            fmt::format(
                "This SQLite database doesn't look like {} wallet",
                compat::mode()->agent_name()
            )
        );
    }

    static void setMagic(sqlite::Connection& connection) {
        // Pragmas may be executed during statement preparation,
        // thus have to resort to string formatting
        {
            std::string query{
                fmt::format("PRAGMA application_id = {};", compat::mode()->network_magic())
            };
            connection.execute(query.c_str());
        }
        connection.execute("PRAGMA user_version = 1;");
    }

    static void createSchema(sqlite::Connection& connection) {
        connection.execute(
            "CREATE TABLE transactions(id BLOB PRIMARY KEY, bytes BLOB NOT NULL) STRICT;"
        );
    }

    static Wallet initialize(sqlite::Connection&& connection) {
        configure(connection);
        setMagic(connection);
        createSchema(connection);
        return connection;
    }
};

}

#endif
