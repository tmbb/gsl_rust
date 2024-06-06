/*
    special/dilog.rs
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
/// These routines compute the dilogarithm for a real argument. In Lewin’s
/// notation this is $Li\_2(x)$, the real part of the dilogarithm of a
/// real $x$. It is defined by the integral representation
/// $Li\_2(x) = - \Re \int\_0^x ds \log(1-s) / s$
/// Note that $\Im(Li\_2(x)) = 0$ for
/// $x \le 1$, and $-\pi\log(x)$ for $x > 1$.
/// Note that Abramowitz & Stegun refer to the Spence integral
/// $S(x) = Li\_2(1 - x)$ as the dilogarithm rather than $Li\_2(x)$.
///
/// Binds the [`gsl_sf_dilog_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_dilog_e).
pub fn dilog(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_dilog_e(x, &mut result))?;
        Ok(result.into())
    }
}
