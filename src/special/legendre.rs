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


#[allow(non_snake_case)]
pub fn legendre_Pl(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Pl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_P1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_P1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_P2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_P2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_P3(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_P3_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_Q0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Q0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_Q1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Q1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_Ql(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Ql_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_Plm(l: i32, m: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_Plm_e(l, m, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_sphPlm(l: i32, m: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_sphPlm_e(l, m, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn conicalP_half(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_half_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn conicalP_mhalf(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_mhalf_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn conicalP_0(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_0_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn conicalP_1(lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_1_e(lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn conicalP_sph_reg(l: i32, lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_sph_reg_e(l, lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn conicalP_cyl_reg(m: i32, lambda: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_conicalP_cyl_reg_e(m, lambda, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_H3d_0(lambda: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_H3d_0_e(lambda, eta, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_H3d_1(lambda: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_H3d_1_e(lambda, eta, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn legendre_H3d(l: i32, lambda: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_legendre_H3d_e(l, lambda, eta, &mut result))?;
        Ok(result.into())
    }
}
