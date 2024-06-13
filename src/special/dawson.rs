/*
    special/dawson.rs
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

/// These routines compute the value of Dawson’s integral for `x`.
/// 
/// Binds the function
/// [`gsl_sf_dawson_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_dawson_e).
pub fn dawson_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_dawson_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the value of Dawson’s integral for `x`.
/// 
/// Binds the function
/// [`gsl_sf_dawson`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_dawson).
pub fn dawson(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_dawson(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_dawson_e() {
        check_result!(dawson_e(1.0e-15), 1.0e-15, TEST_TOL0);
        check_result!(dawson_e(0.5), 0.4244363835020222959, TEST_TOL0);
        check_result!(dawson_e(2.0), 0.30134038892379196603, TEST_TOL0);
        check_result!(dawson_e(1000.0), 0.0005000002500003750009, TEST_TOL0);
    }
}