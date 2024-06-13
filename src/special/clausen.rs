/*
    special/clausen.rs
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

/// These routines compute the Clausen integral $Cl_2(x)$.
/// 
/// Binds the function
/// [`gsl_sf_clausen_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_clausen_e).
pub fn clausen_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_clausen_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the Clausen integral $Cl_2(x)$.
/// 
/// Binds the function
/// [`gsl_sf_clausen`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_clausen).
pub fn clausen(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_clausen(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_clausen_e() {
        check_result!(clausen_e(M_PI/20.0), 0.4478882448133546, TEST_TOL0);
        check_result!(clausen_e(M_PI/6.0), 0.8643791310538927, TEST_TOL0);
        check_result!(clausen_e(M_PI/3.0), 1.0149416064096535, TEST_TOL0);
        check_result!(clausen_e(2.0*M_PI + M_PI/3.0), 1.0149416064096535, TEST_TOL0);
        check_result!(clausen_e(100.0*M_PI + M_PI/3.0), 1.0149416064096535, TEST_TOL0);
    }
}