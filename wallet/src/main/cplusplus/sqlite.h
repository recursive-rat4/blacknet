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
#include <exception>
#include <generator>
#include <span>
#include <string>
#include <string_view>
#include <utility>
#include <sqlite3.h>

#include "logger.h"

namespace blacknet::wallet {

namespace sqlite {
constexpr log::Logger& logger() {
    static log::Logger instance;
    return instance;
}

class Exception : public std::exception {
    std::string message;
public:
    Exception(int rc) : message(sqlite3_errstr(rc)) {}
    Exception(const std::string& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

template<typename Fun, typename... Args>
void ok(const Fun& fun, Args&&... args) {
    int rc = fun(std::forward<Args>(args)...);
    if (rc == SQLITE_OK)
        return;
    throw Exception(rc);
}

template<typename Fun, typename... Args>
void pass(const Fun& fun, Args&&... args) {
    int rc = fun(std::forward<Args>(args)...);
    if (rc == SQLITE_OK)
        return;
    logger()->error("{}", sqlite3_errstr(rc));
}

class Binder {
    friend class Statement;
    sqlite3_stmt* const statement;

    constexpr Binder(sqlite3_stmt* const statement) : statement(statement) {}
public:
    constexpr Binder() = delete;
    constexpr Binder(const Binder&) = delete;
    constexpr Binder(Binder&&) = delete;
    ~Binder() noexcept {
        pass(sqlite3_clear_bindings, statement);
    }

    constexpr Binder& operator = (const Binder&) = delete;
    constexpr Binder& operator = (Binder&&) = delete;

    void blob(int column, const std::span<const std::byte>& value) {
        ok(sqlite3_bind_blob, statement, column, value.data(), value.size(), SQLITE_TRANSIENT);
    }

    void real(int column, double value) {
        ok(sqlite3_bind_double, statement, column, value);
    }

    void integer(int column, int64_t value) {
        ok(sqlite3_bind_int64, statement, column, value);
    }

    void null(int column) {
        ok(sqlite3_bind_null, statement, column);
    }

    void text(int column, const std::string_view& value) {
        ok(sqlite3_bind_text, statement, column, value.data(), value.size(), SQLITE_TRANSIENT);
    }
};

class Row {
    friend class Evaluator;
    sqlite3_stmt* const statement;

    constexpr Row(sqlite3_stmt* const statement) : statement(statement) {}
public:
    constexpr Row() = delete;
    constexpr Row(const Row&) = delete;
    constexpr Row(Row&&) = delete;

    constexpr Row& operator = (const Row&) = delete;
    constexpr Row& operator = (Row&&) = delete;

    int columns() {
        return sqlite3_column_count(statement);
    }

    std::span<const std::byte> blob(int column) {
        return {
            reinterpret_cast<const std::byte*>(sqlite3_column_blob(statement, column)),
            static_cast<std::size_t>(sqlite3_column_bytes(statement, column))
        };
    }

    double real(int column) {
        return sqlite3_column_double(statement, column);
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

class Evaluator {
    friend class Statement;
    friend class Connection;
    sqlite3_stmt* const statement;

    constexpr Evaluator(sqlite3_stmt* const statement) : statement(statement) {}
public:
    constexpr Evaluator() = delete;
    constexpr Evaluator(const Evaluator&) = delete;
    constexpr Evaluator(Evaluator&&) = delete;
    ~Evaluator() noexcept {
        pass(sqlite3_reset, statement);
    }

    constexpr Evaluator& operator = (const Evaluator&) = delete;
    constexpr Evaluator& operator = (Evaluator&&) = delete;

    Row row() {
        return statement;
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
    ~Statement() noexcept {
        pass(sqlite3_finalize, statement);
    }

    constexpr Statement& operator = (const Statement&) = delete;
    constexpr Statement& operator = (Statement&& other) noexcept {
        std::swap(statement, other.statement);
        return *this;
    }

    constexpr bool isPrepared() const {
        return statement != nullptr;
    }

    Binder binder() {
        if (statement)
            return statement;
        else
            throw Exception("SQLite statement is not prepared");
    }

    std::generator<Row> evaluate() {
        if (statement) {
            Evaluator evaluator(statement);
            while (true) {
                int rc = sqlite3_step(statement);
                if (rc == SQLITE_ROW) {
                    co_yield evaluator.row();
                } else if (rc == SQLITE_DONE) {
                    co_return;
                } else {
                    throw Exception(rc);
                }
            }
        } else {
            throw Exception("SQLite statement is not prepared");
        }
    }

    void execute() {
        if (statement) {
            Evaluator evaluator(statement);
            while (true) {
                int rc = sqlite3_step(statement);
                if (rc == SQLITE_ROW) {
                    continue;
                } else if (rc == SQLITE_DONE) {
                    break;
                } else {
                    throw Exception(rc);
                }
            }
        } else {
            throw Exception("SQLite statement is not prepared");
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
    ~Connection() noexcept {
        pass(sqlite3_close, connection);
    }

    constexpr Connection& operator = (const Connection&) = delete;
    constexpr Connection& operator = (Connection&& other) noexcept {
        std::swap(connection, other.connection);
        return *this;
    }

    constexpr bool isConnected() const {
        return connection != nullptr;
    }

    std::generator<Row> evaluate(const char* query) {
        Statement sqlite(prepareImpl(query));
        Evaluator evaluator(sqlite.statement);
        while (true) {
            int rc = sqlite3_step(sqlite.statement);
            if (rc == SQLITE_ROW) {
                co_yield evaluator.row();
            } else if (rc == SQLITE_DONE) {
                co_return;
            } else {
                throw Exception(rc);
            }
        }
    }

    void execute(const char* query) {
        if (connection) {
            ok(sqlite3_exec, connection, query, nullptr, nullptr, nullptr);
        } else {
            throw Exception("SQLite is not connected");
        }
    }

    Statement prepare(const char* query) {
        return prepareImpl(query, SQLITE_PREPARE_PERSISTENT);
    }

    static Connection create(const char* filename) {
        return openImpl(filename, SQLITE_OPEN_CREATE);
    }

    static Connection open(const char* filename) {
        return openImpl(filename);
    }

    static Connection memory() {
        return create(":memory:");
    }
private:
    static Connection openImpl(const char* filename, int flags = 0) {
        flags |= SQLITE_OPEN_READWRITE | SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_EXRESCODE;
        Connection sqlite;
        ok(sqlite3_open_v2, filename, &sqlite.connection, flags, nullptr);
        return sqlite;
    }

    Statement prepareImpl(const char* query, int flags = 0) {
        if (connection) {
            Statement sqlite;
            ok(sqlite3_prepare_v3, connection, query, -1, flags, &sqlite.statement, nullptr);
            return sqlite;
        } else {
            throw Exception("SQLite is not connected");
        }
    }
};

class SQLite {
public:
    SQLite() {
        ok(sqlite3_initialize);
        logger() = log::Logger("SQLite");
        logger()->info("Driving SQLite {}", sqlite3_libversion());
    }
    ~SQLite() noexcept {
        logger()->info("Braking SQLite");
        pass(sqlite3_shutdown);
        logger().reset();
    }

    constexpr SQLite(const SQLite&) = delete;
    constexpr SQLite(SQLite&&) = delete;
    constexpr SQLite& operator = (const SQLite&) = delete;
    constexpr SQLite& operator = (SQLite&&) = delete;
};
}

}

#endif
