/*
    special/psi.rs
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

/// These routines compute the Trigamma function $\psi'(x)$ for general `x`.
/// 
/// Binds the function
/// [`gsl_sf_psi_1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_1_e).
pub fn psi_1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_psi_1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the polygamma function $\psi^{(n)}(x)$ for $n \ge 0$, $x
/// > 0$.
/// 
/// Binds the function
/// [`gsl_sf_psi_n_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_n_e).
pub fn psi_n_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_psi_n_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Trigamma function $\psi'(n)$ for positive integer
/// $n$.
/// 
/// Binds the function
/// [`gsl_sf_psi_1_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_1_int_e).
pub fn psi_1_int_e(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_psi_1_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the digamma function $\psi(x)$ for general `x`, $x \ne
/// 0$.
/// 
/// Binds the function
/// [`gsl_sf_psi_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_e).
pub fn psi_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_psi_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the real part of the digamma function on the line $1 + i
/// y$, $\Re[\psi(1 + i y)]$.
/// 
/// Binds the function
/// [`gsl_sf_psi_1piy_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_1piy_e).
pub fn psi_1piy_e(y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_psi_1piy_e(y, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the digamma function $\psi(n)$ for positive integer `n`.
/// 
/// The digamma function is also called the Psi function.
/// 
/// Binds the function
/// [`gsl_sf_psi_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_int_e).
pub fn psi_int_e(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_psi_int_e(n, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the digamma function $\psi(x)$ for general `x`, $x \ne
/// 0$.
/// 
/// Binds the function
/// [`gsl_sf_psi`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi).
pub fn psi(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_psi(x) }
}

/// These routines compute the polygamma function $\psi^{(n)}(x)$ for $n \ge 0$, $x
/// > 0$.
/// 
/// Binds the function
/// [`gsl_sf_psi_n`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_n).
pub fn psi_n(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_psi_n(n, x) }
}

/// These routines compute the Trigamma function $\psi'(n)$ for positive integer
/// $n$.
/// 
/// Binds the function
/// [`gsl_sf_psi_1_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_1_int).
pub fn psi_1_int(n: i32) -> f64 {
    unsafe { bindings::gsl_sf_psi_1_int(n) }
}

/// These routines compute the Trigamma function $\psi'(x)$ for general `x`.
/// 
/// Binds the function
/// [`gsl_sf_psi_1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_1).
pub fn psi_1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_psi_1(x) }
}

/// These routines compute the digamma function $\psi(n)$ for positive integer `n`.
/// 
/// The digamma function is also called the Psi function.
/// 
/// Binds the function
/// [`gsl_sf_psi_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_int).
pub fn psi_int(n: i32) -> f64 {
    unsafe { bindings::gsl_sf_psi_int(n) }
}

/// These routines compute the real part of the digamma function on the line $1 + i
/// y$, $\Re[\psi(1 + i y)]$.
/// 
/// Binds the function
/// [`gsl_sf_psi_1piy`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_psi_1piy).
pub fn psi_1piy(y: f64) -> f64 {
    unsafe { bindings::gsl_sf_psi_1piy(y) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_psi_1_e() {
        check_result!(psi_1_e(1.0/32.0), 1025.5728544782377089, TEST_TOL0);
        check_result!(psi_1_e(1.0), 1.6449340668482264365, TEST_TOL0);
        check_result!(psi_1_e(5.0), 0.22132295573711532536, TEST_TOL0);
        check_result!(psi_1_e(100.0), 0.010050166663333571395, TEST_TOL0);
        check_result!(psi_1_e(110.0), 0.009132356622022545705, TEST_TOL0);
        check_result!(psi_1_e(500.0), 0.0020020013333322666697, TEST_TOL0);
        check_result!(psi_1_e(-1.0 + 1.0/128.0), 16386.648472598746587, TEST_TOL0);
        check_result!(psi_1_e(-1.50), 9.3792466449891237539, TEST_TOL0);
        check_result!(psi_1_e(-10.5), 9.7787577398148123845, TEST_TOL0);
        check_result!(psi_1_e(-15.5), 9.8071247184113896201, TEST_TOL0);
        check_result!(psi_1_e(-50.5), 9.8499971860824842274, TEST_TOL0);
        check_result!(psi_1_e(-1000.5), 9.8686054001734414233, TEST_TOL0);
    }

    #[test]
    fn test_psi_1_int_e() {
        check_result!(psi_1_int_e(1), 1.6449340668482264364, TEST_TOL0);
        check_result!(psi_1_int_e(2), 0.64493406684822643647, TEST_TOL0);
        check_result!(psi_1_int_e(3), 0.39493406684822643647, TEST_TOL0);
        check_result!(psi_1_int_e(4), 0.28382295573711532536, TEST_TOL0);
        check_result!(psi_1_int_e(1), 1.6449340668482264365, TEST_TOL0);
        check_result!(psi_1_int_e(5), 0.22132295573711532536, TEST_TOL0);
        check_result!(psi_1_int_e(100), 0.010050166663333571395, TEST_TOL0);
        check_result!(psi_1_int_e(110), 0.009132356622022545705, TEST_TOL0);
        check_result!(psi_1_int_e(500), 0.0020020013333322666697, TEST_TOL0);
    }

    #[test]
    fn test_psi_1piy_e() {
        check_result!(psi_1piy_e(0.8), -0.07088340212750589223, TEST_TOL1);
        check_result!(psi_1piy_e(1.0), 0.09465032062247697727, TEST_TOL0);
        check_result!(psi_1piy_e(5.0), 1.6127848446157465854, TEST_TOL2);
        check_result!(psi_1piy_e(100.0), 4.605178519404762003, TEST_TOL0);
        check_result!(psi_1piy_e(2000.0), 7.600902480375416216, TEST_TOL0);
        check_result!(psi_1piy_e(-0.8), -0.07088340212750589223, TEST_TOL1);
        check_result!(psi_1piy_e(-1.0), 0.09465032062247697727, TEST_TOL0);
        check_result!(psi_1piy_e(-5.0), 1.6127848446157465854, TEST_TOL2);
        check_result!(psi_1piy_e(-100.0), 4.605178519404762003, TEST_TOL0);
        check_result!(psi_1piy_e(-2000.0), 7.600902480375416216, TEST_TOL0);
    }

    #[test]
    fn test_psi_e() {
        check_result!(psi_e(5000.0), 8.517093188082904107, TEST_TOL0);
        check_result!(psi_e(5.0), 1.5061176684318004727, TEST_TOL0);
        check_result!(psi_e(-10.5), 2.3982391295357816134, TEST_TOL0);
        check_result!(psi_e(-100.5), 4.615124601338064117, TEST_TOL2);
        check_result!(psi_e(-1.0e+5 + 0.5), 11.512935464924395337, 4.0*TEST_TOL4);
        check_result!(psi_e(-262144.0 + 0.5), 12.476653064769611581, 4.0*TEST_TOL4);
    }

    #[test]
    fn test_psi_int_e() {
        check_result!(psi_int_e(1), -0.57721566490153286060, TEST_TOL0);
        check_result!(psi_int_e(2), 0.42278433509846713939, TEST_TOL0);
        check_result!(psi_int_e(3), 0.92278433509846713939, TEST_TOL0);
        check_result!(psi_int_e(4), 1.2561176684318004727, TEST_TOL0);
        check_result!(psi_int_e(5), 1.5061176684318004727, TEST_TOL0);
        check_result!(psi_int_e(100), 4.600161852738087400, TEST_TOL0);
        check_result!(psi_int_e(110), 4.695928024251535633, TEST_TOL0);
        check_result!(psi_int_e(5000), 8.517093188082904107, TEST_TOL0);
    }

    #[test]
    fn test_psi_n_e() {
        check_result!(psi_n_e(1, 1.0), 1.6449340668482264364, TEST_TOL0);
        check_result!(psi_n_e(1, 2.0), 0.64493406684822643647, TEST_TOL0);
        check_result!(psi_n_e(1, 3.0), 0.39493406684822643647, TEST_TOL0);
        check_result!(psi_n_e(1, 4.0), 0.28382295573711532536, TEST_TOL0);
        check_result!(psi_n_e(1, 5.0), 0.22132295573711532536, TEST_TOL0);
        check_result!(psi_n_e(1, 100.0), 0.010050166663333571395, TEST_TOL0);
        check_result!(psi_n_e(1, 110.0), 0.009132356622022545705, TEST_TOL0);
        check_result!(psi_n_e(1, 500.0), 0.0020020013333322666697, TEST_TOL0);
        check_result!(psi_n_e(3, 5.0), 0.021427828192755075022, TEST_TOL0);
        check_result!(psi_n_e(3, 500.0), 1.6048063999872000683e-08, TEST_TOL0);
        check_result!(psi_n_e(10, 5.0), -0.08675107579196581317, TEST_TOL1);
        check_result!(psi_n_e(10, 50.0), -4.101091112731268288e-12, TEST_TOL0);
        check_result!(psi_n_e(0, -1.5), 0.70315664064524318723, TEST_TOL0);
        check_result!(psi_n_e(1, -1.5), 9.3792466449891237539, TEST_TOL0);
    }
}