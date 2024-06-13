/*
    special/dilog.rs
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

/// These routines compute the dilogarithm for a real argument.
/// 
/// In Lewin’s notation this is $Li_2(x)$, the real part of the dilogarithm of a
/// real $x$. It is defined by the integral representation
/// 
/// $$Li_2(x) = - \Re \int_0^x ds \log(1-s) / s$$
/// 
/// Note that $\Im(Li_2(x)) = 0$ for $x \le 1$, and $-\pi\log(x)$ for $x > 1$.
/// 
/// Note that Abramowitz & Stegun refer to the Spence integral $S(x) = Li_2(1 - x)$
/// as the dilogarithm rather than $Li_2(x)$.
/// 
/// Binds the function
/// [`gsl_sf_dilog_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_dilog_e).
pub fn dilog_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_dilog_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the dilogarithm for a real argument.
/// 
/// In Lewin’s notation this is $Li_2(x)$, the real part of the dilogarithm of a
/// real $x$. It is defined by the integral representation
/// 
/// $$Li_2(x) = - \Re \int_0^x ds \log(1-s) / s$$
/// 
/// Note that $\Im(Li_2(x)) = 0$ for $x \le 1$, and $-\pi\log(x)$ for $x > 1$.
/// 
/// Note that Abramowitz & Stegun refer to the Spence integral $S(x) = Li_2(1 - x)$
/// as the dilogarithm rather than $Li_2(x)$.
/// 
/// Binds the function
/// [`gsl_sf_dilog`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_dilog).
pub fn dilog(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_dilog(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_dilog_e() {
        check_result!(dilog_e(-3.0), -1.9393754207667089531, TEST_TOL0);
        check_result!(dilog_e(-0.5), -0.4484142069236462024, TEST_TOL0);
        check_result!(dilog_e(-0.001), -0.0009997501110486510834, TEST_TOL0);
        check_result!(dilog_e(0.1), 0.1026177910993911, TEST_TOL0);
        check_result!(dilog_e(0.7), 0.8893776242860387386, TEST_TOL0);
        check_result!(dilog_e(1.0), 1.6449340668482260, TEST_TOL0);
        check_result!(dilog_e(1.5), 2.3743952702724802007, TEST_TOL0);
        check_result!(dilog_e(2.0), 2.4674011002723397, TEST_TOL0);
        check_result!(dilog_e(5.0), 1.7837191612666306277, TEST_TOL0);
        check_result!(dilog_e(11.0), 0.3218540439999117111, TEST_TOL1);
        check_result!(dilog_e(12.59), 0.0010060918167266208634, TEST_TOL3);
        check_result!(dilog_e(12.595), 0.00003314826006436236810, TEST_TOL5);
        check_result!(dilog_e(13.0), -0.07806971248458575855, TEST_TOL2);
        check_result!(dilog_e(20.0), -1.2479770861745251168, TEST_TOL2);
        check_result!(dilog_e(150.0), -9.270042702348657270, TEST_TOL0);
        check_result!(dilog_e(1100.0), -21.232504073931749553, TEST_TOL0);
    }
}