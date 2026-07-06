/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::branchless::BlSelect;

/// Absolute value of signed int.
pub trait BlAbs {
    /// Result type as unsigned int.
    type UnsignedAbs;

    /// The absolute value.
    /// # Panics
    /// In debug mode, on overflow.
    fn bl_abs(self) -> Self;

    /// The absolute value as unsigned int.
    fn bl_unsigned_abs(self) -> Self::UnsignedAbs;

    /// The absolute value wrapping on overflow.
    fn bl_wrapping_abs(self) -> Self;
}

macro_rules! impl_abs {
    ( $($x:ty, $y:ty),+ ) => {
        $(
            impl BlAbs for $x {
                type UnsignedAbs = $y;

                fn bl_abs(self) -> $x {
                    self.bl_select(-self, self < 0)
                }

                fn bl_unsigned_abs(self) -> $y {
                    self.bl_wrapping_abs() as $y
                }

                fn bl_wrapping_abs(self) -> $x {
                    self.bl_select(self.wrapping_neg(), self < 0)
                }
            }
        )+
    };
}

impl_abs!(i8, u8, i16, u16, i32, u32, i64, u64);
