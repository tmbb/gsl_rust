/*
    special/pow_int.rs
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
/// These routines compute the power $x^n$ for integer `n`. The
/// power is computed using the minimum number of multiplications. For
/// example, $x^8$ is computed as $((x^2)^2)^2$, requiring only 3
/// multiplications. For reasons of efficiency, these functions do not
/// check for overflow or underflow conditions. The following is a simple example:
/// ```
/// #include <gsl/gsl_sf_pow_int.h>
/// /* compute 3.0**12 */
/// double y = gsl_sf_pow_int(3.0, 12);
/// ```
///
/// Binds the [`gsl_sf_pow_int_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_pow_int_e) function.
pub fn pow_int(x: f64, n: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_pow_int_e(x, n, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {}