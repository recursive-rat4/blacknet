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

#ifndef BLACKNET_WALLET_SQLITE_H
#define BLACKNET_WALLET_SQLITE_H

#include <cstddef>
#include <functional>
#include <span>
#include <string_view>
#include <utility>
#include <sqlite3.h>

namespace sqlite {
class Evaluator {
    friend class Statement;
    sqlite3_stmt* const statement;

    constexpr Evaluator(sqlite3_stmt* const statement) : statement(statement) {}

    void reset() {
        int rc = sqlite3_reset(statement);
        if (rc == SQLITE_OK)
            return;
        std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
    }
public:
    constexpr Evaluator() = delete;
    constexpr Evaluator(const Evaluator&) = delete;
    constexpr Evaluator(Evaluator&&) = delete;
    ~Evaluator() {
        reset();
    }

    constexpr Evaluator& operator = (const Evaluator&) = delete;
    constexpr Evaluator& operator = (Evaluator&&) = delete;

    int columns() {
        return sqlite3_column_count(statement);
    }

    std::span<const std::byte> blob(int column) {
        return {
            reinterpret_cast<const std::byte*>(sqlite3_column_blob(statement, column)),
            static_cast<std::size_t>(sqlite3_column_bytes(statement, column))
        };
    }

    int64_t integer(int column) {
        return sqlite3_column_int64(statement, column);
    }

    std::string_view text(int column) {
        return {
            reinterpret_cast<const char*>(sqlite3_column_text(statement, column)),
            static_cast<std::size_t>(sqlite3_column_bytes(statement, column))
        };
    }
};

class Statement {
    friend class Connection;
    sqlite3_stmt* statement;
public:
    consteval Statement() : statement(nullptr) {}
    constexpr Statement(const Statement&) = delete;
    constexpr Statement(Statement&& other) noexcept : statement(other.statement) {
        other.statement = nullptr;
    }
    ~Statement() {
        int rc = sqlite3_finalize(statement);
        if (rc == SQLITE_OK)
            return;
        std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
    }

    constexpr Statement& operator = (const Statement&) = delete;
    constexpr Statement& operator = (Statement&& other) noexcept {
        std::swap(statement, other.statement);
        return *this;
    }

    constexpr bool isPrepared() const {
        return statement != nullptr;
    }

    void evaluate(const std::function<void(Evaluator&)>& fun) {
        if (statement) {
            Evaluator evaluator(statement);
            while (true) {
                int rc = sqlite3_step(statement);
                if (rc == SQLITE_ROW) {
                    fun(evaluator);
                } else if (rc == SQLITE_DONE) {
                    break;
                } else {
                    std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
                    break;
                }
            }
        } else {
            std::cerr << "SQLite is not prepared" << std::endl;
        }
    }

    void clear() {
        if (statement) {
            int rc = sqlite3_clear_bindings(statement);
            if (rc == SQLITE_OK)
                return;
            std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
        } else {
            std::cerr << "SQLite is not prepared" << std::endl;
        }
    }
};

class Connection {
    sqlite3* connection;
public:
    consteval Connection() : connection(nullptr) {}
    constexpr Connection(const Connection&) = delete;
    constexpr Connection(Connection&& other) noexcept : connection(other.connection) {
        other.connection = nullptr;
    }
    ~Connection() {
        int rc = sqlite3_close(connection);
        if (rc == SQLITE_OK)
            return;
        std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
    }

    constexpr Connection& operator = (const Connection&) = delete;
    constexpr Connection& operator = (Connection&& other) noexcept {
        std::swap(connection, other.connection);
        return *this;
    }

    constexpr bool isConnected() const {
        return connection != nullptr;
    }

    void exec(const char* query) {
        if (connection) {
            int rc = sqlite3_exec(connection, query, nullptr, nullptr, nullptr);
            if (rc == SQLITE_OK)
                return;
            std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
        } else {
            std::cerr << "SQLite is not connected" << std::endl;
        }
    }

    Statement prepare(const char* query, int flags = 0) {
        if (connection) {
            Statement sqlite;
            int rc = sqlite3_prepare_v3(connection, query, -1, flags, &sqlite.statement, nullptr);
            if (rc == SQLITE_OK)
                return sqlite;
            std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
            return {};
        } else {
            std::cerr << "SQLite is not connected" << std::endl;
            return {};
        }
    }

    static Connection create(const char* filename) {
        Connection sqlite(open(filename, SQLITE_OPEN_CREATE));
        if (sqlite.connection) {
            sqlite.exec("PRAGMA application_id = 0x17895E7D;");
            sqlite.exec("PRAGMA user_version = 1;");
        }
        return sqlite;
    }

    static Connection open(const char* filename, int flags = 0) {
        flags |= SQLITE_OPEN_READWRITE | SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_EXRESCODE;
        Connection sqlite;
        int rc = sqlite3_open_v2(filename, &sqlite.connection, flags, nullptr);
        if (rc == SQLITE_OK) {
            sqlite.exec("PRAGMA locking_mode = EXCLUSIVE;");
            sqlite.exec("PRAGMA fullfsync = TRUE;");
            sqlite.exec("PRAGMA synchronous = FULL;");
            sqlite.exec("PRAGMA journal_mode = DELETE;");
            return sqlite;
        } else {
            std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
            return {};
        }
    }

    static Connection memory() {
        return create(":memory:");
    }
};
}

#endif
