/*
    special/debye.rs
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

/// These routines compute the second-order Debye function $D_2(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_2_e).
pub fn debye_2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_debye_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the fifth-order Debye function $D_5(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_5_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_5_e).
pub fn debye_5_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_debye_5_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the sixth-order Debye function $D_6(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_6_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_6_e).
pub fn debye_6_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_debye_6_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the fourth-order Debye function $D_4(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_4_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_4_e).
pub fn debye_4_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_debye_4_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the first-order Debye function $D_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_1_e).
pub fn debye_1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_debye_1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the third-order Debye function $D_3(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_3_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_3_e).
pub fn debye_3_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_debye_3_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the fourth-order Debye function $D_4(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_4`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_4).
pub fn debye_4(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_debye_4(x) }
}

/// These routines compute the second-order Debye function $D_2(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_2).
pub fn debye_2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_debye_2(x) }
}

/// These routines compute the third-order Debye function $D_3(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_3`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_3).
pub fn debye_3(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_debye_3(x) }
}

/// These routines compute the sixth-order Debye function $D_6(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_6`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_6).
pub fn debye_6(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_debye_6(x) }
}

/// These routines compute the first-order Debye function $D_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_1).
pub fn debye_1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_debye_1(x) }
}

/// These routines compute the fifth-order Debye function $D_5(x)$.
/// 
/// Binds the function
/// [`gsl_sf_debye_5`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_debye_5).
pub fn debye_5(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_debye_5(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_debye_1_e() {
        check_result!(debye_1_e(0.1), 0.975277750004723276, TEST_TOL0);
        check_result!(debye_1_e(1.0), 0.777504634112248239, TEST_TOL0);
        check_result!(debye_1_e(10.0), 0.164443465679946027, TEST_TOL0);
    }

    #[test]
    fn test_debye_2_e() {
        check_result!(debye_2_e(0.1), 0.967083287045302664, TEST_TOL0);
        check_result!(debye_2_e(1.0), 0.70787847562782924, TEST_TOL0);
        check_result!(debye_2_e(10.0), 0.0479714980201218708, TEST_TOL0);
    }

    #[test]
    fn test_debye_3_e() {
        check_result!(debye_3_e(0.1), 0.962999940487211048, TEST_TOL0);
        check_result!(debye_3_e(1.0), 0.674415564077814667, TEST_TOL0);
        check_result!(debye_3_e(10.0), 0.0192957656903454886, TEST_TOL0);
    }

    #[test]
    fn test_debye_4_e() {
        check_result!(debye_4_e(0.1), 0.960555486124335944, TEST_TOL0);
        check_result!(debye_4_e(1.0), 0.654874068886737049, TEST_TOL0);
        check_result!(debye_4_e(10.0), 0.00967367556027115896, TEST_TOL0);
    }

    #[test]
    fn test_debye_5_e() {
        check_result!(debye_5_e(0.1), 0.95892849428310568745, TEST_TOL0);
        check_result!(debye_5_e(1.0), 0.6421002580217790246, TEST_TOL0);
        check_result!(debye_5_e(10.0), 0.005701535852992908538, TEST_TOL0);
    }

    #[test]
    fn test_debye_6_e() {
        check_result!(debye_6_e(0.1), 0.95776777382605465878, TEST_TOL0);
        check_result!(debye_6_e(1.0), 0.63311142583495107588, TEST_TOL0);
        check_result!(debye_6_e(10.0), 3.7938493294615955279e-3, TEST_TOL0);
    }
}