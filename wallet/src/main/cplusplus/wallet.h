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

#include <exception>
#include <string>

#include "sqlite.h"

class WalletException : public std::exception {
    std::string message;
public:
    WalletException(const std::string& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

class Wallet {
    sqlite::Connection connection;

    constexpr Wallet(sqlite::Connection&& connection) : connection(std::move(connection)) {}
public:
    constexpr Wallet() = delete;
    constexpr Wallet(const Wallet&) = delete;
    constexpr Wallet(Wallet&& other) noexcept
        : connection(std::move(other.connection)) {}

    constexpr Wallet& operator = (const Wallet&) = delete;
    constexpr Wallet& operator = (Wallet&& other) noexcept {
        connection = std::move(other.connection);
        return *this;
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
        connection.exec("PRAGMA locking_mode = EXCLUSIVE;");
        connection.exec("PRAGMA fullfsync = TRUE;");
        connection.exec("PRAGMA synchronous = FULL;");
        connection.exec("PRAGMA journal_mode = DELETE;");
    }

    static void checkMagic(sqlite::Connection& connection) {
        int64_t magic = 0;
        auto statement = connection.prepare("PRAGMA application_id;");
        statement.evaluate([&magic](auto& evaluator) {
            magic = evaluator.integer(0);
        });
        if (magic == 0x17895E7D)
            return;
        throw WalletException("This SQLite database doesn't look like Blacknet wallet");
    }

    static void setMagic(sqlite::Connection& connection) {
        connection.exec("PRAGMA application_id = 0x17895E7D;");
        connection.exec("PRAGMA user_version = 1;");
    }

    static Wallet initialize(sqlite::Connection&& connection) {
        configure(connection);
        setMagic(connection);
        return connection;
    }
};

#endif
