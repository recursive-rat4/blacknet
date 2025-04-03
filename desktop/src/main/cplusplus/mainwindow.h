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

#ifndef BLACKNET_DESKTOP_MAINWINDOW_H
#define BLACKNET_DESKTOP_MAINWINDOW_H

#include <QCoreApplication>
#include <QMainWindow>

#include "historymodel.h"
#include "preferencesdialog.h"
#include "settings.h"

#include "ui_mainwindow.h"
#include "ui_dashboardpage.h"
#include "ui_transferpage.h"
#include "ui_atomicswappage.h"
#include "ui_historypage.h"
#include "ui_leasingpage.h"
#include "ui_stakingpage.h"
#include "ui_addressbookpage.h"

class MainWindow : public QMainWindow {
public:
    QCoreApplication* application;
    Settings* settings;
    Ui::MainWindow mainWindow;
    Ui::DashboardPage dashboard;
    Ui::TransferPage transfer;
    Ui::AtomicSwapPage atomicSwap;
    Ui::HistoryPage history;
    Ui::LeasingPage leasing;
    Ui::StakingPage staking;
    Ui::AddressBookPage addressBook;

    MainWindow(QCoreApplication* application, Settings* settings, QWidget* parent = nullptr)
        : QMainWindow(parent), application(application), settings(settings)
    {
        mainWindow.setupUi(this);
#if QT_VERSION >= QT_VERSION_CHECK(6, 7, 0)
        mainWindow.actionQuit->setIcon(QIcon::fromTheme(QIcon::ThemeIcon::ApplicationExit));
#endif
        connect(mainWindow.actionQuit, &QAction::triggered, application, &QCoreApplication::quit);
        connect(mainWindow.actionPreferences, &QAction::triggered, this, &MainWindow::preferences);

        dashboard.setupUi(mainWindow.pageDashboard);

        transfer.setupUi(mainWindow.pageTransfer);

        atomicSwap.setupUi(mainWindow.pageAtomicSwap);

        history.setupUi(mainWindow.pageHistory);
        HistoryModel* historyModel = new HistoryModel(mainWindow.pageHistory);
        mainWindow.pageHistory->setModel(historyModel);

        leasing.setupUi(mainWindow.pageLeasing);

        staking.setupUi(mainWindow.pageStaking);

        addressBook.setupUi(mainWindow.pageAddressBook);

        setVisible(true);
    }
private:
    void changeEvent(QEvent* event) override {
        QMainWindow::changeEvent(event);
        if (event->type() == QEvent::WindowStateChange) {
            if (settings->hideOnMinimize && isMinimized())
                setVisible(false);
        }
    }
    void closeEvent(QCloseEvent* event) override {
        if (settings->hideOnClose)
            QMainWindow::closeEvent(event);
        else
            application->quit();
    }

    void preferences() {
        PreferencesDialog(settings).exec();
    }
};

#endif
