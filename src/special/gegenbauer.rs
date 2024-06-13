/*
    special/gegenbauer.rs
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

/// These functions evaluate the Gegenbauer polynomials $C^{(\lambda)}_n(x)$ using
/// explicit representations for $n = 1, 2, 3$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_1_e).
pub fn gegenpoly_1_e(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_gegenpoly_1_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

/// These functions evaluate the Gegenbauer polynomials $C^{(\lambda)}_n(x)$ using
/// explicit representations for $n = 1, 2, 3$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_3_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_3_e).
pub fn gegenpoly_3_e(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_gegenpoly_3_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

/// These functions evaluate the Gegenbauer polynomials $C^{(\lambda)}_n(x)$ using
/// explicit representations for $n = 1, 2, 3$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_2_e).
pub fn gegenpoly_2_e(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_gegenpoly_2_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

/// These functions evaluate the Gegenbauer polynomial $C^{(\lambda)}_n(x)$ for a
/// specific value of `n`, `lambda`, `x` subject to $\lambda > -1/2$, $n \ge 0$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_n_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_n_e).
pub fn gegenpoly_n_e(n: i32, lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_gegenpoly_n_e(n, lambda, x, &mut result))?;
        Ok(result.into())
    }
}


/// These functions evaluate the Gegenbauer polynomials $C^{(\lambda)}_n(x)$ using
/// explicit representations for $n = 1, 2, 3$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_1).
pub fn gegenpoly_1(lambda: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_gegenpoly_1(lambda, x) }
}

/// These functions evaluate the Gegenbauer polynomial $C^{(\lambda)}_n(x)$ for a
/// specific value of `n`, `lambda`, `x` subject to $\lambda > -1/2$, $n \ge 0$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_n`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_n).
pub fn gegenpoly_n(n: i32, lambda: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_gegenpoly_n(n, lambda, x) }
}

/// These functions evaluate the Gegenbauer polynomials $C^{(\lambda)}_n(x)$ using
/// explicit representations for $n = 1, 2, 3$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_3`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_3).
pub fn gegenpoly_3(lambda: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_gegenpoly_3(lambda, x) }
}

/// These functions evaluate the Gegenbauer polynomials $C^{(\lambda)}_n(x)$ using
/// explicit representations for $n = 1, 2, 3$.
/// 
/// Binds the function
/// [`gsl_sf_gegenpoly_2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_gegenpoly_2).
pub fn gegenpoly_2(lambda: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_gegenpoly_2(lambda, x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_gegenpoly_1_e() {
        check_result!(gegenpoly_1_e(-0.2, 1.0), -0.4, TEST_TOL0);
        check_result!(gegenpoly_1_e(0.0, 1.0), 2.0, TEST_TOL0);
        check_result!(gegenpoly_1_e(1.0, 1.0), 2.0, TEST_TOL0);
        check_result!(gegenpoly_1_e(1.0, 0.5), 1.0, TEST_TOL0);
        check_result!(gegenpoly_1_e(5.0, 1.0), 10.0, TEST_TOL0);
        check_result!(gegenpoly_1_e(100.0, 0.5), 100.0, TEST_TOL0);
    }

    #[test]
    fn test_gegenpoly_2_e() {
        check_result!(gegenpoly_2_e(-0.2, 0.5), 0.12, TEST_TOL0);
        check_result!(gegenpoly_2_e(0.0, 1.0), 1.00, TEST_TOL0);
        check_result!(gegenpoly_2_e(1.0, 1.0), 3.00, TEST_TOL0);
        check_result!(gegenpoly_2_e(1.0, 0.1), -0.96, TEST_TOL0);
        check_result!(gegenpoly_2_e(5.0, 1.0), 55.0, TEST_TOL0);
        check_result!(gegenpoly_2_e(100.0, 0.5), 4950.0, TEST_TOL0);
    }

    #[test]
    fn test_gegenpoly_3_e() {
        check_result!(gegenpoly_3_e(-0.2, 0.5), 0.112, TEST_TOL0);
        check_result!(gegenpoly_3_e(0.0, 1.0), -2.0/3.0, TEST_TOL0);
        check_result!(gegenpoly_3_e(1.0, 1.0), 4.000, TEST_TOL0);
        check_result!(gegenpoly_3_e(1.0, 0.1), -0.392, TEST_TOL0);
        check_result!(gegenpoly_3_e(5.0, 1.0), 220.000, TEST_TOL0);
        check_result!(gegenpoly_3_e(100.0, 0.5), 161600.000, TEST_TOL0);
    }

    #[test]
    fn test_gegenpoly_n_e() {
        check_result!(gegenpoly_n_e(1, 1.0, 1.0), 2.000, TEST_TOL0);
        check_result!(gegenpoly_n_e(10, 1.0, 1.0), 11.000, TEST_TOL0);
        check_result!(gegenpoly_n_e(10, 1.0, 0.1), -0.4542309376, TEST_TOL0);
        check_result!(gegenpoly_n_e(10, 5.0, 1.0), 9.23780e+4, TEST_TOL0);
        check_result!(gegenpoly_n_e(10, 100.0, 0.5), 1.5729338392690000e+13, TEST_TOL0);
        check_result!(gegenpoly_n_e(1000, 100.0, 1.0), 3.3353666135627322e+232, TEST_TOL1);
        check_result!(gegenpoly_n_e(100, 2000.0, 1.0), 5.8753432034937579e+202, TEST_TOL0);
        check_result!(gegenpoly_n_e(103, 207.0, 2.0), 1.4210272202235983e+145, TEST_TOL0);
        check_result!(gegenpoly_n_e(103, -0.4, 0.3), -1.64527498094522e-04, TEST_TOL1);
    }
}