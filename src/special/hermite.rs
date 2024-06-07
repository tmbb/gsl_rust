/*
    special/hermite.rs
    Copyright (C) 2021 Pim van den Berg

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


use crate::bindings::*;
use crate::*;


#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the probabilist Hermite polynomial $He\_n(x)$ of order `n` at position `x`.
/// If an overflow is detected, `GSL_EOVRFLW` is returned without calling the error handler.
///
/// Binds the [`gsl_sf_hermite_prob_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_prob_e) function.
pub fn hermite_prob(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_prob_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the `m`-th derivative of the probabilist Hermite polynomial $He\_n(x)$
/// of order `n` at position `x`.
///
/// Binds the [`gsl_sf_hermite_prob_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_prob_deriv_e) function.
pub fn hermite_prob_deriv(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_prob_deriv_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the physicist Hermite polynomial $H\_n(x)$ of order `n` at position `x`.
/// If an overflow is detected, `GSL_EOVRFLW` is returned without calling the error handler.
///
/// Binds the [`gsl_sf_hermite_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_e) function.
pub fn hermite(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the `m`-th derivative of the physicist Hermite polynomial $H\_n(x)$ of order `n`
/// at position `x`.
///
/// Binds the [`gsl_sf_hermite_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_deriv_e) function.
pub fn hermite_deriv(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_deriv_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the Hermite function $\psi\_n(x)$ of order `n` at position `x`
/// using a three term recurrence relation. The algorithm complexity is $O(n)$.
///
/// Binds the [`gsl_sf_hermite_func_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_e) function.
pub fn hermite_func(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_func_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the Hermite function $\psi\_n(x)$ of order `n` at position `x`
/// using a the Cauchy integral algorithm due to Bunck, 2009. The algorithm complexity is $O(\sqrt{n})$.
///
/// Binds the [`gsl_sf_hermite_func_fast_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_fast_e) function.
pub fn hermite_func_fast(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_func_fast_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the `m`-th derivative of the Hermite function $\psi\_n(x)$ of order `n` at position `x`.
///
/// Binds the [`gsl_sf_hermite_func_der_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_der_e) function.
pub fn hermite_func_der(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_func_der_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the `s`-th zero of the probabilist Hermite polynomial $He\_n(x)$ of order `n`.
///
/// Binds the [`gsl_sf_hermite_prob_zero_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_prob_zero_e) function.
pub fn hermite_prob_zero(n: i32, s: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_prob_zero_e(n, s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the `s`-th zero of the physicist Hermite polynomial $H\_n(x)$ of order `n`.
///
/// Binds the [`gsl_sf_hermite_zero_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_zero_e) function.
pub fn hermite_zero(n: i32, s: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_zero_e(n, s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the `s`-th zero of the Hermite function $\psi\_n(x)$ of order `n`.
///
/// Binds the [`gsl_sf_hermite_func_zero_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_zero_e) function.
pub fn hermite_func_zero(n: i32, s: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_func_zero_e(n, s, &mut result))?;
        Ok(result.into())
    }
}

pub fn hermite_phys(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_phys_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn hermite_phys_der(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_phys_der_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn hermite_phys_zero(n: i32, s: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_phys_zero_e(n, s, &mut result))?;
        Ok(result.into())
    }
}

pub fn hermite_prob_der(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hermite_prob_der_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::special::special_function_test::*;
    
    #[test]
    fn test_hermite() {
        disable_error_handler();

        check_result(
            hermite(0, 0.75),
            1.0,
            TEST_TOL0
        );

        check_result(
            hermite(1, 0.75),
            1.5,
            TEST_TOL0
        );

        check_result(
            hermite(25, 0.),
            0.,
            TEST_TOL0
        );

        check_result(
            hermite(28, 0.),
            3.497296636753920000e18,
            TEST_TOL0
        );

        check_result(
            hermite(30, 0.),
            -2.028432049317273600e20,
            TEST_TOL0
        );

        check_result(
            hermite(25, 0.75),
            -9.7029819451106077507781088352e15,
            TEST_TOL0
        );

        check_result(
            hermite(28, 0.75),
            3.7538457078067672096408339776e18,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_hermite_deriv() {
        disable_error_handler();

        check_result(
            hermite_deriv(225, 128, 0.75),
            0.,
            TEST_TOL0
        );

        check_result(
            hermite_deriv(5, 128, 0.75),
            2.89461215568095657569833e132,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_hermite_func() {
        disable_error_handler();

        check_result(
            hermite_func(0, 1.3),
            0.322651504564963746879400858624,
            TEST_TOL0
        );

        check_result(
            hermite_func(1, 1.3),
            0.593187573778613235895531272243,
            TEST_TOL0
        );

        check_result(
            hermite_func(1, -1.3),
            -0.593187573778613235895531272243,
            TEST_TOL0
        );

        check_result(
            hermite_func(27, 0.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            hermite_func(28, 0.0),
            0.290371943657199641200016132937,
            TEST_TOL0
        );

        check_result(
            hermite_func(28, 0.75),
            0.23526280808621240649319140441,
            TEST_TOL1
        );

        check_result(
            hermite_func(200, 0.75),
            -0.13725356483699291817038427801,
            TEST_TOL3
        );

        check_result(
            hermite_func(100028, 0.75),
            -0.02903467369856961147236598086,
            TEST_TOL4
        );

        check_result(
            hermite_func(10000, 60.0),
            0.03162606955427450540143292572,
            TEST_TOL3
        );
    }
    
    #[test]
    fn test_hermite_func_der() {
        disable_error_handler();

        check_result(
            hermite_func_der(0, 28, 0.75),
            0.235262808086212406493191404,
            TEST_TOL1
        );

        check_result(
            hermite_func_der(1, 28, 0.75),
            1.289485094958329643927802330,
            TEST_TOL1
        );

        check_result(
            hermite_func_der(2, 28, 0.75),
            -13.27764473136561269145948989,
            TEST_TOL1
        );

        check_result(
            hermite_func_der(3, 28, 0.75),
            -72.42242083458141066943555691,
            TEST_TOL1
        );

        check_result(
            hermite_func_der(4, 28, 0.75),
            753.6960554274941800190147503,
            TEST_TOL1
        );

        check_result(
            hermite_func_der(5, 28, 0.75),
            4035.32788513029308540826835,
            TEST_TOL1
        );

        check_result(
            hermite_func_der(0, 380, 0.75),
            -0.0400554661321992411631174,
            TEST_TOL0
        );

        check_result(
            hermite_func_der(1, 380, 0.75),
            -4.0417244263030600591206553,
            TEST_TOL1
        );

        check_result(
            hermite_func_der(2, 380, 0.75),
            30.4596785269042604519780923,
            TEST_TOL2
        );

        check_result(
            hermite_func_der(3, 380, 0.75),
            3073.4187352276349348458186556,
            TEST_TOL2
        );
    }
    
    #[test]
    fn test_hermite_func_fast() {
        disable_error_handler();

        check_result(
            hermite_func_fast(0, 1.3),
            0.322651504564963746879400858624,
            TEST_TOL0
        );

        check_result(
            hermite_func_fast(1, 1.3),
            0.593187573778613235895531272243,
            TEST_TOL0
        );

        check_result(
            hermite_func_fast(1, -1.3),
            -0.593187573778613235895531272243,
            TEST_TOL0
        );

        check_result(
            hermite_func_fast(27, 0.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            hermite_func_fast(28, 0.0),
            0.290371943657199641200016132937,
            TEST_TOL0
        );

        check_result(
            hermite_func_fast(28, 0.75),
            0.23526280808621240649319140441,
            TEST_TOL1
        );

        check_result(
            hermite_func_fast(200, 0.75),
            -0.13725356483699291817038427801,
            TEST_TOL3
        );

        check_result(
            hermite_func_fast(100028, 0.75),
            -0.02903467369856961147236598086,
            TEST_TOL5
        );

        check_result(
            hermite_func_fast(10000, 60.0),
            0.03162606955427450540143292572,
            TEST_TOL4
        );
    }
    
    #[test]
    fn test_hermite_prob() {
        disable_error_handler();

        check_result(
            hermite_prob(0, 0.75),
            1.,
            TEST_TOL0
        );

        check_result(
            hermite_prob(1, 0.75),
            0.75,
            TEST_TOL0
        );

        check_result(
            hermite_prob(25, 0.),
            0.,
            TEST_TOL0
        );

        check_result(
            hermite_prob(28, 0.),
            2.13458046676875e14,
            TEST_TOL0
        );

        check_result(
            hermite_prob(30, 0.),
            -6.190283353629375e15,
            TEST_TOL0
        );

        check_result(
            hermite_prob(25, 0.75),
            -1.08128685847680748265939328423e12,
            TEST_TOL0
        );

        check_result(
            hermite_prob(28, 0.75),
            -1.60620252094658918105511125135e14,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_hermite_prob_deriv() {
        disable_error_handler();

        check_result(
            hermite_prob_deriv(225, 128, 0.75),
            0.,
            TEST_TOL0
        );

        check_result(
            hermite_prob_deriv(5, 128, 0.75),
            -3.0288278964712702882066404e112,
            TEST_TOL1
        );
    }
}
