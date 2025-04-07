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

#ifndef BLACKNET_DESKTOP_TRAYICON_H
#define BLACKNET_DESKTOP_TRAYICON_H

#include <QIcon>
#include <QMenu>
#include <QSystemTrayIcon>

#include "ui_traymenu.h"

namespace blacknet::desktop {

class TrayIcon : public QSystemTrayIcon {
public:
    QWidget* mainWindow;
    QMenu menu;
    Ui::TrayMenu ui;

    TrayIcon(QCoreApplication* application, QWidget* mainWindow, QObject* parent = nullptr)
        : QSystemTrayIcon(parent), mainWindow(mainWindow)
    {
        ui.setupUi(&menu);
#if QT_VERSION >= QT_VERSION_CHECK(6, 7, 0)
        ui.actionQuit->setIcon(QIcon::fromTheme(QIcon::ThemeIcon::ApplicationExit));
#endif
        connect(ui.actionQuit, &QAction::triggered, application, &QCoreApplication::quit);

        connect(this, &QSystemTrayIcon::activated, this, &TrayIcon::activated);

        setContextMenu(&menu);
        setIcon(QIcon(":/blacknet/resources/logo.png"));
        setToolTip("Blacknet wallet");
        setVisible(true);
    }
private:
    void activated(ActivationReason reason) {
        if (reason == Trigger) {
            if (mainWindow->isVisible())
                mainWindow->setVisible(false);
            else
                mainWindow->showNormal();
        }
    }
};

}

#endif
