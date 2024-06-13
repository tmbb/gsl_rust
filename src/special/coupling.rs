/*
    special/coupling.rs
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

/// These routines compute the Wigner 6-j coefficient,
/// 
/// $$\left\{ \begin{array}{ccc} ja & jb & jc \\ jd & je & jf \end{array} \right\}$$
/// 
/// where the arguments are given in half-integer units, $ja$ = `two_ja`/2, $ma$ =
/// `two_ma`/2, etc.
/// 
/// Binds the function
/// [`gsl_sf_coupling_6j_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_coupling_6j_e).
pub fn coupling_6j_e(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_coupling_6j_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Wigner 9-j coefficient,
/// 
/// $$\left\{ \begin{array}{ccc} ja & jb & jc \\ jd & je & jf \\ jg & jh & ji
/// \end{array} \right\}$$
/// 
/// where the arguments are given in half-integer units, $ja$ = `two_ja`/2, $ma$ =
/// `two_ma`/2, etc.
/// 
/// Binds the function
/// [`gsl_sf_coupling_9j_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_coupling_9j_e).
pub fn coupling_9j_e(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32, two_jg: i32, two_jh: i32, two_ji: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_coupling_9j_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, two_jg, two_jh, two_ji, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Wigner 3-j coefficient,
/// 
/// $$\left( \begin{array}{ccc} ja & jb & jc \\ ma & mb & mc \end{array} \right)$$
/// 
/// where the arguments are given in half-integer units, $ja$ = `two_ja`/2, $ma$ =
/// `two_ma`/2, etc.
/// 
/// Binds the function
/// [`gsl_sf_coupling_3j_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_coupling_3j_e).
pub fn coupling_3j_e(two_ja: i32, two_jb: i32, two_jc: i32, two_ma: i32, two_mb: i32, two_mc: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_coupling_3j_e(two_ja, two_jb, two_jc, two_ma, two_mb, two_mc, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the Wigner 3-j coefficient,
/// 
/// $$\left( \begin{array}{ccc} ja & jb & jc \\ ma & mb & mc \end{array} \right)$$
/// 
/// where the arguments are given in half-integer units, $ja$ = `two_ja`/2, $ma$ =
/// `two_ma`/2, etc.
/// 
/// Binds the function
/// [`gsl_sf_coupling_3j`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_coupling_3j).
pub fn coupling_3j(two_ja: i32, two_jb: i32, two_jc: i32, two_ma: i32, two_mb: i32, two_mc: i32) -> f64 {
    unsafe { bindings::gsl_sf_coupling_3j(two_ja, two_jb, two_jc, two_ma, two_mb, two_mc) }
}

/// These routines compute the Wigner 6-j coefficient,
/// 
/// $$\left\{ \begin{array}{ccc} ja & jb & jc \\ jd & je & jf \end{array} \right\}$$
/// 
/// where the arguments are given in half-integer units, $ja$ = `two_ja`/2, $ma$ =
/// `two_ma`/2, etc.
/// 
/// Binds the function
/// [`gsl_sf_coupling_6j`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_coupling_6j).
pub fn coupling_6j(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32) -> f64 {
    unsafe { bindings::gsl_sf_coupling_6j(two_ja, two_jb, two_jc, two_jd, two_je, two_jf) }
}

/// These routines compute the Wigner 9-j coefficient,
/// 
/// $$\left\{ \begin{array}{ccc} ja & jb & jc \\ jd & je & jf \\ jg & jh & ji
/// \end{array} \right\}$$
/// 
/// where the arguments are given in half-integer units, $ja$ = `two_ja`/2, $ma$ =
/// `two_ma`/2, etc.
/// 
/// Binds the function
/// [`gsl_sf_coupling_9j`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_coupling_9j).
pub fn coupling_9j(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32, two_jg: i32, two_jh: i32, two_ji: i32) -> f64 {
    unsafe { bindings::gsl_sf_coupling_9j(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, two_jg, two_jh, two_ji) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_coupling_3j_e() {
        check_result!(coupling_3j_e(0, 1, 1, 0, 1, -1), sqrt(1.0/2.0), TEST_TOL0);
        check_result!(coupling_3j_e(1, 1, 2, 1, -1, 0), sqrt(1.0/6.0), TEST_TOL0);
        check_result!(coupling_3j_e(2, 4, 6, 0, 2, -2), sqrt(8.0/105.0), TEST_TOL0);
        check_result!(coupling_3j_e(4, 4, 8, 0, 0, 0), sqrt(2.0/35.0), TEST_TOL0);
        check_result!(coupling_3j_e(4, 4, 8, 2, -2, 0), 2.0/3.0*sqrt(2.0/35.0), TEST_TOL2);
        check_result!(coupling_3j_e(4, 4, 8, 4, -4, 0), 1.0/(3.0*sqrt(70.0)), TEST_TOL2);
        check_result!(coupling_3j_e(1, 1, 2, 2, -1, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(1, 1, 2, 1, -2, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(1, 1, 2, 1, -1, 3), 0.0, 0.0);
        check_result!(coupling_3j_e(1, 1, 3, 1, -1, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(1, 4, 2, 1, -1, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(4, 1, 2, 1, -1, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(2*13, 2*13, 2*13, 0, 0, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(2*2, 2*17, 2*18, 0, 0, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(2*203, 2*203, 2*203, 0, 0, 0), 0.0, 0.0);
        check_result!(coupling_3j_e(2*249, 2*248, 2*2, 2*5, 2*(-6), 2*1), 0.0228787564223517967033998, TEST_TOL3);
        check_result!(coupling_3j_e(2*248, 2*247, 2*2, 2*5, 2*(-6), 2*1), -0.022926660587726369939271424097, TEST_TOL3);
    }

    #[test]
    fn test_coupling_6j_e() {
        check_result!(coupling_6j_e(2, 2, 4, 2, 2, 2), 1.0/6.0, TEST_TOL0);
        check_result!(coupling_6j_e(4, 4, 2, 4, 4, 4), -1.0/10.0, TEST_TOL0);
        check_result!(coupling_6j_e(4, 4, 2, 4, 4, 2), 1.0/6.0, TEST_TOL0);
        check_result!(coupling_6j_e(4, 4, 2, 2, 2, 2), -0.5/sqrt(5.0), TEST_TOL0);
        check_result!(coupling_6j_e(4, 4, 4, 2, 2, 2), sqrt(7.0/3.0)/10.0, TEST_TOL0);
        check_result!(coupling_6j_e(1, 0, 1, 1, 0, 1), -1.0/2.0, TEST_TOL0);
        check_result!(coupling_6j_e(2, 2, 4, 2, 2, 7), 0.0, 0.0);
        check_result!(coupling_6j_e(2, 2, 4, 2, 7, 2), 0.0, 0.0);
        check_result!(coupling_6j_e(2, 2, 4, 7, 2, 2), 0.0, 0.0);
        check_result!(coupling_6j_e(2, 2, 7, 2, 2, 2), 0.0, 0.0);
        check_result!(coupling_6j_e(2, 7, 4, 2, 2, 2), 0.0, 0.0);
        check_result!(coupling_6j_e(7, 2, 4, 2, 2, 2), 0.0, 0.0);
        check_result!(coupling_6j_e(0, 2, 2, 44, 43, 43), 0.0, 0.0);
        check_result!(coupling_6j_e(1, 1, 1, 0, 1, 1), 0.0, 0.0);
        check_result!(coupling_6j_e(1, 1, 1, 1, 0, 1), 0.0, 0.0);
        check_result!(coupling_6j_e(1, 1, 1, 1, 1, 0), 0.0, 0.0);
    }

    #[test]
    fn test_coupling_9j_e() {
        check_result!(coupling_9j_e(8, 4, 10, 7, 3, 8, 1, 1, 2), sqrt(7.0/3.0)/60.0, TEST_TOL2);
        check_result!(coupling_9j_e(10, 2, 4, 3, 3, 2, 1, 1, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 10, 4, 3, 3, 2, 1, 1, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 2, 10, 3, 3, 2, 1, 1, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 2, 4, 10, 3, 2, 1, 1, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 2, 4, 3, 10, 2, 1, 1, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 2, 4, 3, 3, 10, 1, 1, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 2, 4, 3, 3, 2, 10, 1, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 2, 4, 3, 3, 2, 1, 10, 2), 0.0, 0.0);
        check_result!(coupling_9j_e(4, 2, 4, 3, 3, 2, 1, 1, 10), 0.0, 0.0);
        check_result!(coupling_9j_e(1, 1, 1, 1, 1, 1, 0, 0, 0), 0.0, 0.0);
        check_result!(coupling_9j_e(1, 1, 0, 1, 1, 0, 1, 1, 0), 0.0, 0.0);
    }
}