/*
    special/erf.rs
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
/// These routines compute the complementary error function
/// $\erfc(x) = 1 - \erf(x) = (2/\sqrt{\pi}) \int\_x^\infty \exp(-t^2)$
///
/// Binds the [`gsl_sf_erfc_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_erfc_e).
pub fn erfc(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_erfc_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the complementary error function
/// $\log(\erfc(x))$.
///
/// Binds the [`gsl_sf_log_erfc_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_log_erfc_e).
pub fn log_erfc(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_log_erfc_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the error function $\erf(x)$,
/// where
/// $\erf(x) = (2/\sqrt{\pi}) \int\_0^x dt \exp(-t^2)$.
///
/// Binds the [`gsl_sf_erf_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_erf_e).
pub fn erf(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_erf_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Gaussian probability density function
/// $Z(x) = (1/\sqrt{2\pi}) \exp(-x^2/2)$
///
/// Binds the [`gsl_sf_erf_Z_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_erf_Z_e).
#[allow(non_snake_case)]
pub fn erf_Z(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_erf_Z_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the upper tail of the Gaussian probability function
/// $Q(x) = (1/\sqrt{2\pi}) \int\_x^\infty dt \exp(-t^2/2)$
///
/// Binds the [`gsl_sf_erf_Q_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_erf_Q_e).
#[allow(non_snake_case)]
pub fn erf_Q(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_erf_Q_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the hazard function for the normal distribution.
///
/// Binds the [`gsl_sf_hazard_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hazard_e).
pub fn hazard(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hazard_e(x, &mut result))?;
        Ok(result.into())
    }
}
