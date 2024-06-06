/*
    special/coulomb.rs
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
/// These routines compute the lowest-order normalized hydrogenic bound
/// state radial wavefunction
/// $R\_1 := 2Z \sqrt{Z} \exp(-Z r)$.
///
/// Binds the [`gsl_sf_hydrogenicR_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hydrogenicR_1_e).
#[allow(non_snake_case)]
pub fn hydrogenicR_1(z: f64, r: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hydrogenicR_1_e(z, r, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the `n`-th normalized hydrogenic bound state
/// radial wavefunction,
/// $R\_n := {2 Z^{3/2} \over n^2} \left({2Z r \over n}\right)^l \sqrt{(n-l-1)! \over (n+l)!} \exp(-Z r/n) L^{2l+1}\_{n-l-1}(2Z r / n).$
/// where $L^a\_b(x)$ is the generalized Laguerre polynomial.
/// The normalization is chosen such that the wavefunction $\psi$ is
/// given by $\psi(n,l,r) = R\_n Y\_{lm}$.
///
/// Binds the [`gsl_sf_hydrogenicR_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hydrogenicR_e).
#[allow(non_snake_case)]
pub fn hydrogenicR(n: i32, l: i32, z: f64, r: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hydrogenicR_e(n, l, z, r, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn coulomb_CL(l: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_coulomb_CL_e(l, eta, &mut result))?;
        Ok(result.into())
    }
}
