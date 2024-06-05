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


#[allow(non_snake_case)]
pub fn expint_E1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_E1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn expint_E2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_E2_e(x, &mut result))?;
        Ok(result.into())
    }
}

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

#[allow(non_snake_case)]
pub fn Shi(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Shi_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn Chi(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Chi_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn expint_3(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_expint_3_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn Si(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Si_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn Ci(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_Ci_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn atanint(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_atanint_e(x, &mut result))?;
        Ok(result.into())
    }
}
