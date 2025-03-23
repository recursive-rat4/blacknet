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

if get_option('tests_report_junit')
    wallet_test_arguments = ['--log_format=JUNIT', '--log_sink=TEST-wallet.xml']
else
    wallet_test_arguments = []
endif

wallet_test_sources = [
    'sqlite.cpp',
    'test.cpp',
    'wallet.cpp',
]
wallet_test_includes = wallet_main_includes
wallet_test_dependencies = [wallet_main_dependencies, boost_test_dependency]
wallet_test_bin = executable(
    'wallet_test',
    wallet_test_sources,
    implicit_include_directories : false,
    include_directories : wallet_test_includes,
    dependencies : wallet_test_dependencies
)
test(
    'wallet test',
    wallet_test_bin,
    args : wallet_test_arguments,
)
