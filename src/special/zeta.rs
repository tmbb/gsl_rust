/*
    special/zeta.rs
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
/// These routines compute the Riemann zeta function $\zeta(n)$
/// for integer `n`,
/// $n \ne 1$.
///
/// Binds the [`gsl_sf_zeta_int_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_zeta_int_e).
pub fn zeta_int(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_zeta_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Riemann zeta function $\zeta(s)$
/// for arbitrary `s`,
/// $s \ne 1$.
///
/// Binds the [`gsl_sf_zeta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_zeta_e).
pub fn zeta(s: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_zeta_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute $\zeta(s) - 1$ for arbitrary `s`,
/// $s \ne 1$.
///
/// Binds the [`gsl_sf_zetam1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_zetam1_e).
pub fn zetam1(s: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_zetam1_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute $\zeta(n) - 1$ for integer `n`,
/// $n \ne 1$.
///
/// Binds the [`gsl_sf_zetam1_int_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_zetam1_int_e).
pub fn zetam1_int(s: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_zetam1_int_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Hurwitz zeta function $\zeta(s,q)$ for
/// $s > 1$, $q > 0$.
///
/// Binds the [`gsl_sf_hzeta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hzeta_e).
pub fn hzeta(s: f64, q: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hzeta_e(s, q, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the eta function $\eta(n)$ for integer `n`.
///
/// Binds the [`gsl_sf_eta_int_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_eta_int_e).
pub fn eta_int(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_eta_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the eta function $\eta(s)$ for arbitrary `s`.
///
/// Binds the [`gsl_sf_eta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_eta_e).
pub fn eta(s: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_eta_e(s, &mut result))?;
        Ok(result.into())
    }
}
