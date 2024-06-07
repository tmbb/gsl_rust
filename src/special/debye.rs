/*
    special/debye.rs
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
/// These routines compute the first-order Debye function $D\_1(x)$.
///
/// Binds the [`gsl_sf_debye_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_debye_1_e) function.
pub fn debye_1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_debye_1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the second-order Debye function $D\_2(x)$.
///
/// Binds the [`gsl_sf_debye_2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_debye_2_e) function.
pub fn debye_2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_debye_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the third-order Debye function $D\_3(x)$.
///
/// Binds the [`gsl_sf_debye_3_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_debye_3_e) function.
pub fn debye_3(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_debye_3_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the fourth-order Debye function $D\_4(x)$.
///
/// Binds the [`gsl_sf_debye_4_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_debye_4_e) function.
pub fn debye_4(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_debye_4_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the fifth-order Debye function $D\_5(x)$.
///
/// Binds the [`gsl_sf_debye_5_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_debye_5_e) function.
pub fn debye_5(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_debye_5_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the sixth-order Debye function $D\_6(x)$.
///
/// Binds the [`gsl_sf_debye_6_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_debye_6_e) function.
pub fn debye_6(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_debye_6_e(x, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {}