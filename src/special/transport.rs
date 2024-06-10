/*
    special/transport.rs
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

/// These routines compute the transport function $J(4,x)$.
/// 
/// Binds the function [`gsl_sf_transport_4_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_4_e).
pub fn transport_4_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_transport_4_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the transport function $J(3,x)$.
/// 
/// Binds the function [`gsl_sf_transport_3_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_3_e).
pub fn transport_3_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_transport_3_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the transport function $J(2,x)$.
/// 
/// Binds the function [`gsl_sf_transport_2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_2_e).
pub fn transport_2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_transport_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the transport function $J(5,x)$.
/// 
/// Binds the function [`gsl_sf_transport_5_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_5_e).
pub fn transport_5_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_transport_5_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the transport function $J(5,x)$.
/// 
/// Binds the function [`gsl_sf_transport_5`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_5).
pub fn transport_5(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_transport_5(x) }
}

/// These routines compute the transport function $J(3,x)$.
/// 
/// Binds the function [`gsl_sf_transport_3`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_3).
pub fn transport_3(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_transport_3(x) }
}

/// These routines compute the transport function $J(4,x)$.
/// 
/// Binds the function [`gsl_sf_transport_4`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_4).
pub fn transport_4(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_transport_4(x) }
}

/// These routines compute the transport function $J(2,x)$.
/// 
/// Binds the function [`gsl_sf_transport_2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_transport_2).
pub fn transport_2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_transport_2(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_transport_2_e() {
        check_result!(transport_2_e(1.0e-10), 9.9999999999999999999e-11, TEST_TOL0);
        check_result!(transport_2_e(1.0), 0.97303256135517012845, TEST_TOL0);
        check_result!(transport_2_e(3.0), 2.41105004901695346199, TEST_TOL0);
        check_result!(transport_2_e(10.0), 3.28432911449795173575, TEST_TOL0);
        check_result!(transport_2_e(100.0), 3.28986813369645287294, TEST_TOL0);
        check_result!(transport_2_e(1.0e+05), 3.28986813369645287294, TEST_TOL0);
    }

    #[test]
    fn test_transport_3_e() {
        check_result!(transport_3_e(1.0e-10), 4.999999999999999999997e-21, TEST_TOL0);
        check_result!(transport_3_e(1.0), 0.479841006572417499939, TEST_TOL0);
        check_result!(transport_3_e(3.0), 3.210604662942246772338, TEST_TOL0);
        check_result!(transport_3_e(5.0), 5.614386613842273228585, TEST_TOL0);
        check_result!(transport_3_e(10.0), 7.150322712008592975030, TEST_TOL0);
        check_result!(transport_3_e(30.0), 7.212341416160946511930, TEST_TOL0);
        check_result!(transport_3_e(100.0), 7.212341418957565712398, TEST_TOL0);
        check_result!(transport_3_e(1.0e+05), 7.212341418957565712398, TEST_TOL0);
    }

    #[test]
    fn test_transport_4_e() {
        check_result!(transport_4_e(1.0e-10), 3.33333333333333333333e-31, TEST_TOL0);
        check_result!(transport_4_e(1.0e-07), 3.33333333333333166666e-22, TEST_TOL0);
        check_result!(transport_4_e(1.0e-04), 3.33333333166666666726e-13, TEST_TOL0);
        check_result!(transport_4_e(0.1), 0.000333166726172109903824, TEST_TOL0);
        check_result!(transport_4_e(1.0), 0.31724404523442648241, TEST_TOL0);
        check_result!(transport_4_e(3.0), 5.96482239737147652446, TEST_TOL0);
        check_result!(transport_4_e(5.0), 15.3597843168821829816, TEST_TOL0);
        check_result!(transport_4_e(10.0), 25.2736676770304417334, TEST_TOL0);
        check_result!(transport_4_e(30.0), 25.9757575220840937469, TEST_TOL0);
        check_result!(transport_4_e(100.0), 25.9757576090673165963, TEST_TOL1);
        check_result!(transport_4_e(1.0e+05), 25.9757576090673165963, TEST_TOL2);
    }

    #[test]
    fn test_transport_5_e() {
        check_result!(transport_5_e(1.0e-10), 2.49999999999999999999e-41, TEST_TOL0);
        check_result!(transport_5_e(1.0e-07), 2.49999999999999861111e-29, TEST_TOL0);
        check_result!(transport_5_e(1.0e-04), 2.49999999861111111163e-17, TEST_TOL0);
        check_result!(transport_5_e(0.1), 0.000024986116317791487410, TEST_TOL0);
        check_result!(transport_5_e(1.0), 0.236615879239094789259153, TEST_TOL0);
        check_result!(transport_5_e(3.0), 12.77055769104415951115760, TEST_TOL0);
        check_result!(transport_5_e(5.0), 50.26309221817518778543615, TEST_TOL0);
        check_result!(transport_5_e(10.0), 116.3807454024207107698556, TEST_TOL0);
        check_result!(transport_5_e(30.0), 124.4313279083858954839911, TEST_TOL0);
        check_result!(transport_5_e(100.0), 124.4313306172043911597639, TEST_TOL0);
        check_result!(transport_5_e(1.0e+05), 124.43133061720439115976, TEST_TOL0);
    }
}