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

#ifndef BLACKNET_DESKTOP_PREFERENCESDIALOG_H
#define BLACKNET_DESKTOP_PREFERENCESDIALOG_H


#include <QDialog>

#include "settings.h"

#include "ui_preferencesdialog.h"

class PreferencesDialog : public QDialog {
public:
    Settings* settings;
    Ui::PreferencesDialog ui;

    PreferencesDialog(Settings* settings, QWidget* parent = nullptr)
        : QDialog(parent), settings(settings)
    {
        ui.setupUi(this);
        init();
    }
private:
    void closeEvent(QCloseEvent* event) override {
        apply();
        QDialog::closeEvent(event);
    }

    void init() {
        ui.checkBoxHideOnClose->setCheckState(settings->hideOnClose ? Qt::Checked : Qt::Unchecked);
        ui.checkBoxHideOnMinimize->setCheckState(settings->hideOnMinimize ? Qt::Checked : Qt::Unchecked);
    }
    void apply() {
        settings->hideOnClose = ui.checkBoxHideOnClose->checkState() == Qt::Checked;
        settings->hideOnMinimize = ui.checkBoxHideOnMinimize->checkState() == Qt::Checked;
    }
};

#endif
