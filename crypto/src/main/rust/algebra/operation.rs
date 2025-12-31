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

/// The addition of an object to itself.
pub trait Double {
    /// Result type.
    type Output;

    /// Perform the operation equivalent to `self + self`.
    fn double(self) -> Self::Output;
}

/// The multiplication of an object by itself.
pub trait Square {
    /// Result type.
    type Output;

    /// Perform the operation equivalent to `self * self`.
    fn square(self) -> Self::Output;
}

/// The unary inversion operator `/`.
pub trait Inv {
    /// Result type.
    type Output;

    /// Perform the unary `/` operation.
    fn inv(self) -> Self::Output;
}
