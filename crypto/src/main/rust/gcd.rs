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

// https://eprint.iacr.org/2020/972
pub fn gcd_inner<const ITERATIONS: usize>(a: &mut u64, b: &mut u64) -> (i64, i64, i64, i64) {
    let (mut f0, mut g0, mut f1, mut g1) = (1, 0, 0, 1);
    for _ in 0..ITERATIONS {
        if *a & 1 == 1 {
            if a < b {
                (*a, *b) = (*b, *a);
                (f0, f1) = (f1, f0);
                (g0, g1) = (g1, g0);
            }
            *a -= *b;
            f0 -= f1;
            g0 -= g1;
        }
        *a >>= 1;
        f1 <<= 1;
        g1 <<= 1;
    }
    (f0, g0, f1, g1)
}
