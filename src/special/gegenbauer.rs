/*
    special/gegenbauer.rs
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
/// These functions evaluate the Gegenbauer polynomials
/// $C^{(\lambda)}\_n(x)$ using explicit
/// representations for $n = 1, 2, 3$.
///
/// Binds the [`gsl_sf_gegenpoly_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gegenpoly_1_e).
pub fn gegenpoly_1(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gegenpoly_1_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These functions evaluate the Gegenbauer polynomials
/// $C^{(\lambda)}\_n(x)$ using explicit
/// representations for $n = 1, 2, 3$.
///
/// Binds the [`gsl_sf_gegenpoly_2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gegenpoly_2_e).
pub fn gegenpoly_2(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gegenpoly_2_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These functions evaluate the Gegenbauer polynomials
/// $C^{(\lambda)}\_n(x)$ using explicit
/// representations for $n = 1, 2, 3$.
///
/// Binds the [`gsl_sf_gegenpoly_3_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gegenpoly_3_e).
pub fn gegenpoly_3(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gegenpoly_3_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These functions evaluate the Gegenbauer polynomial $C^{(\lambda)}\_n(x)$
/// for a specific value of `n`,
/// `lambda`, `x` subject to $\lambda > -1/2$, $n \ge 0$.
///
/// Binds the [`gsl_sf_gegenpoly_n_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gegenpoly_n_e).
pub fn gegenpoly_n(n: i32, lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gegenpoly_n_e(n, lambda, x, &mut result))?;
        Ok(result.into())
    }
}
