/*
    special/synchrotron.rs
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

/// These routines compute the second synchrotron function
/// $x K_{2/3}(x)$ for $x \ge 0$.
/// 
/// Binds the function [`gsl_sf_synchrotron_2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_synchrotron_2_e).
pub fn synchrotron_2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_synchrotron_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the first synchrotron function
/// $x \int_x^\infty dt K_{5/3}(t)$
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_sf_synchrotron_1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_synchrotron_1_e).
pub fn synchrotron_1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_synchrotron_1_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the second synchrotron function
/// $x K_{2/3}(x)$ for $x \ge 0$.
/// 
/// Binds the function [`gsl_sf_synchrotron_2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_synchrotron_2).
pub fn synchrotron_2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_synchrotron_2(x) }
}

/// These routines compute the first synchrotron function
/// $x \int_x^\infty dt K_{5/3}(t)$
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_sf_synchrotron_1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_synchrotron_1).
pub fn synchrotron_1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_synchrotron_1(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_synchrotron_1_e() {
        check_result!(synchrotron_1_e(0.01), 0.444972504114210632, TEST_TOL0);
        check_result!(synchrotron_1_e(1.0), 0.651422815355364504, TEST_TOL1);
        check_result!(synchrotron_1_e(10.0), 0.000192238264300868882, TEST_TOL1);
        check_result!(synchrotron_1_e(100.0), 4.69759366592220221e-43, TEST_TOL1);
    }

    #[test]
    fn test_synchrotron_2_e() {
        check_result!(synchrotron_2_e(0.01), 0.23098077342226277732, TEST_TOL2);
        check_result!(synchrotron_2_e(1.0), 0.4944750621042082670, TEST_TOL1);
        check_result!(synchrotron_2_e(10.0), 0.00018161187569530204281, TEST_TOL1);
        check_result!(synchrotron_2_e(256.0), 1.3272635474353774058e-110, TEST_TOL4);
    }
}