/*
    special/psi.rs
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
/// These routines compute the digamma function $\psi(n)$ for positive
/// integer `n`. The digamma function is also called the Psi function.
///
/// Binds the [`gsl_sf_psi_int_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_psi_int_e) function.
pub fn psi_int(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_psi_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the digamma function $\psi(x)$ for general
/// `x`, $x \ne 0$.
///
/// Binds the [`gsl_sf_psi_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_psi_e) function.
pub fn psi(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_psi_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the real part of the digamma function on the line
/// $1 + i y$, $\Re[\psi(1 + i y)]$.
///
/// Binds the [`gsl_sf_psi_1piy_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_psi_1piy_e) function.
pub fn psi_1piy(y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_psi_1piy_e(y, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Trigamma function $\psi'(n)$ for
/// positive integer $n$.
///
/// Binds the [`gsl_sf_psi_1_int_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_psi_1_int_e) function.
pub fn psi_1_int(n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_psi_1_int_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Trigamma function $\psi'(x)$ for
/// general `x`.
///
/// Binds the [`gsl_sf_psi_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_psi_1_e) function.
pub fn psi_1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_psi_1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the polygamma function $\psi^{(n)}(x)$ for
/// $n \ge 0$, $x > 0$.
///
/// Binds the [`gsl_sf_psi_n_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_psi_n_e) function.
pub fn psi_n(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_psi_n_e(n, x, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {}