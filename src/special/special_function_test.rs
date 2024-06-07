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
    bad_tolerance: bool,
    big_error: bool,
    bad_exponent: bool
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

pub fn check_result(result: Result<ValWithError<f64>>, expected: f64, tolerance: f64) -> () {
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
            bad_exponent: bad_exponent
        };

        panic!("{:#?}", special_function_test_error)

    } else {
        ()
    };
}

