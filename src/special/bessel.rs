/*
    special/bessel.rs
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
pub fn bessel_J0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_J0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_J1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_J1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Jn(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Jn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Y0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Y0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Y1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Y1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Yn(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Yn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_I0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_I1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_In(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_In_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_I0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_I1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_In_scaled(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_In_scaled_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_K0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_K1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Kn(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Kn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_K0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_K1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Kn_scaled(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Kn_scaled_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_j0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_j0_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_j1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_j1_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_j2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_j2_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_jl(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_jl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_y0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_y0_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_y1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_y1_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_y2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_y2_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_yl(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_yl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_i0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_i0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_i1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_i1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_i2_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_i2_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_il_scaled(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_il_scaled_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_k0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_k0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_k1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_k1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_k2_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_k2_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn bessel_kl_scaled(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_kl_scaled_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Jnu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Jnu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Ynu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Ynu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Inu_scaled(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Inu_scaled_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Inu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Inu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Knu_scaled(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Knu_scaled_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_Knu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Knu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_lnKnu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_lnKnu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_zero_J0(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_zero_J0_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_zero_J1(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_zero_J1_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn bessel_zero_Jnu(nu: f64, s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_zero_Jnu_e(nu, s, &mut result))?;
        Ok(result.into())
    }
}
