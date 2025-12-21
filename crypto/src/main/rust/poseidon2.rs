/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

#![allow(clippy::needless_range_loop)]

use crate::field::PrimeField;

// https://eprint.iacr.org/2023/323

pub trait Poseidon2Params<
    F: PrimeField,
    const WIDTH: usize,
    const RBC: usize,
    const RPC: usize,
    const REC: usize,
>
{
    const ALPHA: usize;
    const RCB: [F; RBC];
    const RCP: [F; RPC];
    const RCE: [F; REC];
    const M: [F; WIDTH];
}

pub trait Poseidon2Plain<
    F: PrimeField,
    const WIDTH: usize,
    const RBC: usize,
    const RPC: usize,
    const REC: usize,
>: Poseidon2Params<F, WIDTH, RBC, RPC, REC>
{
    fn m4(x: &mut [F; WIDTH]) {
        for i in 0..WIDTH >> 2 {
            let j = i << 2;
            let t0 = x[j] + x[j + 1];
            let t1 = x[j + 2] + x[j + 3];
            let t2 = x[j + 1].double() + t1;
            let t3 = x[j + 3].double() + t0;
            let t4 = t1.double().double() + t3;
            let t5 = t0.double().double() + t2;
            let t6 = t3 + t5;
            let t7 = t2 + t4;
            x[j] = t6;
            x[j + 1] = t5;
            x[j + 2] = t7;
            x[j + 3] = t4;
        }
    }

    fn external(x: &mut [F; WIDTH]) {
        match WIDTH {
            2 => {
                let s = x[0] + x[1];
                x[0] += s;
                x[1] += s;
            }
            3 => {
                let s = x[0] + x[1] + x[2];
                x[0] += s;
                x[1] += s;
                x[2] += s;
            }
            4 => {
                Self::m4(x);
            }
            8 | 12 | 16 | 20 | 24 => {
                Self::m4(x);
                let mut s = [F::ZERO; 4];
                for i in 0..4 {
                    s[i] = x[i];
                    for j in 1..WIDTH >> 2 {
                        s[i] += x[(j << 2) + i];
                    }
                }
                for i in 0..WIDTH {
                    x[i] += s[i & 3];
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn internal(x: &mut [F; WIDTH]) {
        match WIDTH {
            2 => {
                let s = x[0] + x[1];
                x[0] += s;
                x[1] = x[1].double() + s;
            }
            3 => {
                let s = x[0] + x[1] + x[2];
                x[0] += s;
                x[1] += s;
                x[2] = x[2].double() + s;
            }
            4 | 8 | 12 | 16 | 20 | 24 => {
                let mut s = x[0];
                for i in 1..WIDTH {
                    s += x[i];
                }
                for i in 0..WIDTH {
                    x[i] = x[i] * Self::M[i] + s;
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn rcb(round: usize, x: &mut [F; WIDTH]) {
        for i in 0..WIDTH {
            x[i] += Self::RCB[round * WIDTH + i];
        }
    }

    fn rcp(round: usize, x: &mut [F; WIDTH]) {
        x[0] += Self::RCP[round];
    }

    fn rce(round: usize, x: &mut [F; WIDTH]) {
        for i in 0..WIDTH {
            x[i] += Self::RCE[round * WIDTH + i];
        }
    }

    fn sboxp(x: &mut F) {
        match Self::ALPHA {
            3 => {
                *x *= x.square();
            }
            5 => {
                *x *= x.square().square();
            }
            9 => {
                *x *= x.square().square().square();
            }
            17 => {
                *x *= x.square().square().square().square();
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn sbox(x: &mut [F; WIDTH]) {
        for i in 0..WIDTH {
            Self::sboxp(&mut x[i]);
        }
    }

    fn permute(x: &mut [F; WIDTH]) {
        Self::external(x);

        for round in 0..RBC / WIDTH {
            Self::rcb(round, x);
            Self::sbox(x);
            Self::external(x);
        }

        for round in 0..RPC {
            Self::rcp(round, x);
            Self::sboxp(&mut x[0]);
            Self::internal(x);
        }

        for round in 0..REC / WIDTH {
            Self::rce(round, x);
            Self::sbox(x);
            Self::external(x);
        }
    }
}

impl<
    F: PrimeField,
    const WIDTH: usize,
    const RBC: usize,
    const RPC: usize,
    const REC: usize,
    P: Poseidon2Params<F, WIDTH, RBC, RPC, REC>,
> Poseidon2Plain<F, WIDTH, RBC, RPC, REC> for P
{
}
