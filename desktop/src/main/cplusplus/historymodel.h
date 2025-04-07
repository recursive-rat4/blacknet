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

#ifndef BLACKNET_DESKTOP_HISTORYMODEL_H
#define BLACKNET_DESKTOP_HISTORYMODEL_H

#include <array>
#include <QAbstractTableModel>

#include "logger.h"

namespace blacknet::desktop {

class HistoryModel : public QAbstractTableModel {
    mutable log::Logger logger{"HistoryModel"};
    constexpr static const std::array header{
        "#", "Date", "Type", "Amount", "Fingerprint"
    };
public:
    HistoryModel(QObject* parent = nullptr)
        : QAbstractTableModel(parent)
    {
    }

    QVariant headerData(int section, Qt::Orientation orientation, int role) const override {
        if (role != Qt::DisplayRole)
            return {};

        if (orientation != Qt::Horizontal)
            return {};

        return header[section];
    }

    int columnCount(const QModelIndex& parent) const override {
        if (parent.isValid())
            return 0;
        else
            return header.size();
    }

    int rowCount(const QModelIndex& parent) const override {
        if (parent.isValid())
            return 0;
        else
            return 0; //TODO
    }

    QVariant data(const QModelIndex& index, int role) const override {
        if (role != Qt::DisplayRole)
            return {};

        //TODO

        logger->error("QModelIndex({}, {}) not in table", index.row(), index.column());
        return {};
    }
};

}

#endif
