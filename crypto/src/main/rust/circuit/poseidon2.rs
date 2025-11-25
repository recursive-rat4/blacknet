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

use crate::circuit::circuitbuilder::{CircuitBuilder, Constant, LinearCombination, Scope};
use crate::field::PrimeField;
use crate::operation::{Double, Square};
use crate::poseidon2::Poseidon2Params;

pub trait Poseidon2Circuit<
    F: PrimeField,
    const WIDTH: usize,
    const RBC: usize,
    const RPC: usize,
    const REC: usize,
>: Poseidon2Params<F, WIDTH, RBC, RPC, REC>
{
    fn m4(x: &mut [LinearCombination<F>; WIDTH]) {
        for i in 0..WIDTH >> 2 {
            let j = i << 2;
            let t0 = &x[j] + &x[j + 1];
            let t1 = &x[j + 2] + &x[j + 3];
            let t2 = (&x[j + 1]).double() + &t1;
            let t3 = (&x[j + 3]).double() + &t0;
            let t4 = t1.double().double() + &t3;
            let t5 = t0.double().double() + &t2;
            let t6 = t3 + &t5;
            let t7 = t2 + &t4;
            x[j] = t6;
            x[j + 1] = t5;
            x[j + 2] = t7;
            x[j + 3] = t4;
        }
    }

    fn external(x: &mut [LinearCombination<F>; WIDTH]) {
        match WIDTH {
            2 => {
                let s = &x[0] + &x[1];
                x[0] += &s;
                x[1] += s;
            }
            3 => {
                let s = &x[0] + &x[1] + &x[2];
                x[0] += &s;
                x[1] += &s;
                x[2] += s;
            }
            4 => {
                Self::m4(x);
            }
            8 | 12 | 16 | 20 | 24 => {
                Self::m4(x);
                let mut s: [LinearCombination<F>; 4] = Default::default();
                for i in 0..4 {
                    s[i] = x[i].clone();
                    for j in 1..WIDTH >> 2 {
                        s[i] += &x[(j << 2) + i];
                    }
                }
                for i in 0..WIDTH {
                    x[i] += &s[i & 3];
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn internal(x: &mut [LinearCombination<F>; WIDTH]) {
        match WIDTH {
            2 => {
                let s = &x[0] + &x[1];
                x[0] += &s;
                x[1] = (&x[1]).double() + s;
            }
            3 => {
                let s = &x[0] + &x[1] + &x[2];
                x[0] += &s;
                x[1] += &s;
                x[2] = (&x[2]).double() + s;
            }
            4 | 8 | 12 | 16 | 20 | 24 => {
                let mut s = x[0].clone();
                for i in 1..WIDTH {
                    s += &x[i];
                }
                for i in 0..WIDTH {
                    x[i] = &x[i] * Constant::new(Self::M[i]) + &s;
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn rcb(round: usize, x: &mut [LinearCombination<F>; WIDTH]) {
        for i in 0..WIDTH {
            x[i] += Constant::new(Self::RCB[round * WIDTH + i]);
        }
    }

    fn rcp(round: usize, x: &mut [LinearCombination<F>; WIDTH]) {
        x[0] += Constant::new(Self::RCP[round]);
    }

    fn rce(round: usize, x: &mut [LinearCombination<F>; WIDTH]) {
        for i in 0..WIDTH {
            x[i] += Constant::new(Self::RCE[round * WIDTH + i]);
        }
    }

    fn sboxp(scope: &Scope<F>, x: &mut LinearCombination<F>) {
        match Self::ALPHA {
            3 => {
                let x2 = scope.auxiliary();
                let x3 = scope.auxiliary();
                scope.constrain((&*x).square(), x2);
                scope.constrain(&*x * x2, x3);
                *x = x3.into();
            }
            5 => {
                let x2 = scope.auxiliary();
                let x4 = scope.auxiliary();
                let x5 = scope.auxiliary();
                scope.constrain((&*x).square(), x2);
                scope.constrain(x2.square(), x4);
                scope.constrain(&*x * x4, x5);
                *x = x5.into();
            }
            9 => {
                let x2 = scope.auxiliary();
                let x4 = scope.auxiliary();
                let x8 = scope.auxiliary();
                let x9 = scope.auxiliary();
                scope.constrain((&*x).square(), x2);
                scope.constrain(x2.square(), x4);
                scope.constrain(x4.square(), x8);
                scope.constrain(&*x * x8, x9);
                *x = x9.into();
            }
            17 => {
                let x2 = scope.auxiliary();
                let x4 = scope.auxiliary();
                let x8 = scope.auxiliary();
                let x16 = scope.auxiliary();
                let x17 = scope.auxiliary();
                scope.constrain((&*x).square(), x2);
                scope.constrain(x2.square(), x4);
                scope.constrain(x4.square(), x8);
                scope.constrain(x8.square(), x16);
                scope.constrain(&*x * x16, x17);
                *x = x17.into();
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn sbox(scope: &Scope<F>, x: &mut [LinearCombination<F>; WIDTH]) {
        for i in 0..WIDTH {
            Self::sboxp(scope, &mut x[i]);
        }
    }

    fn permute(circuit: &CircuitBuilder<F>, x: &mut [LinearCombination<F>; WIDTH]) {
        let scope = circuit.scope("Poseidon2::permute");

        Self::external(x);

        for round in 0..RBC / WIDTH {
            Self::rcb(round, x);
            Self::sbox(&scope, x);
            Self::external(x);
        }

        for round in 0..RPC {
            Self::rcp(round, x);
            Self::sboxp(&scope, &mut x[0]);
            Self::internal(x);
        }

        for round in 0..REC / WIDTH {
            Self::rce(round, x);
            Self::sbox(&scope, x);
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
> Poseidon2Circuit<F, WIDTH, RBC, RPC, REC> for P
{
}
