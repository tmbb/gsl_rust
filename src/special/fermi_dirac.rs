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


pub fn fermi_dirac_m1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_m1_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_0_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_1_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_int(j: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_int_e(j, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_mhalf(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_mhalf_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_half(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_half_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_3half(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_3half_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn fermi_dirac_inc_0(x: f64, b: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fermi_dirac_inc_0_e(x, b, &mut result))?;
        Ok(result.into())
    }
}
