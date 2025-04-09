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

#ifndef BLACKNET_COMPAT_MODE_H
#define BLACKNET_COMPAT_MODE_H

#include <cstdlib>
#include <exception>
#include <filesystem>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <fmt/format.h>

#include "magic.h"

namespace blacknet::compat {

/**
 * An abstract mode of operation: production or various research, development, testing.
 */
class Mode {
protected:
    /**
     * The position in enumeration.
     */
    virtual int ordinal() const noexcept = 0;
    /**
     * An agent suffix for network indication.
     */
    virtual std::string_view agent_suffix() const noexcept = 0;
public:
    /**
     * A subdirectory to separate data.
     */
    virtual std::optional<std::filesystem::path> subdirectory() const noexcept = 0;
    /**
     * An address prefix to designate a different network.
     */
    virtual std::string_view address_prefix() const noexcept = 0;
    /**
     * Whether the node requires network peers.
     */
    virtual bool requires_network() const noexcept = 0;

    std::string agent_name() const noexcept {
        return fmt::format("{}{}", compat::agent_name(), agent_suffix());
    }

    uint16_t default_p2p_port() const noexcept {
        return compat::default_p2p_port() + ordinal();
    }

    uint32_t network_magic() const noexcept {
        return compat::network_magic() + ordinal();
    }
};

/**
 * The main network. It's the production mode.
 */
class MainNet : public Mode {
protected:
    virtual int ordinal() const noexcept override {
        return 0;
    }
    virtual std::string_view agent_suffix() const noexcept override {
        return {};
    }
public:
    virtual std::optional<std::filesystem::path> subdirectory() const noexcept override {
        return std::nullopt;
    }
    virtual std::string_view address_prefix() const noexcept override {
        return {};
    }
    virtual bool requires_network() const noexcept override {
        return true;
    }
};

class TestNet : public Mode {
protected:
    virtual int ordinal() const noexcept override {
        return 1;
    }
    virtual std::string_view agent_suffix() const noexcept override {
        return "-TestNet";
    }
public:
    virtual std::optional<std::filesystem::path> subdirectory() const noexcept override {
        return "TestNet";
    }
    virtual std::string_view address_prefix() const noexcept override {
        return "t";
    }
    virtual bool requires_network() const noexcept override {
        return true;
    }
};

/**
 * A regression testing mode. Usually it's a sole offline node,
 * or else it can be a tiny private network.
 */
class RegTest : public Mode {
protected:
    virtual int ordinal() const noexcept override {
        return 3;
    }
    virtual std::string_view agent_suffix() const noexcept override {
        return "-RegTest";
    }
public:
    virtual std::optional<std::filesystem::path> subdirectory() const noexcept override {
        return "RegTest";
    }
    virtual std::string_view address_prefix() const noexcept override {
        return "r";
    }
    virtual bool requires_network() const noexcept override {
        return false;
    }
};

using DefaultMode = MainNet;

/**
 * A `Mode` the program is running in. By default, it's `MainNet`.
 */
constexpr std::shared_ptr<Mode>& mode() {
    static std::shared_ptr<Mode> instance;
    return instance;
}

class ModeManager {
public:
    ModeManager() {
        const char* ptr = std::getenv("BLACKNET_MODE");
        if (ptr == nullptr) {
            mode() = std::make_shared<DefaultMode>();
        } else {
            std::string_view str(ptr);
            if (str == "MainNet")
                mode() = std::make_shared<MainNet>();
            else if (str == "TestNet")
                throw std::runtime_error("TestNet was not tested");
            else if (str == "RegTest")
                mode() = std::make_shared<RegTest>();
            else
                throw std::runtime_error(
                    fmt::format("Unrecognized mode: {}. Possible values: MainNet, RegTest.", str));
        }
    }
    ~ModeManager() noexcept {
        mode().reset();
    }

    constexpr ModeManager(const ModeManager&) = delete;
    constexpr ModeManager(ModeManager&&) = delete;
    constexpr ModeManager& operator = (const ModeManager&) = delete;
    constexpr ModeManager& operator = (ModeManager&&) = delete;
};

}

#endif
