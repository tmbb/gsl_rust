/*
    special/legendre.rs
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
/// These functions evaluate the Legendre polynomial $P\_l(x)$
/// for a specific value of `l`, `x` subject to $l \ge 0$ and
/// $|x| \le 1$.
///
/// Binds the [`gsl_sf_legendre_Pl_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_Pl_e).
#[allow(non_snake_case)]
pub fn legendre_Pl(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Pl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These functions evaluate the Legendre polynomials
/// $P\_l(x)$ using explicit representations for $l = 1, 2, 3$.
///
/// Binds the [`gsl_sf_legendre_P1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_P1_e).
#[allow(non_snake_case)]
pub fn legendre_P1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_P1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These functions evaluate the Legendre polynomials
/// $P\_l(x)$ using explicit representations for $l = 1, 2, 3$.
///
/// Binds the [`gsl_sf_legendre_P2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_P2_e).
#[allow(non_snake_case)]
pub fn legendre_P2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_P2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These functions evaluate the Legendre polynomials
/// $P\_l(x)$ using explicit representations for $l = 1, 2, 3$.
///
/// Binds the [`gsl_sf_legendre_P3_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_P3_e).
#[allow(non_snake_case)]
pub fn legendre_P3(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_P3_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Legendre function $Q\_0(x)$ for
/// $x > -1$ and $x \ne 1$.
///
/// Binds the [`gsl_sf_legendre_Q0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_Q0_e).
#[allow(non_snake_case)]
pub fn legendre_Q0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Q0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Legendre function $Q\_1(x)$ for
/// $x > -1$ and $x \ne 1$.
///
/// Binds the [`gsl_sf_legendre_Q1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_Q1_e).
#[allow(non_snake_case)]
pub fn legendre_Q1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Q1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Legendre function $Q\_l(x)$ for
/// $x > -1$, $x \ne 1$ and $l \ge 0$.
///
/// Binds the [`gsl_sf_legendre_Ql_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_Ql_e).
#[allow(non_snake_case)]
pub fn legendre_Ql(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Ql_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the associated Legendre polynomial
/// $P\_l^m(x)$ for $m \ge 0$,
/// $l \ge m$, and $|x| \le 1$.
///
/// Binds the [`gsl_sf_legendre_Plm_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_Plm_e).
#[allow(non_snake_case)]
pub fn legendre_Plm(l: i32, m: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Plm_e(l, m, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the normalized associated Legendre polynomial
/// $\sqrt{(2l+1)/(4\pi)} \sqrt{(l-m)!/(l+m)!} P\_l^m(x)$ suitable
/// for use in spherical harmonics. The parameters must satisfy $m \ge 0$,
/// $l \ge m$, and $|x| \le 1$.
/// These routines avoid the overflows
/// that occur for the standard normalization of $P\_l^m(x)$.
///
/// Binds the [`gsl_sf_legendre_sphPlm_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_sphPlm_e).
#[allow(non_snake_case)]
pub fn legendre_sphPlm(l: i32, m: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_sphPlm_e(l, m, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular Spherical Conical Function
/// $P^{1/2}\_{-1/2 + i \lambda}(x)$ for $x > -1$.
///
/// Binds the [`gsl_sf_conicalP_half_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_conicalP_half_e).
#[allow(non_snake_case)]
pub fn conicalP_half(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_half_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular Spherical Conical Function
/// $P^{-1/2}\_{-1/2 + i \lambda}(x)$ for $x > -1$.
///
/// Binds the [`gsl_sf_conicalP_mhalf_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_conicalP_mhalf_e).
#[allow(non_snake_case)]
pub fn conicalP_mhalf(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_mhalf_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the conical function
/// $P^0\_{-1/2 + i \lambda}(x)$ for $x > -1$.
///
/// Binds the [`gsl_sf_conicalP_0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_conicalP_0_e).
#[allow(non_snake_case)]
pub fn conicalP_0(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_0_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the conical function
/// $P^1\_{-1/2 + i \lambda}(x)$ for $x > -1$.
///
/// Binds the [`gsl_sf_conicalP_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_conicalP_1_e).
#[allow(non_snake_case)]
pub fn conicalP_1(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_1_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Regular Spherical Conical Function
/// $P^{-1/2-l}\_{-1/2 + i \lambda}(x)$
/// for $x > -1$ and $l \ge -1$.
///
/// Binds the [`gsl_sf_conicalP_sph_reg_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_conicalP_sph_reg_e).
#[allow(non_snake_case)]
pub fn conicalP_sph_reg(l: i32, lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_sph_reg_e(l, lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Regular Cylindrical Conical Function
/// $P^{-m}\_{-1/2 + i \lambda}(x)$
/// for $x > -1$ and $m \ge -1$.
///
/// Binds the [`gsl_sf_conicalP_cyl_reg_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_conicalP_cyl_reg_e).
#[allow(non_snake_case)]
pub fn conicalP_cyl_reg(m: i32, lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_cyl_reg_e(m, lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the zeroth radial eigenfunction of the Laplacian on the
/// 3-dimensional hyperbolic space,
/// $L^{H3d}\_0(\lambda,\eta) := {\sin(\lambda\eta) \over \lambda\sinh(\eta)}$
/// for $\eta \ge 0$.
/// In the flat limit this takes the form
/// $L^{H3d}\_0(\lambda,\eta) = j\_0(\lambda\eta)$.
///
/// Binds the [`gsl_sf_legendre_H3d_0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_H3d_0_e).
#[allow(non_snake_case)]
pub fn legendre_H3d_0(lambda: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_H3d_0_e(lambda, eta, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the first radial eigenfunction of the Laplacian on
/// the 3-dimensional hyperbolic space,
/// $L^{H3d}\_1(\lambda,\eta) := {1\over\sqrt{\lambda^2 + 1}} {\left(\sin(\lambda \eta)\over \lambda \sinh(\eta)\right)} \left(\coth(\eta) - \lambda \cot(\lambda\eta)\right)$
/// for $\eta \ge 0$
/// In the flat limit this takes the form
/// $L^{H3d}\_1(\lambda,\eta) = j\_1(\lambda\eta)$.
///
/// Binds the [`gsl_sf_legendre_H3d_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_H3d_1_e).
#[allow(non_snake_case)]
pub fn legendre_H3d_1(lambda: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_H3d_1_e(lambda, eta, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the `l`-th radial eigenfunction of the
/// Laplacian on the 3-dimensional hyperbolic space $\eta \ge 0$ and
/// $l \ge 0$.
/// In the flat limit this takes the form
/// $L^{H3d}\_l(\lambda,\eta) = j\_l(\lambda\eta)$.
///
/// Binds the [`gsl_sf_legendre_H3d_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_legendre_H3d_e).
#[allow(non_snake_case)]
pub fn legendre_H3d(l: i32, lambda: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_H3d_e(l, lambda, eta, &mut result))?;
        Ok(result.into())
    }
}
