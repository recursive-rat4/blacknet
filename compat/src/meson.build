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
    #include <version>
    int main() {
        static_assert(__cpp_static_assert >= 202306L);
        static_assert(__cpp_implicit_move >= 202207L);
        static_assert(__cpp_multidimensional_subscript >= 202110L);
        static_assert(__cpp_concepts >= 201907L);
        static_assert(__cpp_impl_three_way_comparison >= 201907L);
        static_assert(__cpp_impl_coroutine >= 201902L);
        static_assert(__cpp_consteval >= 201811L);
        static_assert(__cpp_guaranteed_copy_elision >= 201606L);
        static_assert(__cpp_if_constexpr >= 201606L);
        static_assert(__cpp_structured_bindings >= 201606L);
        static_assert(__cpp_fold_expressions >= 201603L);
        static_assert(__cpp_range_based_for >= 201603L);
        static_assert(__cpp_constexpr >= 201304L);
        static_assert(__cpp_decltype_auto >= 201304L);
        static_assert(__cpp_lambdas >= 200907L);
        static_assert(__cpp_attributes >= 200809L);
        static_assert(__cpp_user_defined_literals >= 200809L);
        static_assert(__cpp_initializer_lists >= 200806L);
        static_assert(__cpp_threadsafe_static_init >= 200806L);
        static_assert(__cpp_ref_qualifiers >= 200710L);
        static_assert(__cpp_decltype >= 200707L);
        static_assert(__cpp_alias_templates >= 200704L);
        static_assert(__cpp_unicode_characters >= 200704L);
        static_assert(__cpp_variadic_templates >= 200704L);
        static_assert(__cpp_rvalue_references >= 200610L);

        static_assert(__has_cpp_attribute(assume) >= 202207L);
        static_assert(__has_cpp_attribute(nodiscard) >= 201907L);
        static_assert(__has_cpp_attribute(maybe_unused) >= 201603L);
        static_assert(__has_cpp_attribute(noreturn) >= 200809L);

        #if not __GNUG__
        static_assert(__cpp_lib_constexpr_cmath >= 202306L);
        #endif
        static_assert(__cpp_lib_fstream_native_handle >= 202306L);
        static_assert(__cpp_lib_constexpr_charconv >= 202207L);
        static_assert(__cpp_lib_generator >= 202207L);
        static_assert(__cpp_lib_ios_noreplace >= 202207L);
        static_assert(__cpp_lib_concepts >= 202002L);
        static_assert(__cpp_lib_int_pow2 >= 202002L);
        static_assert(__cpp_lib_span >= 202002L);
        static_assert(__cpp_lib_ranges >= 201911L);
        static_assert(__cpp_lib_bitops >= 201907L);
        static_assert(__cpp_lib_chrono >= 201907L);
        static_assert(__cpp_lib_constexpr_string >= 201907L);
        static_assert(__cpp_lib_constexpr_vector >= 201907L);
        static_assert(__cpp_lib_endian >= 201907L);
        static_assert(__cpp_lib_array_constexpr >= 201811L);
        static_assert(__cpp_lib_constexpr_algorithms >= 201806L);
        static_assert(__cpp_lib_atomic_shared_ptr >= 201711L);
        static_assert(__cpp_lib_starts_ends_with >= 201711L);
        static_assert(__cpp_lib_filesystem >= 201703L);
        static_assert(__cpp_lib_optional >= 201606L);
        static_assert(__cpp_lib_string_view >= 201606L);
        static_assert(__cpp_lib_byte >= 201603L);
        static_assert(__cpp_lib_make_unique >= 201304L);
    }'''.replace('__GNUG__', compiler.get_id() == 'gcc' ? '1' : '0'), # Thou shalt not pass
    name : 'C++',
    required : true
)

subdir('main/cplusplus')

if get_option('tests')
    subdir('test/cplusplus')
endif
