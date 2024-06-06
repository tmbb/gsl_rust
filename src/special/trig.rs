/*
    special/trig.rs
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
/// These routines compute the sine function $\sin(x)$.
///
/// Binds the [`gsl_sf_sin_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_sin_e).
pub fn sin(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_sin_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the cosine function $\cos(x)$.
///
/// Binds the [`gsl_sf_cos_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_cos_e).
pub fn cos(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_cos_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the hypotenuse function $\sqrt{x^2 + y^2}$
/// avoiding overflow and underflow.
///
/// Binds the [`gsl_sf_hypot_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hypot_e).
pub fn hypot(x: f64, y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hypot_e(x, y, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute $\sinc(x) = \sin(\pi x) / (\pi x)$ for any
/// value of `x`.
///
/// Binds the [`gsl_sf_sinc_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_sinc_e).
pub fn sinc(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_sinc_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute $\log(\sinh(x))$ for $x > 0$.
///
/// Binds the [`gsl_sf_lnsinh_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnsinh_e).
pub fn lnsinh(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lnsinh_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute $\log(\cosh(x))$ for any `x`.
///
/// Binds the [`gsl_sf_lncosh_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lncosh_e).
pub fn lncosh(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lncosh_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn sin_err(x: f64, dx: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_sin_err_e(x, dx, &mut result))?;
        Ok(result.into())
    }
}

pub fn cos_err(x: f64, dx: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_cos_err_e(x, dx, &mut result))?;
        Ok(result.into())
    }
}

pub fn angle_restrict_symm_err(theta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_angle_restrict_symm_err_e(theta, &mut result))?;
        Ok(result.into())
    }
}

pub fn angle_restrict_pos_err(theta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_angle_restrict_pos_err_e(theta, &mut result))?;
        Ok(result.into())
    }
}
