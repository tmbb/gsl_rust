/*
    special/laguerre.rs
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

/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_n(x)$ for $a > -1$,
/// $n \ge 0$.
/// 
/// Binds the function [`gsl_sf_laguerre_n_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_n_e).
pub fn laguerre_n_e(n: i32, a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_laguerre_n_e(n, a, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_1(x)$, $L^a_2(x)$, $L^a_3(x)$ using explicit
/// representations.
/// 
/// Binds the function [`gsl_sf_laguerre_3_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_3_e).
pub fn laguerre_3_e(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_laguerre_3_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_1(x)$, $L^a_2(x)$, $L^a_3(x)$ using explicit
/// representations.
/// 
/// Binds the function [`gsl_sf_laguerre_1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_1_e).
pub fn laguerre_1_e(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_laguerre_1_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_1(x)$, $L^a_2(x)$, $L^a_3(x)$ using explicit
/// representations.
/// 
/// Binds the function [`gsl_sf_laguerre_2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_2_e).
pub fn laguerre_2_e(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_laguerre_2_e(a, x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_1(x)$, $L^a_2(x)$, $L^a_3(x)$ using explicit
/// representations.
/// 
/// Binds the function [`gsl_sf_laguerre_2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_2).
pub fn laguerre_2(a: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_laguerre_2(a, x) }
}

/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_1(x)$, $L^a_2(x)$, $L^a_3(x)$ using explicit
/// representations.
/// 
/// Binds the function [`gsl_sf_laguerre_1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_1).
pub fn laguerre_1(a: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_laguerre_1(a, x) }
}

/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_n(x)$ for $a > -1$,
/// $n \ge 0$.
/// 
/// Binds the function [`gsl_sf_laguerre_n`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_n).
pub fn laguerre_n(n: i32, a: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_laguerre_n(n, a, x) }
}

/// These routines evaluate the generalized Laguerre polynomials
/// $L^a_1(x)$, $L^a_2(x)$, $L^a_3(x)$ using explicit
/// representations.
/// 
/// Binds the function [`gsl_sf_laguerre_3`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_laguerre_3).
pub fn laguerre_3(a: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_laguerre_3(a, x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_laguerre_1_e() {
        check_result!(laguerre_1_e(0.5, -1.0), 2.5, TEST_TOL0);
        check_result!(laguerre_1_e(0.5, 1.0), 0.5, TEST_TOL0);
        check_result!(laguerre_1_e(1.0, 1.0), 1.0, TEST_TOL0);
    }

    #[test]
    fn test_laguerre_2_e() {
        check_result!(laguerre_2_e(0.5, -1.0), 4.875, TEST_TOL0);
        check_result!(laguerre_2_e(0.5, 1.0), -0.125, TEST_TOL0);
        check_result!(laguerre_2_e(1.0, 1.0), 0.5, TEST_TOL0);
        check_result!(laguerre_2_e(-1.0, 1.0), -0.5, TEST_TOL0);
        check_result!(laguerre_2_e(-2.0, 1.0), 0.5, TEST_TOL0);
        check_result!(laguerre_2_e(-3.0, 1.0), 2.5, TEST_TOL0);
    }

    #[test]
    fn test_laguerre_3_e() {
        check_result!(laguerre_3_e(0.5, -1.0), 8.479166666666666667, TEST_TOL0);
        check_result!(laguerre_3_e(0.5, 1.0), -0.6041666666666666667, TEST_TOL0);
        check_result!(laguerre_3_e(1.0, 1.0), -0.16666666666666666667, TEST_TOL1);
        check_result!(laguerre_3_e(2.0, 1.0), 2.3333333333333333333, TEST_TOL0);
        check_result!(laguerre_3_e(-2.0, 1.0), 1.0/3.0, TEST_TOL0);
        check_result!(laguerre_3_e(-3.0, 1.0), -1.0/6.0, TEST_TOL0);
        check_result!(laguerre_3_e(-4.0, 1.0), -8.0/3.0, TEST_TOL0);
    }

    #[test]
    fn test_laguerre_n_e() {
        check_result!(laguerre_n_e(1, 0.5, 1.0), 0.5, TEST_TOL0);
        check_result!(laguerre_n_e(2, 1.0, 1.0), 0.5, TEST_TOL1);
        check_result!(laguerre_n_e(3, 2.0, 1.0), 2.3333333333333333333, TEST_TOL1);
        check_result!(laguerre_n_e(4, 2.0, 0.5), 6.752604166666666667, TEST_TOL1);
        check_result!(laguerre_n_e(90, 2.0, 0.5), -48.79047157201507897, TEST_TOL1);
        check_result!(laguerre_n_e(90, 2.0, -100.0), 2.5295879275042410902e+63, TEST_TOL2);
        check_result!(laguerre_n_e(90, 2.0, 100.0), -2.0929042259546928670e+20, TEST_TOL1);
        check_result!(laguerre_n_e(100, 2.0, -0.5), 2.2521795545919391405e+07, TEST_TOL2);
        check_result!(laguerre_n_e(100, 2.0, 0.5), -28.764832945909097418, TEST_TOL2);
        check_result!(laguerre_n_e(1000, 2.0, -0.5), 2.4399915170947549589e+21, TEST_TOL3);
        check_result!(laguerre_n_e(1000, 2.0, 0.5), -306.77440254315317525, TEST_TOL2);
        check_result!(laguerre_n_e(100000, 2.0, 1.0), 5107.73491348319, TEST_TOL4);
        check_result!(laguerre_n_e(1, -2.0, 1.0), -2.0, TEST_TOL0);
        check_result!(laguerre_n_e(2, -2.0, 1.0), 0.5, TEST_TOL0);
        check_result!(laguerre_n_e(3, -2.0, 1.0), 1.0/3.0, TEST_TOL0);
        check_result!(laguerre_n_e(10, -2.0, 1.0), -0.04654954805996472663, TEST_TOL2);
        check_result!(laguerre_n_e(10, -5.0, 1.0), -0.0031385030864197530864, TEST_TOL2);
        check_result!(laguerre_n_e(10, -9.0, 1.0), -2.480158730158730159e-06, TEST_TOL5);
        check_result!(laguerre_n_e(10, -11.0, 1.0), 2.7182818011463844797, TEST_TOL2);
        check_result!(laguerre_n_e(10, -11.0, -1.0), 0.3678794642857142857, TEST_TOL2);
        check_result!(laguerre_n_e(100, -2.0, 1.0), -0.0027339992019526273866, TEST_SQRT_TOL0);
        check_result!(laguerre_n_e(100, -2.0, -1.0), 229923.09193402028290, TEST_TOL5);
        check_result!(laguerre_n_e(100, -10.0, 1.0), 3.25966665871244092e-11, TEST_TOL6);
        check_result!(laguerre_n_e(100, -10.0, -1.0), 0.00016484365618205810025, TEST_TOL6);
        check_result!(laguerre_n_e(100, -20.0, 1.0), 5.09567630343671251e-21, TEST_TOL3);
        check_result!(laguerre_n_e(100, -30.0, 1.0), 3.46063150272466192e-34, TEST_TOL1);
        check_result!(laguerre_n_e(100, -50.0, 1.0), 1.20981872933162889e-65, TEST_TOL1);
        check_result!(laguerre_n_e(100, -50.0, -1.0), 8.60763477742332922e-65, TEST_TOL1);
        check_result!(laguerre_n_e(100, -50.5, 1.0), 4.84021010426688393e-31, TEST_TOL1);
        check_result!(laguerre_n_e(100, -50.5, -1.0), 8.49861345212160618e-33, TEST_TOL1);
        check_result!(laguerre_n_e(100, -101.0, 1.0), 2.7182818284590452354, TEST_TOL1);
        check_result!(laguerre_n_e(100, -101.0, -1.0), 0.3678794411714423216, TEST_TOL1);
        check_result!(laguerre_n_e(100, -102.0, 1.0), 271.8281828459045235, TEST_TOL1);
        check_result!(laguerre_n_e(100, -102.0, -1.0), 37.52370299948711680, TEST_TOL1);
        check_result!(laguerre_n_e(100, -110.0, 1.0), 1.0666955248998831554e+13, TEST_TOL1);
        check_result!(laguerre_n_e(100, -110.0, -1.0), 1.7028306108058225871e+12, TEST_TOL1);
        check_result!(laguerre_n_e(100, -200.0, 1.0), 7.47851889721356628e+58, TEST_TOL1);
        check_result!(laguerre_n_e(100, -200.0, -1.0), 2.73740299754732273e+58, TEST_TOL1);
        check_result!(laguerre_n_e(100, -50.0, 10.0), 4.504712811317745591e-21, TEST_SQRT_TOL0);
        check_result!(laguerre_n_e(100, -50.0, -10.0), 1.475165520610679937e-11, TEST_TOL1);
        check_result!(laguerre_n_e(100, 0.0, 0.5), 0.18682260367692278801, TEST_TOL2);
        check_result!(laguerre_n_e(100, 0.0, 10.5), 9.1796907354050059874, TEST_TOL2);
        check_result!(laguerre_n_e(100, 0.0, -10.5), 5.6329215744170606488e24, TEST_TOL2);
        check_result!(laguerre_n_e(100, 0.0, 100.5), -3.9844782875811907525e20, TEST_TOL2);
        check_result!(laguerre_n_e(100, 0.0, 150.0), -1.4463204337261709595e31, TEST_TOL2);
    }
}