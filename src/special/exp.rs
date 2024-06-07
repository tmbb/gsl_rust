/*
    special/exp.rs
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
/// These routines provide an exponential function $\exp(x)$ using GSL
/// semantics and error checking.
///
/// Binds the [`gsl_sf_exp_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_exp_e) function.
pub fn exp(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exp_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines exponentiate `x` and multiply by the factor `y`
/// to return the product $y \exp(x)$.
///
/// Binds the [`gsl_sf_exp_mult_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_exp_mult_e) function.
pub fn exp_mult(x: f64, y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exp_mult_e(x, y, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the quantity $\exp(x)-1$ using an algorithm
/// that is accurate for small $x$.
///
/// Binds the [`gsl_sf_expm1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_expm1_e) function.
pub fn expm1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expm1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the quantity $(\exp(x)-1)/x$ using an
/// algorithm that is accurate for small `x`. For small `x` the
/// algorithm is based on the expansion
/// $(\exp(x)-1)/x = 1 + x/2 + x^2/(2\*3) + x^3/(2\*3\*4) + \dots$.
///
/// Binds the [`gsl_sf_exprel_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_exprel_e) function.
pub fn exprel(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exprel_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the quantity $2(\exp(x)-1-x)/x^2$ using an
/// algorithm that is accurate for small `x`. For small `x` the
/// algorithm is based on the expansion
/// $2(\exp(x)-1-x)/x^2 = 1 + x/3 + x^2/(3\*4) + x^3/(3\*4\*5) + \dots$.
///
/// Binds the [`gsl_sf_exprel_2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_exprel_2_e) function.
pub fn exprel_2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exprel_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the $N$-relative exponential, which is the
/// `n`-th generalization of the functions `gsl_sf_exprel()` and
/// `gsl_sf_exprel_2()`. The $N$-relative exponential is given by,
/// $\hbox{exprel}\_N(x)
///  &= N!/x^N \left(\exp(x) - \sum\_{k=0}^{N-1} x^k/k!\right)\cr
///  &= 1 + x/(N+1) + x^2/((N+1)(N+2)) + \dots\cr
///  &= {}\_1F\_1(1,1+N,x)\cr$
///
/// Binds the [`gsl_sf_exprel_n_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_exprel_n_e) function.
pub fn exprel_n(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exprel_n_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn exprel_n_CF(n: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exprel_n_CF_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn exp_err(x: f64, dx: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exp_err_e(x, dx, &mut result))?;
        Ok(result.into())
    }
}

pub fn exp_mult_err(x: f64, dx: f64, y: f64, dy: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exp_mult_err_e(x, dx, y, dy, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {}