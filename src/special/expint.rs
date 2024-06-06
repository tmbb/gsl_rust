/*
    special/expint.rs
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
/// These routines compute the exponential integral $E\_1(x)$,
/// $E\_1(x) := \Re \int\_1^\infty dt \exp(-xt)/t.$
///
/// Binds the [`gsl_sf_expint_E1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_expint_E1_e).
#[allow(non_snake_case)]
pub fn expint_E1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_E1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the second-order exponential integral $E\_2(x)$,
/// $E\_2(x) := \Re \int\_1^\infty dt \exp(-xt)/t^2$
///
/// Binds the [`gsl_sf_expint_E2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_expint_E2_e).
#[allow(non_snake_case)]
pub fn expint_E2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_E2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the exponential integral $E\_n(x)$ of order `n`,
/// $E\_n(x) := \Re \int\_1^\infty dt \exp(-xt)/t^n.$
///
/// Binds the [`gsl_sf_expint_En_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_expint_En_e).
#[allow(non_snake_case)]
pub fn expint_En(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_En_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn expint_E1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_E1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn expint_E2_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_E2_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn expint_En_scaled(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_En_scaled_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the exponential integral $Ei(x)$,
/// $\hbox{Ei}(x) = - PV \left( \int\_{-x}^\infty dt \exp(-t)/t \right)$
/// where $PV$ denotes the principal value of the integral.
///
/// Binds the [`gsl_sf_expint_Ei_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_expint_Ei_e).
#[allow(non_snake_case)]
pub fn expint_Ei(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_Ei_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn expint_Ei_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_Ei_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the integral
/// $\hbox{Shi}(x) = \int\_0^x dt \sinh(t)/t$
///
/// Binds the [`gsl_sf_Shi_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_Shi_e).
#[allow(non_snake_case)]
pub fn Shi(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Shi_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the integral
/// $\hbox{Chi}(x) := \Re \left[ \gamma\_E + \log(x) + \int\_0^x dt (\cosh(t)-1)/t \right]$
/// where $\gamma\_E$ is the Euler constant (available as the macro `M_EULER`).
///
/// Binds the [`gsl_sf_Chi_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_Chi_e).
#[allow(non_snake_case)]
pub fn Chi(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Chi_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the third-order exponential integral
/// ${\rm Ei}\_3(x) = \int\_0^x dt \exp(-t^3)$
/// for $x \ge 0$.
///
/// Binds the [`gsl_sf_expint_3_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_expint_3_e).
pub fn expint_3(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_3_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Sine integral
/// $\hbox{Si}(x) = \int\_0^x dt \sin(t)/t$
///
/// Binds the [`gsl_sf_Si_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_Si_e).
#[allow(non_snake_case)]
pub fn Si(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Si_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Cosine integral
/// $\hbox{Ci}(x) = -\int\_x^\infty dt \cos(t)/t$
/// for $x > 0$
///
/// Binds the [`gsl_sf_Ci_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_Ci_e).
#[allow(non_snake_case)]
pub fn Ci(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Ci_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Arctangent integral, which is defined as
/// $\hbox{AtanInt}(x) = \int\_0^x dt \arctan(t)/t$
///
/// Binds the [`gsl_sf_atanint_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_atanint_e).
pub fn atanint(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_atanint_e(x, &mut result))?;
        Ok(result.into())
    }
}
