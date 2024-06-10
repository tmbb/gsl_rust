/*
    special/elementary.rs
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

/// This function multiplies `x` and `y` storing the product and its
/// associated error in `result`.
/// 
/// Binds the function [`gsl_sf_multiply_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_multiply_e).
pub fn multiply_e(x: f64, y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_multiply_e(x, y, &mut result))?;
        Ok(result.into())
    }
}

/// This function multiplies `x` and `y` with associated absolute
/// errors `dx` and `dy`.
/// The product
/// $xy \pm xy \sqrt{(dx/x)^2 +(dy/y)^2}$
/// is stored in `result`.
/// 
/// Binds the function [`gsl_sf_multiply_err_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_multiply_err_e).
pub fn multiply_err_e(x: f64, dx: f64, y: f64, dy: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_multiply_err_e(x, dx, y, dy, &mut result))?;
        Ok(result.into())
    }
}


/// This function multiplies `x` and `y` storing the product and its
/// associated error in `result`.
/// 
/// Binds the function [`gsl_sf_multiply`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_multiply).
pub fn multiply(x: f64, y: f64) -> f64 {
    unsafe { bindings::gsl_sf_multiply(x, y) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_multiply_e() {
        check_result!(multiply_e(-3.0, 2.0), -6.0, TEST_TOL0);
    }
}