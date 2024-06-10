/*
    special/erf.rs
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

/// These routines compute the logarithm of the complementary error function
/// $\log(\text{erfc}(x))$.
/// 
/// Binds the function [`gsl_sf_log_erfc_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_erfc_e).
pub fn log_erfc_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_log_erfc_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the error function $\text{erf}(x)$,
/// where
/// $\text{erf}(x) = (2/\sqrt{\pi}) \int_0^x dt \exp(-t^2)$.
/// 
/// Binds the function [`gsl_sf_erf_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erf_e).
pub fn erf_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_erf_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the upper tail of the Gaussian probability function
/// $Q(x) = (1/\sqrt{2\pi}) \int_x^\infty dt \exp(-t^2/2)$
/// 
/// Binds the function [`gsl_sf_erf_Q_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erf_Q_e).
pub fn erf_Q_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_erf_Q_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Gaussian probability density function
/// $Z(x) = (1/\sqrt{2\pi}) \exp(-x^2/2)$
/// 
/// Binds the function [`gsl_sf_erf_Z_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erf_Z_e).
pub fn erf_Z_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_erf_Z_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complementary error function
/// $\text{erfc}(x) = 1 - \text{erf}(x) = (2/\sqrt{\pi}) \int_x^\infty \exp(-t^2)$
/// 
/// Binds the function [`gsl_sf_erfc_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erfc_e).
pub fn erfc_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_erfc_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the hazard function for the normal distribution.
/// 
/// Binds the function [`gsl_sf_hazard_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hazard_e).
pub fn hazard_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hazard_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the Gaussian probability density function
/// $Z(x) = (1/\sqrt{2\pi}) \exp(-x^2/2)$
/// 
/// Binds the function [`gsl_sf_erf_Z`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erf_Z).
pub fn erf_Z(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_erf_Z(x) }
}

/// These routines compute the logarithm of the complementary error function
/// $\log(\text{erfc}(x))$.
/// 
/// Binds the function [`gsl_sf_log_erfc`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_log_erfc).
pub fn log_erfc(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_log_erfc(x) }
}

/// These routines compute the complementary error function
/// $\text{erfc}(x) = 1 - \text{erf}(x) = (2/\sqrt{\pi}) \int_x^\infty \exp(-t^2)$
/// 
/// Binds the function [`gsl_sf_erfc`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erfc).
pub fn erfc(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_erfc(x) }
}

/// These routines compute the error function $\text{erf}(x)$,
/// where
/// $\text{erf}(x) = (2/\sqrt{\pi}) \int_0^x dt \exp(-t^2)$.
/// 
/// Binds the function [`gsl_sf_erf`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erf).
pub fn erf(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_erf(x) }
}

/// These routines compute the hazard function for the normal distribution.
/// 
/// Binds the function [`gsl_sf_hazard`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hazard).
pub fn hazard(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hazard(x) }
}

/// These routines compute the upper tail of the Gaussian probability function
/// $Q(x) = (1/\sqrt{2\pi}) \int_x^\infty dt \exp(-t^2/2)$
/// 
/// Binds the function [`gsl_sf_erf_Q`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_erf_Q).
pub fn erf_Q(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_erf_Q(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_erf_Q_e() {
        check_result!(erf_Q_e(10.0), 7.619853024160526066e-24, TEST_TOL2);
    }

    #[test]
    fn test_erf_Z_e() {
        check_result!(erf_Z_e(1.0), 0.24197072451914334980, TEST_TOL0);
    }

    #[test]
    fn test_erf_e() {
        check_result!(erf_e(-10.0), -1.0000000000000000000, TEST_TOL0);
        check_result!(erf_e(0.5), 0.5204998778130465377, TEST_TOL0);
        check_result!(erf_e(1.0), 0.8427007929497148693, TEST_TOL0);
        check_result!(erf_e(10.0), 1.0000000000000000000, TEST_TOL0);
    }

    #[test]
    fn test_erfc_e() {
        check_result!(erfc_e(-10.0), 2.0, TEST_TOL0);
        check_result!(erfc_e(-5.0000002), 1.9999999999984625433, TEST_TOL0);
        check_result!(erfc_e(-5.0), 1.9999999999984625402, TEST_TOL0);
        check_result!(erfc_e(-1.0), 1.8427007929497148693, TEST_TOL0);
        check_result!(erfc_e(-0.5), 1.5204998778130465377, TEST_TOL0);
        check_result!(erfc_e(1.0), 0.15729920705028513066, TEST_TOL0);
        check_result!(erfc_e(3.0), 0.000022090496998585441373, TEST_TOL1);
        check_result!(erfc_e(7.0), 4.183825607779414399e-23, TEST_TOL2);
        check_result!(erfc_e(10.0), 2.0884875837625447570e-45, TEST_TOL2);
    }

    #[test]
    fn test_hazard_e() {
        check_result!(hazard_e(-20.0), 5.5209483621597631896e-88, TEST_TOL2);
        check_result!(hazard_e(-10.0), 7.6945986267064193463e-23, TEST_TOL2);
        check_result!(hazard_e(-1.0), 0.28759997093917836123, TEST_TOL0);
        check_result!(hazard_e(0.0), 0.79788456080286535588, TEST_TOL0);
        check_result!(hazard_e(1.0), 1.5251352761609812091, TEST_TOL0);
        check_result!(hazard_e(10.0), 10.098093233962511963, TEST_TOL2);
        check_result!(hazard_e(20.0), 20.049753068527850542, TEST_TOL2);
        check_result!(hazard_e(30.0), 30.033259667433677037, TEST_TOL2);
        check_result!(hazard_e(50.0), 50.019984031905639809, TEST_TOL0);
        check_result!(hazard_e(80.0), 80.012496096798234468, TEST_TOL0);
        check_result!(hazard_e(150.0), 150.00666607420571802, TEST_TOL0);
        check_result!(hazard_e(300.0), 300.00333325926337415, TEST_TOL0);
        check_result!(hazard_e(900.0), 900.00111110836764382, TEST_TOL0);
        check_result!(hazard_e(1001.0), 1001.0009989990049990, TEST_TOL0);
        check_result!(hazard_e(2000.0), 2000.0004999997500003, TEST_TOL0);
    }

    #[test]
    fn test_log_erfc_e() {
        check_result!(log_erfc_e(-1.0), log(1.842700792949714869), TEST_TOL0);
        check_result!(log_erfc_e(-0.1), 0.106576400586522485015, TEST_TOL0);
        check_result!(log_erfc_e(0.0), log(1.0), TEST_TOL0);
        check_result!(log_erfc_e(0.001), -0.0011290158896213548027, TEST_TOL0);
        check_result!(log_erfc_e(0.1), -0.119304973737395598329, TEST_TOL0);
        check_result!(log_erfc_e(1.0), log(0.15729920705028513066), TEST_TOL0);
        check_result!(log_erfc_e(10.0), log(2.0884875837625447570e-45), TEST_TOL0);
    }
}