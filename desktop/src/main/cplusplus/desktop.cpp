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
#include <boost/asio/thread_pool.hpp>
#include <fmt/format.h>
#include <fmt/std.h>
#include <QApplication>
#include <QIcon>
#include <QMessageBox>

#include "mainwindow.h"
#include "node.h"
#include "settings.h"
#include "trayicon.h"

using namespace blacknet::desktop;
using blacknet::log::LogManager;
using blacknet::network::Node;

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
        Node node(LogManager::Regime::Desktop);
        boost::asio::thread_pool thread_pool;
        node.co_spawn(thread_pool);

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
