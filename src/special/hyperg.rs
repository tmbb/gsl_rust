/*
    special/hyperg.rs
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
pub fn hyperg_0F1(c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_0F1_e(c, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_1F1_int(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_1F1_int_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_1F1(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_1F1_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_U_int(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_U_int_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_U(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_U_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_2F1(a: f64, b: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_2F1_e(a, b, c, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_2F1_conj(ar: f64, ai: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_2F1_conj_e(ar, ai, c, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_2F1_renorm(a: f64, b: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_2F1_renorm_e(a, b, c, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_2F1_conj_renorm(ar: f64, ai: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_2F1_conj_renorm_e(ar, ai, c, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn hyperg_2F0(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hyperg_2F0_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}
