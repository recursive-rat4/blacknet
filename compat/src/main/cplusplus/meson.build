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

compiler.compiles('''
        #include <climits>
        int main() { static_assert(CHAR_BIT == 8); }
    ''',
    name : 'byte is 8 bits',
    required : true
)

compat_main_includes = [
    config_includes,
    include_directories('.')
]
compat_main_dependencies = [
    fmt_dependency,
]
