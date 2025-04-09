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

#include "blacknet-config.h"

#include <exception>
#include <filesystem>
#include <thread>
#include <fmt/format.h>
#include <fmt/std.h>
#include <QApplication>
#include <QIcon>
#include <QMessageBox>

#include "getuid.h"
#include "logmanager.h"
#include "settings.h"
#include "trayicon.h"
#include "mainwindow.h"
#include "mode.h"
#include "uname.h"
#include "xdgdirectories.h"

using namespace blacknet;
using namespace blacknet::desktop;
using blacknet::compat::DirManager;
using blacknet::compat::ModeManager;
using blacknet::log::Logger;
using blacknet::log::LogManager;

class Desktop : public QApplication {
public:
    Desktop(int& argc, char* argv[]) : QApplication(argc, argv) {
        setApplicationName("Blacknet Desktop");
        setApplicationVersion(BLACKNET_VERSION_STRING);
        setOrganizationDomain("blacknet.ninja");
        setOrganizationName("Blacknet");
        setApplicationDisplayName("Blacknet");
        setDesktopFileName("blacknet-desktop");
        setQuitOnLastWindowClosed(false);
        setWindowIcon(QIcon(":/blacknet/resources/logo.png"));
    }
};

int main(int argc, char* argv[]) {
    Desktop desktop(argc, argv);
    try {
        ModeManager modeManager;
        DirManager dirManager;
        LogManager logManager(LogManager::Regime::Desktop);

        auto [os_name, os_version, os_machine] = compat::uname();

        Logger logger("main");
        logger->info("Starting up {} node {}", compat::mode()->agent_name(), BLACKNET_VERSION_STRING);
        logger->info("CPU: {} cores {}", std::thread::hardware_concurrency(), os_machine);
        logger->info("OS: {} version {}", os_name, os_version);
        logger->info("Using config directory {}", std::filesystem::absolute(compat::configDir()));
        logger->info("Using data directory {}", std::filesystem::absolute(compat::dataDir()));
        logger->info("Using state directory {}", std::filesystem::absolute(compat::stateDir()));

        if (compat::getuid() == 0)
            logger->warn("Running as root");
#if 0
        if (compat::getsid() == "S-1-5-18")
            logger->warn("Running as SYSTEM");
#endif

        Settings settings;
        MainWindow mainWindow(&desktop, &settings);
        TrayIcon trayIcon(&desktop, &mainWindow);
        return desktop.exec();
    } catch (const std::exception& e) {
#if FMT_VERSION >= 100000
        auto message = fmt::format("{:t}", e);
        QMessageBox::critical(nullptr, "Error", QString::fromStdString(message));
#else
        QMessageBox::critical(nullptr, "Error", e.what());
#endif
        return 1;
    }
}
