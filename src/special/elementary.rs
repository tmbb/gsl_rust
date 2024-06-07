/*
    special/elementary.rs
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
/// This function multiplies `x` and `y` storing the product and its
/// associated error in `result`.
///
/// Binds the [`gsl_sf_multiply_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_multiply_e) function.
pub fn multiply(x: f64, y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_multiply_e(x, y, &mut result))?;
        Ok(result.into())
    }
}

pub fn multiply_err(x: f64, dx: f64, y: f64, dy: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_multiply_err_e(x, dx, y, dy, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {}