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

use blacknet_crypto::distribution::{Distribution, UniformDistribution};
use blacknet_crypto::duplex::Duplex;
use blacknet_crypto::eqextension::EqExtension;
use blacknet_crypto::lm::LMField;
use blacknet_crypto::multilinearextension::MultilinearExtension;
use blacknet_crypto::operation::Double;
use blacknet_crypto::poseidon2lm::DuplexPoseidon2LM;
use blacknet_crypto::semiring::{Presemiring, Semiring};
use blacknet_crypto::sumcheck::SumCheck;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const VARS: usize = 16;
type Z = LMField;
type D = DuplexPoseidon2LM;
type E = UniformDistribution<D>;

fn make_eq() -> (EqExtension<Z>, Z) {
    let mut i = Z::ONE;
    let mut coefficients = Vec::<Z>::with_capacity(VARS);
    (0..VARS).for_each(|_| {
        coefficients.push(i);
        i = i.double();
    });
    (coefficients.into(), Z::ONE)
}

fn make_mle() -> (MultilinearExtension<Z>, Z) {
    let mut i = Z::ZERO;
    let mut sum = Z::ZERO;
    let mut coefficients = Vec::<Z>::with_capacity(1 << VARS);
    (0..1 << VARS).for_each(|_| {
        coefficients.push(i);
        sum += i;
        i += Z::ONE;
    });
    (coefficients.into(), sum)
}

fn criterion_benchmark(crit: &mut Criterion) {
    let mut duplex = D::default();
    let mut exceptional_set = UniformDistribution::default();

    let (eq, sum) = black_box(make_eq());
    crit.bench_function("SumCheck prove Eq", |bench| {
        bench.iter(|| {
            type SC = SumCheck<Z, EqExtension<Z>, D, E>;

            let proof = SC::prove(eq.clone(), sum, &mut duplex, &mut exceptional_set);
            duplex.reset();
            exceptional_set.reset();

            proof
        })
    });

    let (mle, sum) = black_box(make_mle());
    crit.bench_function("SumCheck prove MLE", |bench| {
        bench.iter(|| {
            type SC = SumCheck<Z, MultilinearExtension<Z>, D, E>;

            let proof = SC::prove(mle.clone(), sum, &mut duplex, &mut exceptional_set);
            duplex.reset();
            exceptional_set.reset();

            proof
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
