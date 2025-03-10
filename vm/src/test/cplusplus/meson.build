#
# Copyright (c) 2024-2025 Pavel Vasin
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
    vm_test_arguments = ['--log_format=JUNIT', '--log_sink=TEST-vm.xml']
else
    vm_test_arguments = []
endif

vm_test_sources = [
    'cell.cpp',
    'test.cpp',
]
vm_test_includes = [crypto_test_includes, vm_main_includes]
vm_test_dependencies = [vm_main_dependencies, boost_test_dependency]
vm_test_bin = executable(
    'vm_test',
    vm_test_sources,
    implicit_include_directories : false,
    include_directories : vm_test_includes,
    dependencies : vm_test_dependencies
)
test(
    'vm test',
    vm_test_bin,
    args : vm_test_arguments,
)
