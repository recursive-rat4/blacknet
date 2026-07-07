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

use crate::algebra::{
    AdditiveCommutativeMagma, AdditiveSemigroup, BalancedRepresentative, Double, IntegerModRing,
    LeftOne, LeftZero, MultiplicativeCommutativeMagma, MultiplicativeSemigroup, One, RightOne,
    RightZero, Set, Square, Zero,
};
use crate::branchless::{BlAbs, BlAssign, BlEq, BlSelect};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use zeroize::DefaultIsZeroes;

macro_rules! impl_uring {
    ( $($x:tt, $int:ty, $sint:ty, $long:ty, $bits:literal, $modulus:literal),+ ) => {
        $(
            #[doc = concat!(stringify!($int), " as ring.")]
            #[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
            #[repr(transparent)]
            pub struct $x {
                n: $int,
            }

            impl Debug for $x {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    write!(f, "{}", self.canonical())
                }
            }

            impl From<$int> for $x {
                fn from(n: $int) -> $x {
                    $x { n }
                }
            }

            impl From<$x> for $int {
                fn from(e: $x) -> $int {
                    e.n
                }
            }

            impl Add for $x {
                type Output = $x;

                fn add(self, rps: $x) -> $x {
                    $x {
                        n: self.n.wrapping_add(rps.n),
                    }
                }
            }

            impl Add<&$x> for $x {
                type Output = $x;

                fn add(self, rps: &$x) -> $x {
                    $x {
                        n: self.n.wrapping_add(rps.n),
                    }
                }
            }

            impl Add<$x> for &$x {
                type Output = $x;

                fn add(self, rps: $x) -> $x {
                    $x {
                        n: self.n.wrapping_add(rps.n),
                    }
                }
            }

            impl<'a> Add<&'a $x> for &$x {
                type Output = $x;

                fn add(self, rps: &'a $x) -> $x {
                    $x {
                        n: self.n.wrapping_add(rps.n),
                    }
                }
            }

            impl AddAssign for $x {
                fn add_assign(&mut self, rps: $x) {
                    self.n = self.n.wrapping_add(rps.n)
                }
            }

            impl AddAssign<&$x> for $x {
                fn add_assign(&mut self, rps: &$x) {
                    self.n = self.n.wrapping_add(rps.n)
                }
            }

            impl Double for $x {
                type Output = $x;

                fn double(self) -> $x {
                    $x { n: self.n << 1 }
                }
            }

            impl Double for &$x {
                type Output = $x;

                fn double(self) -> $x {
                    $x { n: self.n << 1 }
                }
            }

            impl Neg for $x {
                type Output = $x;

                fn neg(self) -> $x {
                    $x {
                        n: self.n.wrapping_neg(),
                    }
                }
            }

            impl Neg for &$x {
                type Output = $x;

                fn neg(self) -> $x {
                    $x {
                        n: self.n.wrapping_neg(),
                    }
                }
            }

            impl Sub for $x {
                type Output = $x;

                fn sub(self, rps: $x) -> $x {
                    $x {
                        n: self.n.wrapping_sub(rps.n),
                    }
                }
            }

            impl Sub<&$x> for $x {
                type Output = $x;

                fn sub(self, rps: &$x) -> $x {
                    $x {
                        n: self.n.wrapping_sub(rps.n),
                    }
                }
            }

            impl Sub<$x> for &$x {
                type Output = $x;

                fn sub(self, rps: $x) -> $x {
                    $x {
                        n: self.n.wrapping_sub(rps.n),
                    }
                }
            }

            impl<'a> Sub<&'a $x> for &$x {
                type Output = $x;

                fn sub(self, rps: &'a $x) -> $x {
                    $x {
                        n: self.n.wrapping_sub(rps.n),
                    }
                }
            }

            impl SubAssign for $x {
                fn sub_assign(&mut self, rps: $x) {
                    self.n = self.n.wrapping_sub(rps.n)
                }
            }

            impl SubAssign<&$x> for $x {
                fn sub_assign(&mut self, rps: &$x) {
                    self.n = self.n.wrapping_sub(rps.n)
                }
            }

            impl Mul for $x {
                type Output = $x;

                fn mul(self, rps: $x) -> $x {
                    $x {
                        n: self.n.wrapping_mul(rps.n),
                    }
                }
            }

            impl Mul<&$x> for $x {
                type Output = $x;

                fn mul(self, rps: &$x) -> $x {
                    $x {
                        n: self.n.wrapping_mul(rps.n),
                    }
                }
            }

            impl Mul<$x> for &$x {
                type Output = $x;

                fn mul(self, rps: $x) -> $x {
                    $x {
                        n: self.n.wrapping_mul(rps.n),
                    }
                }
            }

            impl<'a> Mul<&'a $x> for &$x {
                type Output = $x;

                fn mul(self, rps: &'a $x) -> $x {
                    $x {
                        n: self.n.wrapping_mul(rps.n),
                    }
                }
            }

            impl MulAssign for $x {
                fn mul_assign(&mut self, rps: $x) {
                    self.n = self.n.wrapping_mul(rps.n)
                }
            }

            impl MulAssign<&$x> for $x {
                fn mul_assign(&mut self, rps: &$x) {
                    self.n = self.n.wrapping_mul(rps.n)
                }
            }

            impl Square for $x {
                type Output = $x;

                fn square(self) -> $x {
                    $x {
                        n: self.n.wrapping_mul(self.n),
                    }
                }
            }

            impl Square for &$x {
                type Output = $x;

                fn square(self) -> $x {
                    $x {
                        n: self.n.wrapping_mul(self.n),
                    }
                }
            }

            impl Sum for $x {
                fn sum<I: Iterator<Item = $x>>(iter: I) -> $x {
                    iter.reduce(|lps, rps| lps + rps).unwrap_or($x::ZERO)
                }
            }

            impl<'a> Sum<&'a $x> for $x {
                fn sum<I: Iterator<Item = &'a $x>>(iter: I) -> $x {
                    iter.copied().sum()
                }
            }

            impl Product for $x {
                fn product<I: Iterator<Item = $x>>(iter: I) -> $x {
                    iter.reduce(|lps, rps| lps * rps).unwrap_or($x::ONE)
                }
            }

            impl<'a> Product<&'a $x> for $x {
                fn product<I: Iterator<Item = &'a $x>>(iter: I) -> $x {
                    iter.copied().product()
                }
            }

            impl LeftZero for $x {
                const LEFT_ZERO: $x = $x { n: 0 };
            }

            impl RightZero for $x {
                const RIGHT_ZERO: $x = $x { n: 0 };
            }

            impl Zero for $x {
                const ZERO: $x = $x { n: 0 };
            }

            impl LeftOne for $x {
                const LEFT_ONE: $x = $x { n: 1 };
            }

            impl RightOne for $x {
                const RIGHT_ONE: $x = $x { n: 1 };
            }

            impl One for $x {
                const ONE: $x = $x { n: 1 };
            }

            impl Set for $x {}

            impl AdditiveCommutativeMagma for $x {}

            impl AdditiveSemigroup for $x {}

            impl MultiplicativeCommutativeMagma for $x {}

            impl MultiplicativeSemigroup for $x {}

            impl IntegerModRing for $x {
                type Int = $int;
                type Modulus = $long;

                fn new(n: $int) -> $x {
                    $x { n }
                }
                fn with_limb(n: $int) -> $x {
                    $x { n }
                }

                fn canonical(&self) -> $int {
                    self.n
                }
                fn absolute(&self) -> $int {
                    self.balanced().bl_unsigned_abs()
                }

                const BITS: u32 = $bits;
                const MODULUS: $long = $modulus;
            }

            impl BalancedRepresentative for $x {
                type Output = $sint;

                fn balanced(&self) -> $sint {
                    self.n as $sint
                }
            }

            impl BlAssign for $x {
                fn bl_assign(&mut self, rps: $x, condition: bool) {
                    self.n.bl_assign(rps.n, condition)
                }
            }

            impl BlAssign<&$x> for $x {
                fn bl_assign(&mut self, rps: &$x, condition: bool) {
                    self.n.bl_assign(&rps.n, condition)
                }
            }

            impl BlSelect for $x {
                type Output = $x;

                fn bl_select(self, rps: $x, condition: bool) -> $x {
                    let n = self.n.bl_select(rps.n, condition);
                    $x { n }
                }
            }

            impl BlSelect<&$x> for $x {
                type Output = $x;

                fn bl_select(self, rps: &$x, condition: bool) -> $x {
                    let n = self.n.bl_select(&rps.n, condition);
                    $x { n }
                }
            }

            impl BlSelect<$x> for &$x {
                type Output = $x;

                fn bl_select(self, rps: $x, condition: bool) -> $x {
                    let n = (&self.n).bl_select(rps.n, condition);
                    $x { n }
                }
            }

            impl BlSelect for &$x {
                type Output = $x;

                fn bl_select(self, rps: &$x, condition: bool) -> $x {
                    let n = (&self.n).bl_select(&rps.n, condition);
                    $x { n }
                }
            }

            impl BlEq for $x {
                fn bl_eq(&self, rps: &$x) -> bool {
                    self.n.bl_eq(&rps.n)
                }

                fn bl_ne(&self, rps: &$x) -> bool {
                    self.n.bl_ne(&rps.n)
                }
            }

            impl DefaultIsZeroes for $x {}
        )+
    };
}

impl_uring!(
    U8Ring,
    u8,
    i8,
    u16,
    8,
    256,
    U16Ring,
    u16,
    i16,
    u32,
    16,
    65536,
    U32Ring,
    u32,
    i32,
    u64,
    32,
    4294967296,
    U64Ring,
    u64,
    i64,
    u128,
    64,
    18446744073709551616
);
