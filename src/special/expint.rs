/*
    special/expint.rs
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

/// These routines compute the integral
/// 
/// $$\hbox{Chi}(x) := \Re \left[ \gamma_E + \log(x) + \int_0^x dt (\cosh(t)-1)/t \right]$$
/// 
/// where $\gamma_E$ is the Euler constant (available as the macro `M_EULER`).
/// 
/// Binds the function [`gsl_sf_Chi_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Chi_e).
pub fn Chi_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_Chi_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the third-order exponential integral
/// 
/// $${\rm Ei}_3(x) = \int_0^x dt \exp(-t^3)$$
/// 
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_sf_expint_3_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_3_e).
pub fn expint_3_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_expint_3_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Arctangent integral, which is defined as
/// 
/// $$\hbox{AtanInt}(x) = \int_0^x dt \arctan(t)/t$$
/// 
/// Binds the function [`gsl_sf_atanint_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_atanint_e).
pub fn atanint_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_atanint_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the second-order exponential integral $E_2(x)$,
/// 
/// $$E_2(x) := \Re \int_1^\infty dt \exp(-xt)/t^2$$
/// 
/// Binds the function [`gsl_sf_expint_E2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_E2_e).
pub fn expint_E2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_expint_E2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Cosine integral
/// 
/// $$\hbox{Ci}(x) = -\int_x^\infty dt \cos(t)/t$$
/// 
/// for $x > 0$
/// 
/// Binds the function [`gsl_sf_Ci_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Ci_e).
pub fn Ci_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_Ci_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the exponential integral $E_1(x)$,
/// 
/// $$E_1(x) := \Re \int_1^\infty dt \exp(-xt)/t.$$
/// 
/// Binds the function [`gsl_sf_expint_E1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_E1_e).
pub fn expint_E1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_expint_E1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the integral
/// 
/// $$\hbox{Shi}(x) = \int_0^x dt \sinh(t)/t$$
/// 
/// Binds the function [`gsl_sf_Shi_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Shi_e).
pub fn Shi_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_Shi_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Sine integral
/// 
/// $$\hbox{Si}(x) = \int_0^x dt \sin(t)/t$$
/// 
/// Binds the function [`gsl_sf_Si_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Si_e).
pub fn Si_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_Si_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the exponential integral $Ei(x)$,
/// 
/// $$\hbox{Ei}(x) = - PV \left( \int_{-x}^\infty dt \exp(-t)/t \right)$$
/// 
/// where $PV$ denotes the principal value of the integral.
/// 
/// Binds the function [`gsl_sf_expint_Ei_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_Ei_e).
pub fn expint_Ei_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_expint_Ei_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the exponential integral $E_n(x)$ of order `n`,
/// 
/// $$E_n(x) := \Re \int_1^\infty dt \exp(-xt)/t^n.$$
/// 
/// Binds the function [`gsl_sf_expint_En_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_En_e).
pub fn expint_En_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_expint_En_e(n, x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the integral
/// 
/// $$\hbox{Shi}(x) = \int_0^x dt \sinh(t)/t$$
/// 
/// Binds the function [`gsl_sf_Shi`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Shi).
pub fn Shi(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_Shi(x) }
}

/// These routines compute the Sine integral
/// 
/// $$\hbox{Si}(x) = \int_0^x dt \sin(t)/t$$
/// 
/// Binds the function [`gsl_sf_Si`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Si).
pub fn Si(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_Si(x) }
}

/// These routines compute the second-order exponential integral $E_2(x)$,
/// 
/// $$E_2(x) := \Re \int_1^\infty dt \exp(-xt)/t^2$$
/// 
/// Binds the function [`gsl_sf_expint_E2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_E2).
pub fn expint_E2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_expint_E2(x) }
}

/// These routines compute the third-order exponential integral
/// 
/// $${\rm Ei}_3(x) = \int_0^x dt \exp(-t^3)$$
/// 
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_sf_expint_3`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_3).
pub fn expint_3(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_expint_3(x) }
}

/// These routines compute the exponential integral $Ei(x)$,
/// 
/// $$\hbox{Ei}(x) = - PV \left( \int_{-x}^\infty dt \exp(-t)/t \right)$$
/// 
/// where $PV$ denotes the principal value of the integral.
/// 
/// Binds the function [`gsl_sf_expint_Ei`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_Ei).
pub fn expint_Ei(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_expint_Ei(x) }
}

/// These routines compute the Arctangent integral, which is defined as
/// 
/// $$\hbox{AtanInt}(x) = \int_0^x dt \arctan(t)/t$$
/// 
/// Binds the function [`gsl_sf_atanint`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_atanint).
pub fn atanint(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_atanint(x) }
}

/// These routines compute the integral
/// 
/// $$\hbox{Chi}(x) := \Re \left[ \gamma_E + \log(x) + \int_0^x dt (\cosh(t)-1)/t \right]$$
/// 
/// where $\gamma_E$ is the Euler constant (available as the macro `M_EULER`).
/// 
/// Binds the function [`gsl_sf_Chi`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Chi).
pub fn Chi(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_Chi(x) }
}

/// These routines compute the exponential integral $E_1(x)$,
/// 
/// $$E_1(x) := \Re \int_1^\infty dt \exp(-xt)/t.$$
/// 
/// Binds the function [`gsl_sf_expint_E1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_E1).
pub fn expint_E1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_expint_E1(x) }
}

/// These routines compute the Cosine integral
/// 
/// $$\hbox{Ci}(x) = -\int_x^\infty dt \cos(t)/t$$
/// 
/// for $x > 0$
/// 
/// Binds the function [`gsl_sf_Ci`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_Ci).
pub fn Ci(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_Ci(x) }
}

/// These routines compute the exponential integral $E_n(x)$ of order `n`,
/// 
/// $$E_n(x) := \Re \int_1^\infty dt \exp(-xt)/t^n.$$
/// 
/// Binds the function [`gsl_sf_expint_En`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_expint_En).
pub fn expint_En(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_expint_En(n, x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_Chi_e() {
        check_result!(Chi_e(-1.0), 0.8378669409802082409, TEST_TOL0);
        check_result!(Chi_e(1.0/4294967296.0), -21.603494113016717041, TEST_TOL0);
        check_result!(Chi_e(1.0/65536.0), -10.513139223999384429, TEST_TOL0);
        check_result!(Chi_e(1.0/8.0), -1.4983170827635760646, TEST_TOL0);
        check_result!(Chi_e(1.0), 0.8378669409802082409, TEST_TOL0);
        check_result!(Chi_e(10.0), 1246.1144860424544147, TEST_TOL1);
        check_result!(Chi_e(50.0), 5.292818448565845482e+19, TEST_TOL2);
        check_result!(Chi_e(300.0), 3.248241254044332895e+127, TEST_TOL2);
    }

    #[test]
    fn test_Ci_e() {
        check_result!(Ci_e(1.0/4294967296.0), -21.603494113016717041, TEST_TOL0);
        check_result!(Ci_e(1.0/65536.0), -10.513139224115799751, TEST_TOL0);
        check_result!(Ci_e(1.0/8.0), -1.5061295845296396649, TEST_TOL0);
        check_result!(Ci_e(1.0), 0.3374039229009681347, TEST_TOL0);
        check_result!(Ci_e(10.0), -0.04545643300445537263, TEST_TOL0);
        check_result!(Ci_e(50.0), -0.005628386324116305440, TEST_TOL0);
        check_result!(Ci_e(300.0), -0.003332199918592111780, TEST_TOL0);
        check_result!(Ci_e(65536.0), 0.000010560248837656279453, TEST_TOL0);
        check_result!(Ci_e(4294967296.0), -1.0756463261957757485e-10, TEST_SQRT_TOL0);
        check_result!(Ci_e(1099511627776.0), -3.689865584710764214e-13, 1024.0*TEST_SQRT_TOL0);
    }

    #[test]
    fn test_Shi_e() {
        check_result!(Shi_e(-1.0), -1.0572508753757285146, TEST_TOL0);
        check_result!(Shi_e(1.0/4294967296.0), 2.3283064365386962891e-10, TEST_TOL0);
        check_result!(Shi_e(1.0/65536.0), 0.00001525878906269737298, TEST_TOL0);
        check_result!(Shi_e(0.1), 0.1000555722250569955, TEST_TOL0);
        check_result!(Shi_e(1.0), 1.0572508753757285146, TEST_TOL0);
        check_result!(Shi_e(10.0), 1246.1144901994233444, TEST_TOL1);
        check_result!(Shi_e(50.0), 5.292818448565845482e+19, TEST_TOL2);
        check_result!(Shi_e(300.0), 3.248241254044332895e+127, TEST_TOL2);
    }

    #[test]
    fn test_Si_e() {
        check_result!(Si_e(-1.0), -0.9460830703671830149, TEST_TOL0);
        check_result!(Si_e(1.0e-10), 1.0e-10, TEST_TOL0);
        check_result!(Si_e(1.0e-05), 9.999999999944444444e-06, TEST_TOL0);
        check_result!(Si_e(0.1), 0.09994446110827695016, TEST_TOL0);
        check_result!(Si_e(1.0), 0.9460830703671830149, TEST_TOL0);
        check_result!(Si_e(10.0), 1.6583475942188740493, TEST_TOL0);
        check_result!(Si_e(50.0), 1.5516170724859358947, TEST_TOL0);
        check_result!(Si_e(300.0), 1.5708810882137495193, TEST_TOL0);
        check_result!(Si_e(1.0e+20), 1.5707963267948966192, TEST_TOL0);
    }

    #[test]
    fn test_atanint_e() {
        check_result!(atanint_e(1.0e-10), 1.0e-10, TEST_TOL0);
        check_result!(atanint_e(1.0e-05), 9.99999999988888888889e-06, TEST_TOL0);
        check_result!(atanint_e(0.1), 0.09988928686033618404, TEST_TOL0);
        check_result!(atanint_e(1.0), 0.91596559417721901505, TEST_TOL0);
        check_result!(atanint_e(2.0), 1.57601540344632342236, TEST_TOL0);
        check_result!(atanint_e(10.0), 3.71678149306806859029, TEST_TOL0);
        check_result!(atanint_e(50.0), 6.16499047850274874222, TEST_TOL0);
        check_result!(atanint_e(300.0), 8.96281388924518959990, TEST_TOL0);
        check_result!(atanint_e(1.0e+5), 18.084471031038661920, TEST_TOL0);
        check_result!(atanint_e(1.0e+9), 32.552029856869591656, TEST_TOL0);
    }

    #[test]
    fn test_expint_3_e() {
        check_result!(expint_3_e(1.0e-10), 1.0e-10, TEST_TOL0);
        check_result!(expint_3_e(1.0e-05), 9.9999999999999975e-06, TEST_TOL0);
        check_result!(expint_3_e(0.1), 0.09997500714119079665122, TEST_TOL0);
        check_result!(expint_3_e(0.5), 0.48491714311363971332427, TEST_TOL0);
        check_result!(expint_3_e(1.0), 0.80751118213967145285833, TEST_TOL0);
        check_result!(expint_3_e(2.0), 0.89295351429387631138208, TEST_TOL0);
        check_result!(expint_3_e(5.0), 0.89297951156924921121856, TEST_TOL0);
        check_result!(expint_3_e(10.0), 0.89297951156924921121856, TEST_TOL0);
        check_result!(expint_3_e(100.0), 0.89297951156924921121856, TEST_TOL0);
    }

    #[test]
    fn test_expint_E1_e() {
        check_result!(expint_E1_e(-1.0), -1.8951178163559367555, TEST_TOL0);
        check_result!(expint_E1_e(1.0e-10), 22.448635265138923980, TEST_TOL0);
        check_result!(expint_E1_e(1.0e-05), 10.935719800043695615, TEST_TOL0);
        check_result!(expint_E1_e(0.1), 1.82292395841939066610, TEST_TOL0);
        check_result!(expint_E1_e(1.0), 0.21938393439552027368, TEST_TOL0);
        check_result!(expint_E1_e(10.0), 4.156968929685324277e-06, TEST_TOL1);
        check_result!(expint_E1_e(50.0), 3.783264029550459019e-24, TEST_TOL2);
        check_result!(expint_E1_e(300.0), 1.710384276804510115e-133, TEST_TOL2);
    }

    #[test]
    fn test_expint_E2_e() {
        check_result!(expint_E2_e(-1.0), 0.8231640121031084799, TEST_TOL1);
        check_result!(expint_E2_e(0.0), 1.0, TEST_TOL0);
        check_result!(expint_E2_e(1.0/4294967296.0), 0.9999999947372139168, TEST_TOL0);
        check_result!(expint_E2_e(1.0/65536.0), 0.9998243233207178845, TEST_TOL0);
        check_result!(expint_E2_e(0.1), 0.7225450221940205066, TEST_TOL0);
        check_result!(expint_E2_e(1.0), 0.14849550677592204792, TEST_TOL0);
        check_result!(expint_E2_e(10.0), 3.830240465631608762e-06, TEST_TOL1);
        check_result!(expint_E2_e(50.0), 3.711783318868827367e-24, TEST_TOL2);
        check_result!(expint_E2_e(300.0), 1.7047391998483433998e-133, TEST_TOL2);
    }

    #[test]
    fn test_expint_Ei_e() {
        check_result!(expint_Ei_e(-1.0), -0.21938393439552027368, TEST_TOL0);
        check_result!(expint_Ei_e(1.0/4294967296.0), -21.603494112783886397, TEST_TOL0);
        check_result!(expint_Ei_e(1.0), 1.8951178163559367555, TEST_TOL0);
    }

    #[test]
    fn test_expint_En_e() {
        check_result!(expint_En_e(1, -1.0), -1.8951178163559367555, TEST_TOL0);
        check_result!(expint_En_e(1, 1.0e-10), 22.448635265138923980, TEST_TOL0);
        check_result!(expint_En_e(1, 1.0e-05), 10.935719800043695615, TEST_TOL0);
        check_result!(expint_En_e(1, 0.1), 1.82292395841939066610, TEST_TOL0);
        check_result!(expint_En_e(1, 1.0), 0.21938393439552027368, TEST_TOL0);
        check_result!(expint_En_e(1, 10.0), 4.156968929685324277e-06, TEST_TOL1);
        check_result!(expint_En_e(1, 50.0), 3.783264029550459019e-24, TEST_TOL2);
        check_result!(expint_En_e(1, 300.0), 1.710384276804510115e-133, TEST_TOL2);
        check_result!(expint_En_e(2, -1.0), 0.8231640121031084799, TEST_TOL1);
        check_result!(expint_En_e(2, 0.0), 1.0, TEST_TOL0);
        check_result!(expint_En_e(2, 1.0/4294967296.0), 0.9999999947372139168, TEST_TOL0);
        check_result!(expint_En_e(2, 1.0/65536.0), 0.9998243233207178845, TEST_TOL0);
        check_result!(expint_En_e(2, 0.1), 0.7225450221940205066, TEST_TOL0);
        check_result!(expint_En_e(2, 1.0), 0.14849550677592204792, TEST_TOL0);
        check_result!(expint_En_e(2, 10.0), 3.830240465631608762e-06, TEST_TOL1);
        check_result!(expint_En_e(2, 50.0), 3.711783318868827367e-24, TEST_TOL2);
        check_result!(expint_En_e(2, 300.0), 1.7047391998483433998e-133, TEST_TOL2);
        check_result!(expint_En_e(3, 0.0), 0.5, TEST_TOL0);
        check_result!(expint_En_e(3, 1.0/4294967296.0), 0.499999999767169356972, TEST_TOL1);
        check_result!(expint_En_e(3, 1.0/65536.0), 0.4999847426094515610, TEST_TOL0);
        check_result!(expint_En_e(3, 0.1), 0.4162914579082787612543, TEST_TOL0);
        check_result!(expint_En_e(3, 1.0), 0.10969196719776013683858, TEST_TOL1);
        check_result!(expint_En_e(3, 10.0), 0.000003548762553084381959981, TEST_TOL1);
        check_result!(expint_En_e(3, 50.0), 3.6429094264752049812e-24, TEST_TOL2);
        check_result!(expint_En_e(3, 300.0), 1.699131143349179084e-133, TEST_TOL2);
        check_result!(expint_En_e(10, 0.0), 0.111111111111111111, TEST_TOL0);
        check_result!(expint_En_e(10, 1.0/4294967296.0), 0.111111111082007280658, TEST_TOL2);
        check_result!(expint_En_e(10, 1.0/65536.0), 0.11110920377910896018606, TEST_TOL1);
        check_result!(expint_En_e(10, 0.1), 0.099298432000896813567905, TEST_TOL1);
        check_result!(expint_En_e(10, 1.0), 0.036393994031416401634164534, TEST_TOL1);
        check_result!(expint_En_e(10, 10.0), 0.00000232530265702821081778968, TEST_TOL1);
        check_result!(expint_En_e(10, 50.0), 3.223296586749110919572e-24, TEST_TOL2);
        check_result!(expint_En_e(10, 300.0), 1.6608815083360041367294736e-133, TEST_TOL2);
    }
}