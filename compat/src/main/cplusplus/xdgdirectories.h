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

#ifndef BLACKNET_COMPAT_XDGDIRECTORIES_H
#define BLACKNET_COMPAT_XDGDIRECTORIES_H

#include "blacknet-config.h"

#include <cstdlib>
#include <filesystem>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string_view>

#include "magic.h"
#include "mkdirs.h"
#include "mode.h"

namespace blacknet::compat {

constexpr std::filesystem::path& configDir() {
    static std::filesystem::path instance;
    return instance;
}

constexpr std::filesystem::path& dataDir() {
    static std::filesystem::path instance;
    return instance;
}

constexpr std::filesystem::path& stateDir() {
    static std::filesystem::path instance;
    return instance;
}

class DirManager {
    static std::optional<std::filesystem::path> get_optional(const char* var) {
        const char* ptr = std::getenv(var);
        if (ptr == nullptr)
            return std::nullopt;
        else
            return ptr;
    }
    static std::filesystem::path get_or_throw(const char* var) {
        const char* ptr = std::getenv(var);
        if (ptr != nullptr)
            return ptr;
        else
            throw std::runtime_error(fmt::format("Environment variable {} is not set", var));
    }

    class Strategy {
    public:
        virtual ~Strategy() noexcept = default;
        virtual std::filesystem::path configDir() = 0;
        virtual std::filesystem::path dataDir() = 0;
        virtual std::filesystem::path stateDir() = 0;
    };

    class WindowsStrategy : public Strategy {
        static std::filesystem::path userprofile() {
            return get_or_throw("USERPROFILE");
        }
    public:
        WindowsStrategy() noexcept = default;
        virtual std::filesystem::path configDir() override {
            return userprofile() / "AppData" / "Local" / xdg_subdirectory();
        }
        virtual std::filesystem::path dataDir() override {
            return userprofile() / "AppData" / "Local" / xdg_subdirectory();
        }
        virtual std::filesystem::path stateDir() override {
            return userprofile() / "AppData" / "Local" / xdg_subdirectory();
        }
    };

    class DarwinStrategy : public Strategy {
        static std::filesystem::path home() {
            return get_or_throw("HOME");
        }
    public:
        DarwinStrategy() noexcept = default;
        virtual std::filesystem::path configDir() override {
            return home() / "Library" / "Application Support" / xdg_subdirectory();
        }
        virtual std::filesystem::path dataDir() override {
            return home() / "Library" / "Application Support" / xdg_subdirectory();
        }
        virtual std::filesystem::path stateDir() override {
            return home() / "Library" / "Application Support" / xdg_subdirectory();
        }
    };

    // https://specifications.freedesktop.org/basedir-spec/0.8/
    class XDGStrategy : public Strategy {
        static std::filesystem::path home() {
            return get_or_throw("HOME");
        }
    public:
        XDGStrategy() noexcept = default;
        virtual std::filesystem::path configDir() override {
            auto base = get_optional("XDG_CONFIG_HOME");
            if (base && base->is_absolute())
                return (*base) / xdg_subdirectory();
            else
                return home() / ".config" / xdg_subdirectory();
        }
        virtual std::filesystem::path dataDir() override {
            auto base = get_optional("XDG_DATA_HOME");
            if (base && base->is_absolute())
                return (*base) / xdg_subdirectory();
            else
                return home() / ".local" / "share" / xdg_subdirectory();
        }
        virtual std::filesystem::path stateDir() override {
            auto base = get_optional("XDG_STATE_HOME");
            if (base && base->is_absolute())
                return (*base) / xdg_subdirectory();
            else
                return home() / ".local" / "state" / xdg_subdirectory();
        }
    };
public:
    DirManager() {
        std::unique_ptr<Strategy> strategy;
        constexpr std::string_view os(BLACKNET_HOST_SYSTEM);
        if constexpr (os == "windows")
            strategy = std::make_unique<WindowsStrategy>();
        else if constexpr (os == "darwin")
            strategy = std::make_unique<DarwinStrategy>();
        else
            strategy = std::make_unique<XDGStrategy>();

        if (auto dir = get_optional("BLACKNET_CONFIGDIR"))
            configDir() = *dir;
        else
            configDir() = strategy->configDir();

        if (auto dir = get_optional("BLACKNET_DATADIR"))
            dataDir() = *dir;
        else
            dataDir() = strategy->dataDir();

        if (auto dir = get_optional("BLACKNET_STATEDIR"))
            stateDir() = *dir;
        else
            stateDir() = strategy->stateDir();

        if (auto subdir = mode()->subdirectory()) {
            configDir() /= *subdir;
            dataDir() /= *subdir;
            stateDir() /= *subdir;
        }

        mkdirs(configDir(), std::filesystem::perms::owner_all);
        mkdirs(dataDir(), std::filesystem::perms::owner_all);
        mkdirs(stateDir(), std::filesystem::perms::owner_all);
    }
    ~DirManager() noexcept {
        configDir().clear();
        dataDir().clear();
        stateDir().clear();
    }

    constexpr DirManager(const DirManager&) = delete;
    constexpr DirManager(DirManager&&) = delete;
    constexpr DirManager& operator = (const DirManager&) = delete;
    constexpr DirManager& operator = (DirManager&&) = delete;
};

}

#endif
