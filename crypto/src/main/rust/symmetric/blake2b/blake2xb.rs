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

use crate::random::UniformGenerator;
use crate::symmetric::blake2b::Blake2b;
use zeroize::Zeroize;

/// BLAKE2Xb extensible-output function.
#[derive(Clone, Copy, Zeroize)]
pub struct Blake2xb {
    state: Blake2b<64>,
    xof_length: u32,
}

impl Blake2xb {
    /// Construct new hasher.
    ///
    /// The digest length is set to the maximum.
    pub const fn new() -> Self {
        Self::with_length(u32::MAX)
    }

    /// Construct new hasher with digest length.
    pub const fn with_length(xof_length: u32) -> Self {
        Self {
            state: Blake2b::with_params(1, 1, 0, 0, xof_length, 0, [0; 16]),
            xof_length,
        }
    }

    /// Process input data.
    #[inline]
    pub fn update(&mut self, input: impl AsRef<[u8]>) {
        self.state.update_impl(input.as_ref())
    }

    /// Produce XOF output.
    pub fn finalize(self) -> XOFOutput {
        XOFOutput {
            buffer: [0; 64],
            position: 64,
            h0: self.state.finalize(),
            node_offset: 0,
            xof_length: self.xof_length,
        }
    }
}

impl Default for Blake2xb {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Zeroize)]
pub struct XOFOutput {
    buffer: [u8; 64],
    position: usize,
    h0: [u8; 64],
    node_offset: u32,
    xof_length: u32,
}

impl UniformGenerator for XOFOutput {
    type Output = u8;

    fn generate(&mut self) -> Self::Output {
        if self.position == self.buffer.len() {
            let mut hasher = Blake2b::<64>::with_params(
                0,
                0,
                64,
                self.node_offset,
                self.xof_length,
                64,
                [0; 16],
            );
            hasher.update(self.h0);
            self.buffer = hasher.finalize();
            self.position = 0;
            self.node_offset += 1;
        }
        let b = self.buffer[self.position];
        self.position += 1;
        b
    }
}
