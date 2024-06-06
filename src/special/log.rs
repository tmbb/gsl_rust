/*
    special/log.rs
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
/// These routines compute the logarithm of `x`, $\log(x)$, for
/// $x > 0$.
///
/// Binds the [`gsl_sf_log_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_log_e).
pub fn log(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_log_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the magnitude of `x`,
/// $\log(|x|)$, for $x \ne 0$.
///
/// Binds the [`gsl_sf_log_abs_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_log_abs_e).
pub fn log_abs(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_log_abs_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute $\log(1 + x)$ for $x > -1$ using an
/// algorithm that is accurate for small `x`.
///
/// Binds the [`gsl_sf_log_1plusx_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_log_1plusx_e).
pub fn log_1plusx(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_log_1plusx_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute $\log(1 + x) - x$ for $x > -1$ using an
/// algorithm that is accurate for small `x`.
///
/// Binds the [`gsl_sf_log_1plusx_mx_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_log_1plusx_mx_e).
pub fn log_1plusx_mx(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_log_1plusx_mx_e(x, &mut result))?;
        Ok(result.into())
    }
}
