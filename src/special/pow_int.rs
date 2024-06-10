/*
    special/pow_int.rs
    Copyright (C) 2021 Pim van den Berg
    Copyright (C) 2024 Tiago Barroso

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

#![allow(non_snake_case)]

use crate::bindings;
use crate::{GSLError, ValWithError, Result};

/// These routines compute the power $x^n$ for integer `n`.
/// The
/// power is computed using the minimum number of multiplications. For
/// example, $x^8$ is computed as $((x^2)^2)^2$, requiring only 3
/// multiplications.
/// For reasons of efficiency, these functions do not
/// check for overflow or underflow conditions. The following is a simple example:
/// 
/// #include <gsl/gsl_sf_pow_int.h>/*compute3.0**12*/doubley=gsl_sf_pow_int(3.0,12);Binds the function [`gsl_sf_pow_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_pow_int_e).
pub fn pow_int_e(x: f64, n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_pow_int_e(x, n, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the power $x^n$ for integer `n`.
/// The
/// power is computed using the minimum number of multiplications. For
/// example, $x^8$ is computed as $((x^2)^2)^2$, requiring only 3
/// multiplications.
/// For reasons of efficiency, these functions do not
/// check for overflow or underflow conditions. The following is a simple example:
/// 
/// #include <gsl/gsl_sf_pow_int.h>/*compute3.0**12*/doubley=gsl_sf_pow_int(3.0,12);Binds the function [`gsl_sf_pow_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_pow_int).
pub fn pow_int(x: f64, n: i32) -> f64 {
    unsafe { bindings::gsl_sf_pow_int(x, n) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_pow_int_e() {
        check_result!(pow_int_e(2.0, 3), 8.0, TEST_TOL0);
        check_result!(pow_int_e(-2.0, 3), -8.0, TEST_TOL0);
        check_result!(pow_int_e(2.0, -3), 1.0/8.0, TEST_TOL0);
        check_result!(pow_int_e(-2.0, -3), -1.0/8.0, TEST_TOL0);
        check_result!(pow_int_e(10.0, 4), 1.0e+4, TEST_TOL0);
        check_result!(pow_int_e(10.0, -4), 1.0e-4, TEST_TOL0);
        check_result!(pow_int_e(-10.0, 4), 1.0e+4, TEST_TOL0);
        check_result!(pow_int_e(-10.0, -4), 1.0e-4, TEST_TOL0);
        check_result!(pow_int_e(10.0, 40), 1.0e+40, TEST_TOL0);
        check_result!(pow_int_e(8.0, -40), 7.523163845262640051e-37, TEST_TOL0);
        check_result!(pow_int_e(-10.0, 40), 1.0e+40, TEST_TOL0);
        check_result!(pow_int_e(-8.0, -40), 7.523163845262640051e-37, TEST_TOL0);
        check_result!(pow_int_e(10.0, 41), 1.0e+41, TEST_TOL0);
        check_result!(pow_int_e(8.0, -41), 9.403954806578300064e-38, TEST_TOL0);
        check_result!(pow_int_e(-10.0, 41), -1.0e+41, TEST_TOL0);
        check_result!(pow_int_e(-8.0, -41), -9.403954806578300064e-38, TEST_TOL0);
    }
}