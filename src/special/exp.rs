/*
    special/exp.rs
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

/// These routines compute the quantity $\exp(x)-1$ using an algorithm
/// that is accurate for small $x$.
/// 
/// Binds the function [`gsl_sf_expm1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expm1_e).
pub fn expm1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_expm1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the quantity $2(\exp(x)-1-x)/x^2$ using an
/// algorithm that is accurate for small `x`.
/// For small `x` the
/// algorithm is based on the expansion
/// $2(\exp(x)-1-x)/x^2 = 1 + x/3 + x^2/(3*4) + x^3/(3*4*5) + \dots$.
/// 
/// Binds the function [`gsl_sf_exprel_2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exprel_2_e).
pub fn exprel_2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_exprel_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the quantity $(\exp(x)-1)/x$ using an
/// algorithm that is accurate for small `x`.
/// For small `x` the
/// algorithm is based on the expansion
/// $(\exp(x)-1)/x = 1 + x/2 + x^2/(2*3) + x^3/(2*3*4) + \dots$.
/// 
/// Binds the function [`gsl_sf_exprel_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exprel_e).
pub fn exprel_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_exprel_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// This routine computes the product $y \exp(x)$ for the quantities
/// `x`, `y` with associated absolute errors `dx`, `dy`.
/// 
/// Binds the function [`gsl_sf_exp_mult_err_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exp_mult_err_e).
pub fn exp_mult_err_e(x: f64, dx: f64, y: f64, dy: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_exp_mult_err_e(x, dx, y, dy, &mut result))?;
        Ok(result.into())
    }
}

/// These routines exponentiate `x` and multiply by the factor `y`
/// to return the product $y \exp(x)$.
/// 
/// Binds the function [`gsl_sf_exp_mult_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exp_mult_e).
pub fn exp_mult_e(x: f64, y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_exp_mult_e(x, y, &mut result))?;
        Ok(result.into())
    }
}

/// These routines provide an exponential function $\exp(x)$ using GSL
/// semantics and error checking.
/// 
/// Binds the function [`gsl_sf_exp_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exp_e).
pub fn exp_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_exp_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// This function exponentiates `x` with an associated absolute error
/// `dx`.
/// 
/// Binds the function [`gsl_sf_exp_err_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exp_err_e).
pub fn exp_err_e(x: f64, dx: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_exp_err_e(x, dx, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the $N$-relative exponential, which is the
/// `n`-th generalization of the functions `gsl_sf_exprel()` and
/// `gsl_sf_exprel_2()`.
/// The $N$-relative exponential is given by,
/// 
/// $$\hbox{exprel}_N(x)
///             &= N!/x^N \left(\exp(x) - \sum_{k=0}^{N-1} x^k/k!\right)\cr
///             &= 1 + x/(N+1) + x^2/((N+1)(N+2)) + \dots\cr
///             &= {}_1F_1(1,1+N,x)\cr$$
/// 
/// Binds the function [`gsl_sf_exprel_n_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exprel_n_e).
pub fn exprel_n_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_exprel_n_e(n, x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the quantity $\exp(x)-1$ using an algorithm
/// that is accurate for small $x$.
/// 
/// Binds the function [`gsl_sf_expm1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expm1).
pub fn expm1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_expm1(x) }
}

/// These routines provide an exponential function $\exp(x)$ using GSL
/// semantics and error checking.
/// 
/// Binds the function [`gsl_sf_exp`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exp).
pub fn exp(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_exp(x) }
}

/// These routines exponentiate `x` and multiply by the factor `y`
/// to return the product $y \exp(x)$.
/// 
/// Binds the function [`gsl_sf_exp_mult`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exp_mult).
pub fn exp_mult(x: f64, y: f64) -> f64 {
    unsafe { bindings::gsl_sf_exp_mult(x, y) }
}

/// These routines compute the $N$-relative exponential, which is the
/// `n`-th generalization of the functions `gsl_sf_exprel()` and
/// `gsl_sf_exprel_2()`.
/// The $N$-relative exponential is given by,
/// 
/// $$\hbox{exprel}_N(x)
///             &= N!/x^N \left(\exp(x) - \sum_{k=0}^{N-1} x^k/k!\right)\cr
///             &= 1 + x/(N+1) + x^2/((N+1)(N+2)) + \dots\cr
///             &= {}_1F_1(1,1+N,x)\cr$$
/// 
/// Binds the function [`gsl_sf_exprel_n`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exprel_n).
pub fn exprel_n(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_exprel_n(n, x) }
}

/// These routines compute the quantity $(\exp(x)-1)/x$ using an
/// algorithm that is accurate for small `x`.
/// For small `x` the
/// algorithm is based on the expansion
/// $(\exp(x)-1)/x = 1 + x/2 + x^2/(2*3) + x^3/(2*3*4) + \dots$.
/// 
/// Binds the function [`gsl_sf_exprel`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exprel).
pub fn exprel(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_exprel(x) }
}

/// These routines compute the quantity $2(\exp(x)-1-x)/x^2$ using an
/// algorithm that is accurate for small `x`.
/// For small `x` the
/// algorithm is based on the expansion
/// $2(\exp(x)-1-x)/x^2 = 1 + x/3 + x^2/(3*4) + x^3/(3*4*5) + \dots$.
/// 
/// Binds the function [`gsl_sf_exprel_2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_exprel_2).
pub fn exprel_2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_exprel_2(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_exp_e() {
        check_result!(exp_e(-10.0), exp(-10.0), TEST_TOL0);
        check_result!(exp_e(10.0), exp(10.0), TEST_TOL0);
    }

    #[test]
    fn test_exp_err_e() {
        check_result!(exp_err_e(-10.0, TEST_TOL1), exp(-10.0), TEST_TOL1);
        check_result!(exp_err_e(10.0, TEST_TOL1), exp(10.0), TEST_TOL1);
    }

    #[test]
    fn test_exp_mult_e() {
        check_result!(exp_mult_e(-10.0, 1.0e-06), 1.0e-06*exp(-10.0), TEST_TOL0);
        check_result!(exp_mult_e(-10.0, 2.0), 2.0*exp(-10.0), TEST_TOL0);
        check_result!(exp_mult_e(-10.0, -2.0), -2.0*exp(-10.0), TEST_TOL0);
        check_result!(exp_mult_e(10.0, 1.0e-06), 1.0e-06*exp(10.0), TEST_TOL0);
        check_result!(exp_mult_e(10.0, -2.0), -2.0*exp(10.0), TEST_TOL0);
    }

    #[test]
    fn test_exp_mult_err_e() {
        check_result!(exp_mult_err_e(-10.0, TEST_SQRT_TOL0, 2.0, TEST_SQRT_TOL0), 2.0*exp(-10.0), TEST_SQRT_TOL0);
    }

    #[test]
    fn test_expm1_e() {
        check_result!(expm1_e(-10.0), exp(-10.0) + 1.0, TEST_TOL0);
        check_result!(expm1_e(-0.001), -0.00099950016662500845, TEST_TOL0);
        check_result!(expm1_e(-1.0e-8), -1.0e-08 + 0.5e-16, TEST_TOL0);
        check_result!(expm1_e(1.0e-8), 1.0e-08 + 0.5e-16, TEST_TOL0);
        check_result!(expm1_e(0.001), 0.0010005001667083417, TEST_TOL0);
        check_result!(expm1_e(10.0), exp(10.0) + 1.0, TEST_TOL0);
    }

    #[test]
    fn test_exprel_2_e() {
        check_result!(exprel_2_e(-10.0), 0.18000090799859524970, TEST_TOL0);
        check_result!(exprel_2_e(-0.001), 0.9996667499833361107, TEST_TOL0);
        check_result!(exprel_2_e(-1.0e-8), 0.9999999966666666750, TEST_TOL0);
        check_result!(exprel_2_e(1.0e-8), 1.0000000033333333417, TEST_TOL0);
        check_result!(exprel_2_e(0.001), 1.0003334166833361115, TEST_TOL0);
        check_result!(exprel_2_e(10.0), 440.3093158961343303, TEST_TOL0);
    }

    #[test]
    fn test_exprel_e() {
        check_result!(exprel_e(-10.0), 0.0999954600070237515, TEST_TOL0);
        check_result!(exprel_e(-0.001), 0.9995001666250084, TEST_TOL0);
        check_result!(exprel_e(-1.0e-8), 1.0 + 0.5e-08, TEST_TOL0);
        check_result!(exprel_e(1.0e-8), 1.0 + 0.5e-08, TEST_TOL0);
        check_result!(exprel_e(0.001), 1.0005001667083417, TEST_TOL0);
        check_result!(exprel_e(10.0), 2202.5465794806716517, TEST_TOL0);
    }

    #[test]
    fn test_exprel_n_e() {
        check_result!(exprel_n_e(3, -1000.0), 0.00299400600000000000, TEST_TOL0);
        check_result!(exprel_n_e(3, -100.0), 0.02940600000000000000, TEST_TOL0);
        check_result!(exprel_n_e(3, -10.0), 0.24599972760042142509, TEST_TOL0);
        check_result!(exprel_n_e(3, -3.0), 0.5444917625849191238, TEST_TOL0);
        check_result!(exprel_n_e(3, -0.001), 0.9997500499916678570, TEST_TOL0);
        check_result!(exprel_n_e(3, -1.0e-8), 0.9999999975000000050, TEST_TOL0);
        check_result!(exprel_n_e(3, 1.0e-8), 1.0000000025000000050, TEST_TOL0);
        check_result!(exprel_n_e(3, 0.001), 1.0002500500083345240, TEST_TOL0);
        check_result!(exprel_n_e(3, 3.0), 2.5745637607083706091, TEST_TOL0);
        check_result!(exprel_n_e(3, 3.1), 2.6772417068460206247, TEST_TOL0);
        check_result!(exprel_n_e(3, 10.0), 131.79279476884029910, TEST_TOL1);
        check_result!(exprel_n_e(3, 100.0), 1.6128702850896812690e+38, TEST_TOL2);
        check_result!(exprel_n_e(50, -1000.0), 0.04766231609253975959, TEST_TOL0);
        check_result!(exprel_n_e(50, -100.0), 0.3348247572345889317, TEST_TOL0);
        check_result!(exprel_n_e(50, -10.0), 0.8356287051853286482, TEST_TOL0);
        check_result!(exprel_n_e(50, -3.0), 0.9443881609152163615, TEST_TOL0);
        check_result!(exprel_n_e(50, -1.0), 0.980762245565660617, TEST_TOL0);
        check_result!(exprel_n_e(50, -1.0e-8), 1.0 + 1.0e-8/51.0, TEST_TOL0);
        check_result!(exprel_n_e(50, 1.0e-8), 1.0 + 1.0e-8/51.0, TEST_TOL0);
        check_result!(exprel_n_e(50, 1.0), 1.01999216583666790, TEST_TOL0);
        check_result!(exprel_n_e(50, 3.0), 1.0624205757460368307, TEST_TOL0);
        check_result!(exprel_n_e(50, 48.0), 7.499573876877194416, TEST_TOL0);
        check_result!(exprel_n_e(50, 50.1), 9.311803306230992272, TEST_TOL4);
        check_result!(exprel_n_e(50, 100.0), 8.175664432485807634e+07, TEST_TOL4);
        check_result!(exprel_n_e(50, 500.0), 4.806352370663185330e+146, TEST_TOL3);
        check_result!(exprel_n_e(500, -1000.0), 0.3334815803127619256, TEST_TOL0);
        check_result!(exprel_n_e(500, -100.0), 0.8335646217536183909, TEST_TOL0);
        check_result!(exprel_n_e(500, -10.0), 0.9804297803131823066, TEST_TOL0);
        check_result!(exprel_n_e(500, -3.0), 0.9940475488850672997, TEST_TOL0);
        check_result!(exprel_n_e(500, -1.0), 0.9980079602383488808, TEST_TOL0);
        check_result!(exprel_n_e(500, -1.0e-8), 1.0 + 1.0e-8/501.0, TEST_TOL0);
        check_result!(exprel_n_e(500, 1.0e-8), 1.0 + 1.0e-8/501.0, TEST_TOL0);
        check_result!(exprel_n_e(500, 1.0), 1.0019999920160634252, TEST_TOL0);
        check_result!(exprel_n_e(500, 3.0), 1.0060240236632444934, TEST_TOL0);
        check_result!(exprel_n_e(500, 48.0), 1.1059355517981272174, TEST_TOL0);
        check_result!(exprel_n_e(500, 100.0), 1.2492221464878287204, TEST_TOL1);
        check_result!(exprel_n_e(500, 500.0), 28.363019877927630858, TEST_TOL2);
        check_result!(exprel_n_e(500, 1000.0), 2.4037563160335300322e+68, TEST_TOL4);
        check_result!(exprel_n_e(500, 1600.0), 7.899293535320607403e+226, TEST_TOL4);
        check_result!(exprel_n_e(1263131, 1261282.3637), 545.0113107238425900305428360, TEST_TOL4);
    }
}