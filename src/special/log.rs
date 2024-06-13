/*
    special/log.rs
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

/// These routines compute the logarithm of the magnitude of `x`, $\log(|x|)$, for
/// $x \ne 0$.
/// 
/// Binds the function
/// [`gsl_sf_log_abs_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_abs_e).
pub fn log_abs_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_log_abs_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the logarithm of `x`, $\log(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_log_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_e).
pub fn log_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_log_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute $\log(1 + x)$ for $x > -1$ using an algorithm that is
/// accurate for small `x`.
/// 
/// Binds the function
/// [`gsl_sf_log_1plusx_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_1plusx_e).
pub fn log_1plusx_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_log_1plusx_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute $\log(1 + x) - x$ for $x > -1$ using an algorithm that is
/// accurate for small `x`.
/// 
/// Binds the function
/// [`gsl_sf_log_1plusx_mx_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_1plusx_mx_e).
pub fn log_1plusx_mx_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_log_1plusx_mx_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute $\log(1 + x)$ for $x > -1$ using an algorithm that is
/// accurate for small `x`.
/// 
/// Binds the function
/// [`gsl_sf_log_1plusx`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_1plusx).
pub fn log_1plusx(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_log_1plusx(x) }
}

/// These routines compute $\log(1 + x) - x$ for $x > -1$ using an algorithm that is
/// accurate for small `x`.
/// 
/// Binds the function
/// [`gsl_sf_log_1plusx_mx`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_1plusx_mx).
pub fn log_1plusx_mx(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_log_1plusx_mx(x) }
}

/// These routines compute the logarithm of `x`, $\log(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_log`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log).
pub fn log(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_log(x) }
}

/// These routines compute the logarithm of the magnitude of `x`, $\log(|x|)$, for
/// $x \ne 0$.
/// 
/// Binds the function
/// [`gsl_sf_log_abs`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_abs).
pub fn log_abs(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_log_abs(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_log_1plusx_e() {
        check_result!(log_1plusx_e(1.0e-10), 9.999999999500000000e-11, TEST_TOL0);
        check_result!(log_1plusx_e(1.0e-8), 9.999999950000000333e-09, TEST_TOL0);
        check_result!(log_1plusx_e(1.0e-4), 0.00009999500033330833533, TEST_TOL0);
        check_result!(log_1plusx_e(0.1), 0.09531017980432486004, TEST_TOL0);
        check_result!(log_1plusx_e(0.49), 0.3987761199573677730, TEST_TOL0);
        check_result!(log_1plusx_e(-0.49), -0.6733445532637655964, TEST_TOL0);
        check_result!(log_1plusx_e(1.0), M_LN2, TEST_TOL0);
        check_result!(log_1plusx_e(-0.99), -4.605170185988091368, TEST_TOL0);
    }

    #[test]
    fn test_log_1plusx_mx_e() {
        check_result!(log_1plusx_mx_e(1.0e-10), -4.999999999666666667e-21, TEST_TOL0);
        check_result!(log_1plusx_mx_e(1.0e-8), -4.999999966666666917e-17, TEST_TOL0);
        check_result!(log_1plusx_mx_e(1.0e-4), -4.999666691664666833e-09, TEST_TOL0);
        check_result!(log_1plusx_mx_e(0.1), -0.004689820195675139956, TEST_TOL0);
        check_result!(log_1plusx_mx_e(0.49), -0.09122388004263222704, TEST_TOL0);
        check_result!(log_1plusx_mx_e(-0.49), -0.18334455326376559639, TEST_TOL0);
        check_result!(log_1plusx_mx_e(1.0), M_LN2 + 1.0, TEST_TOL0);
        check_result!(log_1plusx_mx_e(-0.99), -3.615170185988091368, TEST_TOL0);
    }

    #[test]
    fn test_log_abs_e() {
        check_result!(log_abs_e(-0.1), -2.3025850929940456840, TEST_TOL0);
        check_result!(log_abs_e(-1.1), 0.09531017980432486004, TEST_TOL1);
        check_result!(log_abs_e(-1000.0), 6.907755278982137052, TEST_TOL0);
        check_result!(log_abs_e(0.1), -2.3025850929940456840, TEST_TOL0);
        check_result!(log_abs_e(1.1), 0.09531017980432486004, TEST_TOL1);
        check_result!(log_abs_e(1000.0), 6.907755278982137052, TEST_TOL0);
    }

    #[test]
    fn test_log_e() {
        check_result!(log_e(0.1), -2.3025850929940456840, TEST_TOL0);
        check_result!(log_e(1.1), 0.09531017980432486004, TEST_TOL1);
        check_result!(log_e(1000.0), 6.907755278982137052, TEST_TOL0);
    }
}