use std::cmp::PartialOrd;
use crate::rng;

pub const N_ITERATIONS: i32 = 100_000;

pub const TEST_MOMENTS_SIGMA_CUTOFF: f64 = 3.0; 

pub struct TestMomentsResult {
    pub expected_count: i32,
    pub actual_count: i32,
    pub sigma: f64
}

/// Generates `N_ITERATIONS` random numbers from the given random number generator
/// and tests whether the expected statistical properties of the distribution
/// are close to the theoretical values
pub fn test_moments<T>(random_variable_sampler: fn(rng::Rng) -> T, a: T, b: T, expected_probability: f64)
                    -> TestMomentsResult
                    where T: PartialOrd {
    let r = rng::default_rng();

    let actual_count: f64 =
        (1..N_ITERATIONS)
        .filter(|_| {
            let value: T = random_variable_sampler(r);
            value < b && value > a
        })
        .count() as f64;
        

    let expected_count: f64 = expected_probability * (N_ITERATIONS as f64);

    let sigma: f64 =
        if expected_count > 0.0 {
            (actual_count - expected_count).abs() / expected_count.sqrt()
        } else {
            (actual_count - expected_count).abs()
        };

    TestMomentsResult{
        expected_count: expected_count as i32,
        actual_count: actual_count as i32,
        sigma: sigma
    }
}

#[macro_export]
macro_rules! assert_moments {
    ($closure:expr, $a:expr, $b:expr, p = $expected_probability:expr) => {
        let result = $crate::test_helpers::test_moments($closure, $a, $b, $expected_probability);
        if result.sigma > $crate::test_helpers::TEST_MOMENTS_SIGMA_CUTOFF {
            let message = format!("
┌────────────────────────────────────────────────────────────────────────
│ Moments test failed (expected counts too diferent from actual counts):
└─┬──────────────────────────────────────────────────────────────────────
  ├─ expected_count: {}
  ├─ actual_count: {}
  └─ sigma: {:.3} (should be < {:.1})\n\n",
                result.expected_count,
                result.actual_count,
                result.sigma,
                $crate::test_helpers::TEST_MOMENTS_SIGMA_CUTOFF);

            panic!("{}", message)
        } else {
            ()
        }
    }
}

pub use assert_moments;