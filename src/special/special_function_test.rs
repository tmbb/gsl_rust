use crate::machine;
use crate::*;

const TEST_FACTOR: f64 = 100.0;
const TEST_SIGMA: f64 = 2.5;

pub const TEST_TOL0: f64 = 2.0 * machine::DBL_EPSILON;
pub const TEST_TOL1: f64 = 16.0 * machine::DBL_EPSILON;
pub const TEST_TOL2: f64 = 256.0 * machine::DBL_EPSILON;
pub const TEST_TOL3: f64 = 2048.0 * machine::DBL_EPSILON;
pub const TEST_TOL4: f64 = 16384.0 * machine::DBL_EPSILON;
pub const TEST_TOL5: f64 = 131072.0 * machine::DBL_EPSILON;
pub const TEST_TOL6: f64 = 1048576.0 * machine::DBL_EPSILON;

pub const TEST_SQRT_TOL0: f64 = 2.0 * machine::DBL_EPSILON;
pub const TEST_SNGL: f64 = 1.0e-6;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SpecialFunctionTestError {
    incons: bool,
    negative_error: bool,
    bad_error: bool,
    pub bad_tolerance: bool,
    pub big_error: bool,
    bad_exponent: bool,
    pub tolerance: f64,
    pub frac_diff: f64,
    pub abs_diff: f64,
    pub error: f64
}


pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

pub fn fractional_difference(x1: f64, x2: f64) -> f64 {
    if x1 == 0.0 && x2 == 0.0 {
        return 0.0;
    } else if x1 == 0.0 {
        return x1.abs();
    } else if x1 <= machine::DBL_MAX && x2 <= machine::DBL_MAX && (x1 + x2 != 0.0) {
        return ((x1 - x2) / (x1 + x2)).abs();
    } else {
        return 1.0;
    }
}

#[macro_export]
macro_rules! check_result {
    ($call:expr, $expected:expr, $tolerance:expr) => {
        match $crate::special::special_function_test::check_result_helper(
            $call, $expected, $tolerance
        ) {
            std::result::Result::Ok(()) =>
                (),
            
            std::result::Result::Err($crate::special::special_function_test::SpecialFunctionTestError{
                tolerance: tolerance,
                frac_diff: frac_diff,
                abs_diff: abs_diff,
                error: error,
                big_error: big_error,
                bad_tolerance: bad_tolerance,
                ..
            }) => {
                let message = format!("

    ┌────────────────────────────────────────────────────────────
    │ Special function test failed:
    └─┬──────────────────────────────────────────────────────────
      ├─ tolerance: {:+e}
      ├─ fractional difference: {:+e}
      ├─ absolute difference: {:+e}
      ├─ error: {:+e}
      │
      └──┬─ error too big?: {}
         └─ value outside tolerance?: {}\n\n",
                    tolerance,
                    frac_diff,
                    abs_diff,
                    error,
                    big_error,
                    bad_tolerance);
                
                panic!("{}", message)
            }
        }
    }
}

pub fn check_result_helper(result: Result<ValWithError<f64>>, expected: f64, tolerance: f64)
        -> std::result::Result<(), SpecialFunctionTestError> {
    let val_with_err = result.unwrap();

    let frac_diff: f64 = fractional_difference(expected, val_with_err.val);
    let abs_diff: f64 = (expected - val_with_err.val).abs();

    let incons: bool = abs_diff > 2.0 * TEST_SIGMA * val_with_err.err;
    let negative_error: bool = val_with_err.err < 0.0;
    let bad_error: bool = false;
    let bad_tolerance: bool = frac_diff > TEST_FACTOR * tolerance;
    let big_error: bool = (abs_diff > 0.0) && (val_with_err.err > 1e4 * expected.abs() * tolerance);
    let bad_exponent: bool = false;

    let is_error: bool = incons || bad_error || negative_error || bad_tolerance || big_error || bad_exponent;

    if is_error {
        let special_function_test_error = SpecialFunctionTestError {
            incons: incons,
            bad_error: bad_error,
            negative_error: negative_error,
            bad_tolerance: bad_tolerance,
            big_error: big_error,
            bad_exponent: bad_exponent,
            tolerance: tolerance,
            frac_diff: frac_diff,
            abs_diff: abs_diff,
            error: val_with_err.err
        };

        std::result::Result::Err(special_function_test_error)

    } else {
        std::result::Result::Ok(())
    }
}

// Make these useful constants available
pub use math::*;
pub use machine::*;

pub use check_result;