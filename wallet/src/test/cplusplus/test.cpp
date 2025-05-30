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

#define BOOST_TEST_MODULE wallet
#include <boost/test/unit_test.hpp>

#include "logmanager.h"
#include "mode.h"
#include "sqlite.h"

using blacknet::compat::ModeManager;
using blacknet::log::LogManager;
using blacknet::wallet::sqlite::SQLite;

struct WalletGlobalFixture {
    ModeManager _;
    LogManager _{LogManager::Regime::UnitTest};
    SQLite _;
};

BOOST_TEST_GLOBAL_FIXTURE(WalletGlobalFixture);
