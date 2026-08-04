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
use crate::symmetric::Duplexer;
use crate::symmetric::blake2b::Blake2b512;

#[derive(Clone, Copy)]
pub struct Blake2bDuplexer {
    inner: Inner,
}

#[derive(Clone, Copy)]
enum Inner {
    Absorb {
        hasher: Blake2b512,
    },
    Squeeze {
        chain: [u8; 32],
        buffer: [u8; 32],
        position: usize,
    },
}

impl Blake2bDuplexer {
    /// Construct new duplexer.
    pub const fn new() -> Self {
        Self {
            inner: Inner::Absorb {
                hasher: Self::hasher(),
            },
        }
    }

    const fn hasher() -> Blake2b512 {
        Blake2b512::with_personalization(*b"hashchain duplex")
    }
}

impl Default for Blake2bDuplexer {
    fn default() -> Self {
        Self::new()
    }
}

impl Duplexer for Blake2bDuplexer {
    type Msg = u8;

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn absorb_msg(&mut self, e: Self::Msg) {
        match self.inner {
            Inner::Absorb { ref mut hasher } => hasher.update([e]),
            Inner::Squeeze {
                chain,
                buffer: _,
                position: _,
            } => {
                let mut hasher = Self::hasher();
                hasher.update(chain);
                hasher.update([e]);
                self.inner = Inner::Absorb { hasher };
            }
        }
    }

    fn squeeze_msg(&mut self) -> Self::Msg {
        match self.inner {
            Inner::Absorb { hasher } => {
                let mut chain = [0u8; 32];
                let mut buffer = [0u8; 32];
                let hash = hasher.finalize();
                let (l, r) = hash.split_at(32);
                chain.copy_from_slice(l);
                buffer.copy_from_slice(r);
                self.inner = Inner::Squeeze {
                    chain,
                    buffer,
                    position: 1,
                };
                buffer[0]
            }
            Inner::Squeeze {
                ref mut chain,
                ref mut buffer,
                ref mut position,
            } => {
                if *position != 32 {
                    let e = buffer[*position];
                    *position += 1;
                    e
                } else {
                    let mut hasher = Self::hasher();
                    hasher.update(*chain);
                    let hash = hasher.finalize();
                    let (l, r) = hash.split_at(32);
                    chain.copy_from_slice(l);
                    buffer.copy_from_slice(r);
                    *position = 1;
                    buffer[0]
                }
            }
        }
    }
}

impl UniformGenerator for Blake2bDuplexer {
    type Output = u8;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.squeeze_msg()
    }
}
