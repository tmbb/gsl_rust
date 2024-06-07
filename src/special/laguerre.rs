/*
    special/laguerre.rs
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
/// These routines evaluate the generalized Laguerre polynomials
/// $L^a\_1(x)$, $L^a\_2(x)$, $L^a\_3(x)$ using explicit
/// representations.
///
/// Binds the [`gsl_sf_laguerre_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_laguerre_1_e) function.
pub fn laguerre_1(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_laguerre_1_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the generalized Laguerre polynomials
/// $L^a\_1(x)$, $L^a\_2(x)$, $L^a\_3(x)$ using explicit
/// representations.
///
/// Binds the [`gsl_sf_laguerre_2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_laguerre_2_e) function.
pub fn laguerre_2(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_laguerre_2_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the generalized Laguerre polynomials
/// $L^a\_1(x)$, $L^a\_2(x)$, $L^a\_3(x)$ using explicit
/// representations.
///
/// Binds the [`gsl_sf_laguerre_3_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_laguerre_3_e) function.
pub fn laguerre_3(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_laguerre_3_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines evaluate the generalized Laguerre polynomials
/// $L^a\_n(x)$ for $a > -1$,
/// $n \ge 0$.
///
/// Binds the [`gsl_sf_laguerre_n_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_laguerre_n_e) function.
pub fn laguerre_n(n: i32, a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_laguerre_n_e(n, a, x, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {}