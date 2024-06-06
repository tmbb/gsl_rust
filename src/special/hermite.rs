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
/// Binds the [`gsl_sf_hermite_prob_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_prob_e).
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
/// Binds the [`gsl_sf_hermite_prob_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_prob_deriv_e).
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
/// Binds the [`gsl_sf_hermite_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_e).
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
/// Binds the [`gsl_sf_hermite_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_deriv_e).
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
/// Binds the [`gsl_sf_hermite_func_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_e).
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
/// Binds the [`gsl_sf_hermite_func_fast_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_fast_e).
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
/// Binds the [`gsl_sf_hermite_func_der_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_der_e).
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
/// Binds the [`gsl_sf_hermite_prob_zero_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_prob_zero_e).
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
/// Binds the [`gsl_sf_hermite_zero_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_zero_e).
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
/// Binds the [`gsl_sf_hermite_func_zero_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hermite_func_zero_e).
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
