#
# Copyright (c) 2025 Pavel Vasin
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
#

qt = import('qt6')

desktop_main_resources = qt.compile_resources(
    sources : 'resources.qrc',
    extra_args : '-no-compress',
)

desktop_main_ui = qt.compile_ui(
    sources : [
        'resources/addressbookpage.ui',
        'resources/atomicswappage.ui',
        'resources/dashboardpage.ui',
        'resources/historypage.ui',
        'resources/leasingpage.ui',
        'resources/mainwindow.ui',
        'resources/preferencesdialog.ui',
        'resources/stakingpage.ui',
        'resources/transferpage.ui',
        'resources/traymenu.ui',
    ],
)

subdir('cplusplus')
