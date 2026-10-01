/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

//! Sponge metaphor.

use crate::random::Distribution;

/// Phase of sponge state
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Phase {
    /// Absorbing into sponge
    #[default]
    Absorb,
    /// Squeezing from sponge
    Squeeze,
}

pub trait Sponge: Sized {
    type Msg;

    fn reset(&mut self);

    fn absorb_msg(&mut self, e: Self::Msg);
    fn squeeze_msg(&mut self) -> Self::Msg;

    #[inline]
    fn absorb<S: Absorb<Self::Msg>>(&mut self, e: S) {
        e.absorb_into(self)
    }

    #[inline]
    fn absorb_iter<S: Absorb<Self::Msg>, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        for item in iter {
            item.absorb_into(self)
        }
    }

    #[inline]
    fn squeeze<S: Squeeze<Self::Msg>>(&mut self) -> S {
        S::squeeze_from(self)
    }

    #[inline]
    fn squeeze_with_size<S: SqueezeWithSize<Self::Msg>>(&mut self, size: usize) -> S {
        S::squeeze_from(self, size)
    }
}

pub trait Absorb<T> {
    fn absorb_into<S: Sponge<Msg = T>>(self, sponge: &mut S);
}

pub trait Squeeze<T> {
    fn squeeze_from<S: Sponge<Msg = T>>(sponge: &mut S) -> Self;
}

pub trait SqueezeWithSize<T> {
    fn squeeze_from<S: Sponge<Msg = T>>(sponge: &mut S, size: usize) -> Self;
}

impl Absorb<u8> for u8 {
    #[inline]
    fn absorb_into<S: Sponge<Msg = u8>>(self, sponge: &mut S) {
        sponge.absorb_msg(self)
    }
}

impl Squeeze<u8> for u8 {
    #[inline]
    fn squeeze_from<S: Sponge<Msg = u8>>(sponge: &mut S) -> Self {
        sponge.squeeze_msg()
    }
}

/// Uniform distribution from sponge.
#[derive(Default)]
pub struct UniformDistribution;

impl UniformDistribution {
    /// Construct the new distribution.
    pub const fn new() -> Self {
        Self
    }

    /// Reset internal state.
    pub const fn reset(&mut self) {}
}

impl<T: Squeeze<S::Msg>, S: Sponge> Distribution<T, S> for UniformDistribution {
    #[inline]
    fn sample(&mut self, sponge: &mut S) -> T {
        sponge.squeeze()
    }

    #[inline]
    fn reset(&mut self) {}
}
