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

/// Commitment scheme is a cryptographic protocol that binds to a message and hides it.
pub trait CommitmentScheme<Message> {
    /// Result type.
    type Commitment;
    /// Type to open commitment.
    type Opening;

    /// Commit to a message.
    fn commit(&self, message: &Message) -> (Self::Commitment, Self::Opening);

    /// Open commitment.
    fn open(
        &self,
        commitment: &Self::Commitment,
        message: &Message,
        opening: &Self::Opening,
    ) -> bool;
}
