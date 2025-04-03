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

#include <QApplication>
#include <QIcon>

#include "settings.h"
#include "trayicon.h"
#include "mainwindow.h"

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
    Settings settings;
    MainWindow mainWindow(&desktop, &settings);
    TrayIcon trayIcon(&desktop, &mainWindow);
    return desktop.exec();
}
