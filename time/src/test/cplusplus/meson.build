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
    time_test_arguments = ['--log_format=JUNIT', '--log_sink=TEST-time.xml']
else
    time_test_arguments = []
endif

time_test_sources = [
    'milliseconds.cpp',
    'systemclock.cpp',
    'test.cpp',
]
time_test_includes = time_main_includes
time_test_dependencies = [time_main_dependencies, boost_test_dependency]
time_test_bin = executable(
    'time_test',
    time_test_sources,
    implicit_include_directories : false,
    include_directories : time_test_includes,
    dependencies : time_test_dependencies
)
test(
    'time test',
    time_test_bin,
    args : time_test_arguments,
)
