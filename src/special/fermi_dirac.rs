/*
    special/fermi_dirac.rs
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
/// These routines compute the complete Fermi-Dirac integral with an index of $-1$.
/// This integral is given by
/// $F\_{-1}(x) = e^x / (1 + e^x)$.
///
/// Binds the [`gsl_sf_fermi_dirac_m1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_m1_e) function.
pub fn fermi_dirac_m1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_m1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complete Fermi-Dirac integral with an index of $0$.
/// This integral is given by $F\_0(x) = \ln(1 + e^x)$.
///
/// Binds the [`gsl_sf_fermi_dirac_0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_0_e) function.
pub fn fermi_dirac_0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complete Fermi-Dirac integral with an index of $1$,
/// $F\_1(x) = \int\_0^\infty dt (t /(\exp(t-x)+1))$.
///
/// Binds the [`gsl_sf_fermi_dirac_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_1_e) function.
pub fn fermi_dirac_1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complete Fermi-Dirac integral with an index
/// of $2$,
/// $F\_2(x) = (1/2) \int\_0^\infty dt (t^2 /(\exp(t-x)+1))$.
///
/// Binds the [`gsl_sf_fermi_dirac_2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_2_e) function.
pub fn fermi_dirac_2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complete Fermi-Dirac integral with an integer
/// index of $j$,
/// $F\_j(x) = (1/\Gamma(j+1)) \int\_0^\infty dt (t^j /(\exp(t-x)+1))$.
///
/// Binds the [`gsl_sf_fermi_dirac_int_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_int_e) function.
pub fn fermi_dirac_int(j: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_int_e(j, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complete Fermi-Dirac integral
/// $F\_{-1/2}(x)$.
///
/// Binds the [`gsl_sf_fermi_dirac_mhalf_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_mhalf_e) function.
pub fn fermi_dirac_mhalf(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_mhalf_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complete Fermi-Dirac integral
/// $F\_{1/2}(x)$.
///
/// Binds the [`gsl_sf_fermi_dirac_half_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_half_e) function.
pub fn fermi_dirac_half(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_half_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complete Fermi-Dirac integral
/// $F\_{3/2}(x)$.
///
/// Binds the [`gsl_sf_fermi_dirac_3half_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_3half_e) function.
pub fn fermi_dirac_3half(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_3half_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the incomplete Fermi-Dirac integral with an index
/// of zero,
/// $F\_0(x,b) = \ln(1 + e^{b-x}) - (b-x)$
///
/// Binds the [`gsl_sf_fermi_dirac_inc_0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fermi_dirac_inc_0_e) function.
pub fn fermi_dirac_inc_0(x: f64, b: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_inc_0_e(x, b, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {}