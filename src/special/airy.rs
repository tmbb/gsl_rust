/*
    special/airy.rs
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
/// These routines compute the location of the `s`-th zero of the Airy
/// function $Ai(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Ai_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Ai_e).
#[allow(non_snake_case)]
pub fn airy_zero_Ai(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Ai_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th zero of the Airy
/// function $Bi(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Bi_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Bi_e).
#[allow(non_snake_case)]
pub fn airy_zero_Bi(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Bi_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th zero of the Airy
/// function derivative $Ai'(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Ai_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Ai_deriv_e).
#[allow(non_snake_case)]
pub fn airy_zero_Ai_deriv(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Ai_deriv_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th zero of the Airy
/// function derivative $Bi'(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Bi_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Bi_deriv_e).
#[allow(non_snake_case)]
pub fn airy_zero_Bi_deriv(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Bi_deriv_e(s, &mut result))?;
        Ok(result.into())
    }
}
