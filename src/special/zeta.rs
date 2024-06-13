/*
    special/zeta.rs
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

/// These routines compute the eta function $\eta(n)$ for integer `n`.
/// 
/// Binds the function
/// [`gsl_sf_eta_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_eta_int_e).
pub fn eta_int_e(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_eta_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Hurwitz zeta function $\zeta(s,q)$ for $s > 1$, $q >
/// 0$.
/// 
/// Binds the function
/// [`gsl_sf_hzeta_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hzeta_e).
pub fn hzeta_e(s: f64, q: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hzeta_e(s, q, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the eta function $\eta(s)$ for arbitrary `s`.
/// 
/// Binds the function
/// [`gsl_sf_eta_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_eta_e).
pub fn eta_e(s: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_eta_e(s, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Riemann zeta function $\zeta(n)$ for integer `n`, $n
/// \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zeta_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zeta_int_e).
pub fn zeta_int_e(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_zeta_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Riemann zeta function $\zeta(s)$ for arbitrary `s`,
/// $s \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zeta_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zeta_e).
pub fn zeta_e(s: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_zeta_e(s, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute $\zeta(n) - 1$ for integer `n`, $n \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zetam1_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zetam1_int_e).
pub fn zeta_minus_1_int_e(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_zetam1_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute $\zeta(s) - 1$ for arbitrary `s`, $s \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zetam1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zetam1_e).
pub fn zeta_minus_1_e(s: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_zetam1_e(s, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the eta function $\eta(s)$ for arbitrary `s`.
/// 
/// Binds the function
/// [`gsl_sf_eta`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_eta).
pub fn eta(s: f64) -> f64 {
    unsafe { bindings::gsl_sf_eta(s) }
}

/// These routines compute the eta function $\eta(n)$ for integer `n`.
/// 
/// Binds the function
/// [`gsl_sf_eta_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_eta_int).
pub fn eta_int(n: i32) -> f64 {
    unsafe { bindings::gsl_sf_eta_int(n) }
}

/// These routines compute the Riemann zeta function $\zeta(n)$ for integer `n`, $n
/// \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zeta_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zeta_int).
pub fn zeta_int(n: i32) -> f64 {
    unsafe { bindings::gsl_sf_zeta_int(n) }
}

/// These routines compute $\zeta(n) - 1$ for integer `n`, $n \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zetam1_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zetam1_int).
pub fn zeta_minus_1_int(n: i32) -> f64 {
    unsafe { bindings::gsl_sf_zetam1_int(n) }
}

/// These routines compute the Hurwitz zeta function $\zeta(s,q)$ for $s > 1$, $q >
/// 0$.
/// 
/// Binds the function
/// [`gsl_sf_hzeta`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hzeta).
pub fn hzeta(s: f64, q: f64) -> f64 {
    unsafe { bindings::gsl_sf_hzeta(s, q) }
}

/// These routines compute $\zeta(s) - 1$ for arbitrary `s`, $s \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zetam1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zetam1).
pub fn zeta_minus_1(s: f64) -> f64 {
    unsafe { bindings::gsl_sf_zetam1(s) }
}

/// These routines compute the Riemann zeta function $\zeta(s)$ for arbitrary `s`,
/// $s \ne 1$.
/// 
/// Binds the function
/// [`gsl_sf_zeta`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_zeta).
pub fn zeta(s: f64) -> f64 {
    unsafe { bindings::gsl_sf_zeta(s) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_eta_e() {
        check_result!(eta_e(-51.5), -1.2524184036924703656e+41, TEST_TOL2);
        check_result!(eta_e(-5.0), 0.25, TEST_TOL0);
        check_result!(eta_e(0.5), 0.6048986434216303702, TEST_TOL0);
        check_result!(eta_e(0.999), 0.6929872789683383574, TEST_TOL0);
        check_result!(eta_e(1.0), 0.6931471805599453094, TEST_TOL0);
        check_result!(eta_e(1.0 + 1.0e-10), 0.6931471805759321998, TEST_TOL0);
        check_result!(eta_e(5.0), 0.9721197704469093059, TEST_TOL0);
        check_result!(eta_e(5.2), 0.9755278712546684682, TEST_TOL0);
        check_result!(eta_e(6.0), 0.9855510912974351041, TEST_TOL0);
        check_result!(eta_e(20.0), 0.9999990466115815221, TEST_TOL0);
    }

    #[test]
    fn test_eta_int_e() {
        check_result!(eta_int_e(-91), -4.945598888750002040e+94, TEST_TOL0);
        check_result!(eta_int_e(-51), -4.363969073121683116e+40, TEST_TOL0);
        check_result!(eta_int_e(-5), 0.25, TEST_TOL0);
        check_result!(eta_int_e(-1), 0.25, TEST_TOL0);
        check_result!(eta_int_e(0), 0.5, TEST_TOL0);
        check_result!(eta_int_e(5), 0.9721197704469093059, TEST_TOL0);
        check_result!(eta_int_e(6), 0.9855510912974351041, TEST_TOL0);
        check_result!(eta_int_e(20), 0.9999990466115815221, TEST_TOL0);
        check_result!(eta_int_e(1000), 1.0, TEST_TOL0);
    }

    #[test]
    fn test_hzeta_e() {
        check_result!(hzeta_e(2.0, 1.0), 1.6449340668482264365, TEST_TOL0);
        check_result!(hzeta_e(2.0, 10.0), 0.1051663356816857461, TEST_TOL0);
        check_result!(hzeta_e(5.0, 1.0), 1.0369277551433699263, TEST_TOL0);
        check_result!(hzeta_e(5.0, 10.0), 0.000030413798676470276, TEST_TOL0);
        check_result!(hzeta_e(9.0, 0.1), 1.0000000004253980e+09, TEST_TOL0);
        check_result!(hzeta_e(30.0, 0.5), 1.0737418240000053e+09, TEST_TOL0);
        check_result!(hzeta_e(30.0, 0.9), 2.3589824880264765e+01, TEST_TOL1);
        check_result!(hzeta_e(75.0, 0.25), 1.4272476927059599e+45, TEST_TOL1);
    }

    #[test]
    fn test_zeta_e() {
        check_result!(zeta_e(-151.0), 8.195215221831378294e+143, TEST_TOL2);
        check_result!(zeta_e(-51.0), 9.68995788746359406565e+24, TEST_TOL1);
        check_result!(zeta_e(-5.0), -0.003968253968253968253968, TEST_TOL1);
        check_result!(zeta_e(-8.0), 0.0, TEST_TOL1);
        check_result!(zeta_e(-6.0), 0.0, TEST_TOL1);
        check_result!(zeta_e(-4.0), 0.0, TEST_TOL1);
        check_result!(zeta_e(-3.0), 1.0/120.0, TEST_TOL1);
        check_result!(zeta_e(-2.0), 0.0, TEST_TOL1);
        check_result!(zeta_e(-1.0), -1.0/12.0, TEST_TOL1);
        check_result!(zeta_e(-0.5), -0.207886224977354566017307, TEST_TOL1);
        check_result!(zeta_e(0.0), -0.5, TEST_TOL0);
        check_result!(zeta_e(0.5), -1.460354508809586812889499, TEST_TOL0);
        check_result!(zeta_e(1.0 + 1.0/1024.0), -1023.4228554489429787, TEST_TOL0);
        check_result!(zeta_e(1.0 + 1.0/1048576.0), 1.0485765772157343441e+06, TEST_TOL0);
        check_result!(zeta_e(5.0), 1.036927755143369926331365, TEST_TOL0);
        check_result!(zeta_e(25.5), 1.000000021074106110269959, TEST_TOL0);
    }

    #[test]
    fn test_zeta_int_e() {
        check_result!(zeta_int_e(-61), -3.30660898765775767257e+34, TEST_TOL0);
        check_result!(zeta_int_e(-8), 0.0, TEST_TOL0);
        check_result!(zeta_int_e(-6), 0.0, TEST_TOL0);
        check_result!(zeta_int_e(-5), -0.003968253968253968253968, TEST_TOL0);
        check_result!(zeta_int_e(-4), 0.0, TEST_TOL0);
        check_result!(zeta_int_e(-3), 1.0/120.0, TEST_TOL0);
        check_result!(zeta_int_e(-2), 0.0, TEST_TOL0);
        check_result!(zeta_int_e(-1), -1.0/12.0, TEST_TOL0);
        check_result!(zeta_int_e(5), 1.0369277551433699263313655, TEST_TOL0);
        check_result!(zeta_int_e(31), 1.0000000004656629065033784, TEST_TOL0);
    }

    #[test]
    fn test_zeta_minus_1_e() {
        check_result!(zeta_minus_1_e(-8.0), -1.0, TEST_TOL1);
        check_result!(zeta_minus_1_e(-6.0), -1.0, TEST_TOL1);
        check_result!(zeta_minus_1_e(-4.0), -1.0, TEST_TOL1);
        check_result!(zeta_minus_1_e(-3.0), -119.0/120.0, TEST_TOL1);
        check_result!(zeta_minus_1_e(-2.0), -1.0, TEST_TOL1);
        check_result!(zeta_minus_1_e(-1.0), -13.0/12.0, TEST_TOL1);
        check_result!(zeta_minus_1_e(-0.5), -1.207886224977354566017307, TEST_TOL1);
        check_result!(zeta_minus_1_e(0.0), -1.5, TEST_TOL0);
        check_result!(zeta_minus_1_e(0.5), -2.460354508809586812889499, TEST_TOL0);
        check_result!(zeta_minus_1_e(2.0), 0.64493406684822643647, TEST_TOL1);
        check_result!(zeta_minus_1_e(3.0), 0.20205690315959428540, TEST_TOL1);
        check_result!(zeta_minus_1_e(5.0), 0.0369277551433699263314, TEST_TOL1);
        check_result!(zeta_minus_1_e(9.5), 0.0014125906121736622712, TEST_TOL1);
        check_result!(zeta_minus_1_e(10.5), 0.000700842641736155219500, TEST_TOL1);
        check_result!(zeta_minus_1_e(12.5), 0.000173751733643178193390, TEST_TOL1);
        check_result!(zeta_minus_1_e(13.5), 0.000086686727462338155188, TEST_TOL1);
        check_result!(zeta_minus_1_e(15.5), 0.000021619904246069108133, TEST_TOL1);
        check_result!(zeta_minus_1_e(16.5), 0.000010803124900178547671, TEST_TOL0);
        check_result!(zeta_minus_1_e(25.5), 0.000000021074106110269959, TEST_TOL0);
    }

    #[test]
    fn test_zeta_minus_1_int_e() {
        check_result!(zeta_minus_1_int_e(-61), -3.30660898765775767257e+34, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(-5), -1.003968253968253968253968, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(-8), -1.0, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(-6), -1.0, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(-4), -1.0, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(-3), -119.0/120.0, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(-2), -1.0, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(-1), -13.0/12.0, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(5), 0.0369277551433699263313655, TEST_TOL0);
        check_result!(zeta_minus_1_int_e(31), 0.0000000004656629065033784, TEST_TOL0);
    }
}