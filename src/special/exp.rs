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


pub fn exp(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exp_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn exp_mult(x: f64, y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exp_mult_e(x, y, &mut result))?;
        Ok(result.into())
    }
}

pub fn expm1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expm1_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn exprel(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exprel_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn exprel_2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_exprel_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

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
