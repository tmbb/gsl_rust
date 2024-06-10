/*
    rng.rs
    Copyright (C) 2021 Pim van den Berg

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use crate::bindings;

pub type RngType = bindings::gsl_rng_type;
pub type Rng = bindings::gsl_rng;

const DEFAULT_RNG_SEED: u64 = 618033988;

pub fn default_rng_with_seed(seed: u64) -> Rng {
    unsafe {
        let rng_type: *const RngType = bindings::gsl_rng_mt19937;
        let mut rng: Rng = *bindings::gsl_rng_alloc(rng_type);
        bindings::gsl_rng_set(&mut rng, seed);
        rng
    }
}

pub fn default_rng() -> Rng {
    default_rng_with_seed(DEFAULT_RNG_SEED)
}

pub fn random_uniform_integer(mut rng: Rng, n: u64) -> u64 {
    unsafe { bindings::gsl_rng_uniform_int(&mut rng, n) }
}

#[cfg(test)]
mod test {
    use crate::rng;

    #[test]
    fn test_rng_without_seed() {
        _ = rng::default_rng();
    }

    #[test]
    fn test_rng_with_seed() {
        _ = rng::default_rng_with_seed(0);
        _ = rng::default_rng_with_seed(234532);
        _ = rng::default_rng_with_seed(962);
    }

    #[test]
    fn test_reproducibility_of_rng_with_seed() {
        let rng_for_seed: rng::Rng = rng::default_rng_with_seed(95012);
        let n: u64 = 2^32;

        for _i in 1..20 {
            let seed = rng::random_uniform_integer(rng_for_seed, n);

            // Seed two generators with the same seed
            let rng1 = rng::default_rng_with_seed(seed);
            let rng2 = rng::default_rng_with_seed(seed);

            // Generate 1000 values from the RNGs and ensure
            // they are the same for both generators.
            for _j in 1..1000 {
                let x = rng::random_uniform_integer(rng1, n);
                let y = rng::random_uniform_integer(rng2, n);

                assert_eq!(x, y)
            }
        }
    }
}
