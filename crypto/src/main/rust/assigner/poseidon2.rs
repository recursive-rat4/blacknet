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

use crate::assigner::assigment::Assigment;
use crate::field::PrimeField;
use crate::poseidon2::Poseidon2Params;

pub trait Poseidon2Assigner<
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

    fn sboxp(assigment: &Assigment<F>, x: &mut F) {
        match Self::ALPHA {
            3 => {
                let x2 = x.square();
                let x3 = *x * x2;
                assigment.push(x2);
                assigment.push(x3);
                *x = x3;
            }
            5 => {
                let x2 = x.square();
                let x4 = x2.square();
                let x5 = *x * x4;
                assigment.push(x2);
                assigment.push(x4);
                assigment.push(x5);
                *x = x5;
            }
            9 => {
                let x2 = x.square();
                let x4 = x2.square();
                let x8 = x4.square();
                let x9 = *x * x8;
                assigment.push(x2);
                assigment.push(x4);
                assigment.push(x8);
                assigment.push(x9);
                *x = x9;
            }
            17 => {
                let x2 = x.square();
                let x4 = x2.square();
                let x8 = x4.square();
                let x16 = x8.square();
                let x17 = *x * x16;
                assigment.push(x2);
                assigment.push(x4);
                assigment.push(x8);
                assigment.push(x16);
                assigment.push(x17);
                *x = x17;
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn sbox(assigment: &Assigment<F>, x: &mut [F; WIDTH]) {
        for i in 0..WIDTH {
            Self::sboxp(assigment, &mut x[i]);
        }
    }

    fn permute(assigment: &Assigment<F>, x: &mut [F; WIDTH]) {
        Self::external(x);

        for round in 0..RBC / WIDTH {
            Self::rcb(round, x);
            Self::sbox(assigment, x);
            Self::external(x);
        }

        for round in 0..RPC {
            Self::rcp(round, x);
            Self::sboxp(assigment, &mut x[0]);
            Self::internal(x);
        }

        for round in 0..REC / WIDTH {
            Self::rce(round, x);
            Self::sbox(assigment, x);
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
> Poseidon2Assigner<F, WIDTH, RBC, RPC, REC> for P
{
}
