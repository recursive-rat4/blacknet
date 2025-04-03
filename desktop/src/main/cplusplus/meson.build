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

desktop_main_includes = [wallet_main_includes, config_includes, include_directories('.')]
desktop_main_dependencies = [wallet_main_dependencies, qt_dependency]
desktop_main_sources = [
    'desktop.cpp',
]
desktop_main_bin = executable(
    'blacknet-desktop',
    [desktop_main_sources, desktop_main_resources, desktop_main_ui],
    implicit_include_directories : false,
    include_directories : desktop_main_includes,
    install : true,
    dependencies : desktop_main_dependencies,
)
