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

#include <utility>
#include <sqlite3.h>

class SQLite {
    sqlite3* connection;
public:
    consteval SQLite() : connection(nullptr) {}
    constexpr SQLite(const SQLite&) = delete;
    constexpr SQLite(SQLite&& other) noexcept : connection(other.connection) {
        other.connection = nullptr;
    }
    ~SQLite() {
        int rc = sqlite3_close(connection);
        if (rc == SQLITE_OK)
            return;
        std::cerr << "SQLite: " << sqlite3_errstr(rc) << std::endl;
    }

    constexpr SQLite& operator = (const SQLite&) = delete;
    constexpr SQLite& operator = (SQLite&& other) noexcept {
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

    static SQLite create(const char* filename) {
        SQLite sqlite(open(filename, SQLITE_OPEN_CREATE));
        if (sqlite.connection) {
            sqlite.exec("PRAGMA application_id = 0x17895E7D;");
            sqlite.exec("PRAGMA user_version = 1;");
        }
        return sqlite;
    }

    static SQLite open(const char* filename, int flags = 0) {
        flags |= SQLITE_OPEN_READWRITE | SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_EXRESCODE;
        SQLite sqlite;
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

    static SQLite memory() {
        return create(":memory:");
    }
};

#endif
