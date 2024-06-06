/*
    special/coupling.rs
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
/// These routines compute the Wigner 3-j coefficient,
/// $\left(
/// \begin{array}{ccc}
///  ja & jb & jc \\
///  ma & mb & mc
/// \end{array}
/// \right)$
/// where the arguments are given in half-integer units, $ja$ =
/// `two_ja`/2, $ma$ = `two_ma`/2, etc.
///
/// Binds the [`gsl_sf_coupling_3j_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_coupling_3j_e).
pub fn coupling_3j(two_ja: i32, two_jb: i32, two_jc: i32, two_ma: i32, two_mb: i32, two_mc: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_coupling_3j_e(two_ja, two_jb, two_jc, two_ma, two_mb, two_mc, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Wigner 6-j coefficient,
/// $\left\{
/// \begin{array}{ccc}
///  ja & jb & jc \\
///  jd & je & jf
/// \end{array}
/// \right\}$
/// where the arguments are given in half-integer units, $ja$ =
/// `two_ja`/2, $ma$ = `two_ma`/2, etc.
///
/// Binds the [`gsl_sf_coupling_6j_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_coupling_6j_e).
pub fn coupling_6j(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_coupling_6j_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn coupling_RacahW(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_coupling_RacahW_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Wigner 9-j coefficient,
/// $\left\{
/// \begin{array}{ccc}
///  ja & jb & jc \\
///  jd & je & jf \\
///  jg & jh & ji
/// \end{array}
/// \right\}$
/// where the arguments are given in half-integer units, $ja$ =
/// `two_ja`/2, $ma$ = `two_ma`/2, etc.
///
/// Binds the [`gsl_sf_coupling_9j_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_coupling_9j_e).
pub fn coupling_9j(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32, two_jg: i32, two_jh: i32, two_ji: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_coupling_9j_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, two_jg, two_jh, two_ji, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn coupling_6j_INCORRECT(two_ja: i32, two_jb: i32, two_jc: i32, two_jd: i32, two_je: i32, two_jf: i32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_coupling_6j_INCORRECT_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, &mut result))?;
        Ok(result.into())
    }
}
