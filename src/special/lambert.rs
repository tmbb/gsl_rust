/*
    special/lambert.rs
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

/// These compute the secondary real-valued branch of the Lambert W function,
/// $W_{-1}(x)$.
/// 
/// Binds the function [`gsl_sf_lambert_Wm1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lambert_Wm1_e).
pub fn lambert_Wm1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_lambert_Wm1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These compute the principal branch of the Lambert W function, $W_0(x)$.
/// 
/// Binds the function [`gsl_sf_lambert_W0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lambert_W0_e).
pub fn lambert_W0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_lambert_W0_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These compute the secondary real-valued branch of the Lambert W function,
/// $W_{-1}(x)$.
/// 
/// Binds the function [`gsl_sf_lambert_Wm1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lambert_Wm1).
pub fn lambert_Wm1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_lambert_Wm1(x) }
}

/// These compute the principal branch of the Lambert W function, $W_0(x)$.
/// 
/// Binds the function [`gsl_sf_lambert_W0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lambert_W0).
pub fn lambert_W0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_lambert_W0(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_lambert_W0_e() {
        check_result!(lambert_W0_e(0.0), 0.0, TEST_TOL0);
        check_result!(lambert_W0_e(1.0), 0.567143290409783872999969, TEST_TOL0);
        check_result!(lambert_W0_e(2.0), 0.852605502013725491346472, TEST_TOL0);
        check_result!(lambert_W0_e(20.0), 2.205003278024059970493066, TEST_TOL0);
        check_result!(lambert_W0_e(1000.0), 5.24960285240159622712606, TEST_TOL0);
        check_result!(lambert_W0_e(1.0e+6), 11.38335808614005262200016, TEST_TOL0);
        check_result!(lambert_W0_e(1.0e+12), 24.43500440493491313826305, TEST_TOL0);
        check_result!(lambert_W0_e(1.0e+308), 702.641362034106812081125, TEST_TOL0);
        check_result!(lambert_W0_e(1.6849341956993852953416990), 0.775706963944252869680440, TEST_TOL0);
        check_result!(lambert_W0_e(-1.0/M_E + DBL_EPSILON), -1.0, TEST_TOL0);
        check_result!(lambert_W0_e(-1.0/M_E + 1.0/(1024.0*1024.0*1024.0)), -0.999928845560308370714970, TEST_TOL0);
        check_result!(lambert_W0_e(-1.0/M_E + 1.0/(1024.0*1024.0)), -0.997724730359774141620354, TEST_TOL0);
        check_result!(lambert_W0_e(-1.0/M_E + 1.0/512.0), -0.900335676696088773044678, TEST_TOL0);
        check_result!(lambert_W0_e(-1.0/M_E + 0.25), -0.1349044682661213545487599, TEST_TOL0);
    }

    #[test]
    fn test_lambert_Wm1_e() {
        check_result!(lambert_Wm1_e(0.0), 0.0, TEST_TOL0);
        check_result!(lambert_Wm1_e(1.0), 0.567143290409783872999969, TEST_TOL0);
        check_result!(lambert_Wm1_e(2.0), 0.852605502013725491346472, TEST_TOL0);
        check_result!(lambert_Wm1_e(20.0), 2.205003278024059970493066, TEST_TOL0);
        check_result!(lambert_Wm1_e(-1.0/M_E + DBL_EPSILON), -1.0, TEST_TOL0);
        check_result!(lambert_Wm1_e(-1.0/M_E + 1.0/(1024.0*1024.0*1024.0)), -1.000071157815154608049055, TEST_TOL1);
        check_result!(lambert_Wm1_e(-1.0/M_E + 1.0/(1024.0*1024.0)), -1.002278726118593023934693, TEST_TOL1);
        check_result!(lambert_Wm1_e(-1.0/M_E + 1.0/512.0), -1.106761200865743124599130, TEST_TOL1);
        check_result!(lambert_Wm1_e(-1.0/M_E + 1.0/64.0), -1.324240940341812125489772, TEST_TOL1);
        check_result!(lambert_Wm1_e(-1.0/M_E + 0.25), -3.345798131120112, TEST_TOL1);
    }
}