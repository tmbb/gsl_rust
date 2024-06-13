/*
    special/bessel.rs
    Copyright (C) 2021 Pim van den Berg
    Copyright (C) 2024 Tiago Barroso

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

#![allow(non_snake_case)]

use crate::bindings;
use crate::{GSLError, ValWithError, Result};

/// These routines compute the irregular cylindrical Bessel function of zeroth
/// order, $Y_0(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Y0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Y0_e).
pub fn bessel_Y0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Y0_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified cylindrical Bessel function
/// of first order $\exp(x) K_1(x)$ for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K1_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K1_scaled_e).
pub fn bessel_K1_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_K1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// second order, $\exp(-|x|) i_2(x)$
/// 
/// Binds the function
/// [`gsl_sf_bessel_i2_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_i2_scaled_e).
pub fn bessel_i2_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_i2_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular cylindrical Bessel function of fractional
/// order $\nu$, $Y_\nu(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Ynu_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Ynu_e).
pub fn bessel_Ynu_e(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Ynu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular spherical Bessel function of first order,
/// $y_1(x) = -(\cos(x)/x + \sin(x))/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_y1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_y1_e).
pub fn bessel_y1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_y1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular spherical Bessel function of second order,
/// $y_2(x) = (-3/x^3 + 1/x)\cos(x) - (3/x^2)\sin(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_y2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_y2_e).
pub fn bessel_y2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_y2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular modified cylindrical Bessel function of order
/// `n`, $I_n(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_In_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_In_e).
pub fn bessel_In_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_In_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular modified cylindrical Bessel function of
/// first order, $K_1(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K1_e).
pub fn bessel_K1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_K1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular modified cylindrical Bessel function of
/// order `n`, $K_n(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Kn_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Kn_e).
pub fn bessel_Kn_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Kn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular spherical Bessel function of order `l`,
/// $j_l(x)$, for $l \geq 0$ and $x \geq 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_jl_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_jl_e).
pub fn bessel_jl_e(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_jl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified cylindrical Bessel function
/// of first order $\exp(-|x|) I_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I1_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I1_scaled_e).
pub fn bessel_I1_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_I1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular modified Bessel function of fractional
/// order $\nu$, $K_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Knu_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Knu_e).
pub fn bessel_Knu_e(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Knu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// zeroth order, $\exp(-|x|) i_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_i0_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_i0_scaled_e).
pub fn bessel_i0_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_i0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified cylindrical Bessel function
/// of order `n`, $\exp(-|x|) I_n(x)$
/// 
/// Binds the function
/// [`gsl_sf_bessel_In_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_In_scaled_e).
pub fn bessel_In_scaled_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_In_scaled_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of second order, $\exp(x) k_2(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_k2_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_k2_scaled_e).
pub fn bessel_k2_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_k2_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the location of the `s`-th positive zero of the Bessel
/// function $J_\nu(x)$.
/// 
/// The current implementation does not support negative values of `nu`.
/// 
/// Binds the function
/// [`gsl_sf_bessel_zero_Jnu_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_zero_Jnu_e).
pub fn bessel_zero_Jnu_e(nu: f64, s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_zero_Jnu_e(nu, s, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified cylindrical Bessel function
/// of zeroth order $\exp(-|x|) I_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I0_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I0_scaled_e).
pub fn bessel_I0_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_I0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular spherical Bessel function of order `l`,
/// $y_l(x)$, for $l \geq 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_yl_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_yl_e).
pub fn bessel_yl_e(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_yl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular cylindrical Bessel function of first order,
/// $J_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_J1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_J1_e).
pub fn bessel_J1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_J1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified Bessel function of fractional
/// order $\nu$, $\exp(-|x|)I_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Inu_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Inu_scaled_e).
pub fn bessel_Inu_scaled_e(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Inu_scaled_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular cylindrical Bessel function of fractional
/// order $\nu$, $J_\nu(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Jnu_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Jnu_e).
pub fn bessel_Jnu_e(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Jnu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the location of the `s`-th positive zero of the Bessel
/// function $J_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_zero_J1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_zero_J1_e).
pub fn bessel_zero_J1_e(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_zero_J1_e(s, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular cylindrical Bessel function of zeroth order,
/// $J_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_J0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_J0_e).
pub fn bessel_J0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_J0_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified cylindrical Bessel function
/// of order `n`, $\exp(x) K_n(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Kn_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Kn_scaled_e).
pub fn bessel_Kn_scaled_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Kn_scaled_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// order `l`, $\exp(-|x|) i_l(x)$
/// 
/// Binds the function
/// [`gsl_sf_bessel_il_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_il_scaled_e).
pub fn bessel_il_scaled_e(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_il_scaled_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of order `l`, $\exp(x) k_l(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_kl_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_kl_scaled_e).
pub fn bessel_kl_scaled_e(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_kl_scaled_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the location of the `s`-th positive zero of the Bessel
/// function $J_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_zero_J0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_zero_J0_e).
pub fn bessel_zero_J0_e(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_zero_J0_e(s, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of first order, $\exp(x) k_1(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_k1_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_k1_scaled_e).
pub fn bessel_k1_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_k1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the logarithm of the irregular modified Bessel function
/// of fractional order $\nu$, $\ln(K_\nu(x))$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_lnKnu_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_lnKnu_e).
pub fn bessel_ln_Knu_e(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_lnKnu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified Bessel function of
/// fractional order $\nu$, $\exp(+|x|) K_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Knu_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Knu_scaled_e).
pub fn bessel_Knu_scaled_e(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Knu_scaled_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular spherical Bessel function of zeroth order,
/// $y_0(x) = -\cos(x)/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_y0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_y0_e).
pub fn bessel_y0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_y0_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified cylindrical Bessel function
/// of zeroth order $\exp(x) K_0(x)$ for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K0_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K0_scaled_e).
pub fn bessel_K0_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_K0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular modified cylindrical Bessel function of first
/// order, $I_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I1_e).
pub fn bessel_I1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_I1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular cylindrical Bessel function of order `n`,
/// $J_n(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Jn_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Jn_e).
pub fn bessel_Jn_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Jn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular modified cylindrical Bessel function of
/// zeroth order, $I_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I0_e).
pub fn bessel_I0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_I0_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular spherical Bessel function of second order,
/// $j_2(x) = ((3/x^2 - 1)\sin(x) - 3\cos(x)/x)/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_j2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_j2_e).
pub fn bessel_j2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_j2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular modified cylindrical Bessel function of
/// zeroth order, $K_0(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K0_e).
pub fn bessel_K0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_K0_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// first order, $\exp(-|x|) i_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_i1_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_i1_scaled_e).
pub fn bessel_i1_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_i1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular spherical Bessel function of zeroth order,
/// $j_0(x) = \sin(x)/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_j0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_j0_e).
pub fn bessel_j0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_j0_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular modified Bessel function of fractional order
/// $\nu$, $I_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Inu_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Inu_e).
pub fn bessel_Inu_e(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Inu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the regular spherical Bessel function of first order,
/// $j_1(x) = (\sin(x)/x - \cos(x))/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_j1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_j1_e).
pub fn bessel_j1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_j1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of zeroth order, $\exp(x) k_0(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_k0_scaled_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_k0_scaled_e).
pub fn bessel_k0_scaled_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_k0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular cylindrical Bessel function of order `n`,
/// $Y_n(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Yn_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Yn_e).
pub fn bessel_Yn_e(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Yn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the irregular cylindrical Bessel function of first order,
/// $Y_1(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Y1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Y1_e).
pub fn bessel_Y1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_bessel_Y1_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the scaled irregular modified cylindrical Bessel function
/// of order `n`, $\exp(x) K_n(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Kn_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Kn_scaled).
pub fn bessel_Kn_scaled(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Kn_scaled(n, x) }
}

/// These routines compute the irregular modified Bessel function of fractional
/// order $\nu$, $K_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Knu`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Knu).
pub fn bessel_Knu(nu: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Knu(nu, x) }
}

/// These routines compute the regular spherical Bessel function of second order,
/// $j_2(x) = ((3/x^2 - 1)\sin(x) - 3\cos(x)/x)/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_j2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_j2).
pub fn bessel_j2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_j2(x) }
}

/// These routines compute the regular spherical Bessel function of order `l`,
/// $j_l(x)$, for $l \geq 0$ and $x \geq 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_jl`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_jl).
pub fn bessel_jl(l: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_jl(l, x) }
}

/// These routines compute the irregular modified cylindrical Bessel function of
/// first order, $K_1(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K1).
pub fn bessel_K1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_K1(x) }
}

/// These routines compute the scaled regular modified cylindrical Bessel function
/// of order `n`, $\exp(-|x|) I_n(x)$
/// 
/// Binds the function
/// [`gsl_sf_bessel_In_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_In_scaled).
pub fn bessel_In_scaled(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_In_scaled(n, x) }
}

/// These routines compute the scaled regular modified cylindrical Bessel function
/// of zeroth order $\exp(-|x|) I_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I0_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I0_scaled).
pub fn bessel_I0_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_I0_scaled(x) }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of first order, $\exp(x) k_1(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_k1_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_k1_scaled).
pub fn bessel_k1_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_k1_scaled(x) }
}

/// These routines compute the regular spherical Bessel function of zeroth order,
/// $j_0(x) = \sin(x)/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_j0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_j0).
pub fn bessel_j0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_j0(x) }
}

/// These routines compute the scaled regular modified Bessel function of fractional
/// order $\nu$, $\exp(-|x|)I_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Inu_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Inu_scaled).
pub fn bessel_Inu_scaled(nu: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Inu_scaled(nu, x) }
}

/// These routines compute the regular cylindrical Bessel function of fractional
/// order $\nu$, $J_\nu(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Jnu`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Jnu).
pub fn bessel_Jnu(nu: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Jnu(nu, x) }
}

/// These routines compute the irregular cylindrical Bessel function of first order,
/// $Y_1(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Y1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Y1).
pub fn bessel_Y1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Y1(x) }
}

/// These routines compute the irregular cylindrical Bessel function of order `n`,
/// $Y_n(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Yn`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Yn).
pub fn bessel_Yn(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Yn(n, x) }
}

/// These routines compute the scaled irregular modified Bessel function of
/// fractional order $\nu$, $\exp(+|x|) K_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Knu_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Knu_scaled).
pub fn bessel_Knu_scaled(nu: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Knu_scaled(nu, x) }
}

/// These routines compute the regular spherical Bessel function of first order,
/// $j_1(x) = (\sin(x)/x - \cos(x))/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_j1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_j1).
pub fn bessel_j1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_j1(x) }
}

/// These routines compute the location of the `s`-th positive zero of the Bessel
/// function $J_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_zero_J1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_zero_J1).
pub fn bessel_zero_J1(s: u32) -> f64 {
    unsafe { bindings::gsl_sf_bessel_zero_J1(s) }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// second order, $\exp(-|x|) i_2(x)$
/// 
/// Binds the function
/// [`gsl_sf_bessel_i2_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_i2_scaled).
pub fn bessel_i2_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_i2_scaled(x) }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// zeroth order, $\exp(-|x|) i_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_i0_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_i0_scaled).
pub fn bessel_i0_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_i0_scaled(x) }
}

/// These routines compute the location of the `s`-th positive zero of the Bessel
/// function $J_\nu(x)$.
/// 
/// The current implementation does not support negative values of `nu`.
/// 
/// Binds the function
/// [`gsl_sf_bessel_zero_Jnu`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_zero_Jnu).
pub fn bessel_zero_Jnu(nu: f64, s: u32) -> f64 {
    unsafe { bindings::gsl_sf_bessel_zero_Jnu(nu, s) }
}

/// These routines compute the irregular spherical Bessel function of zeroth order,
/// $y_0(x) = -\cos(x)/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_y0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_y0).
pub fn bessel_y0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_y0(x) }
}

/// These routines compute the irregular spherical Bessel function of first order,
/// $y_1(x) = -(\cos(x)/x + \sin(x))/x$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_y1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_y1).
pub fn bessel_y1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_y1(x) }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of second order, $\exp(x) k_2(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_k2_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_k2_scaled).
pub fn bessel_k2_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_k2_scaled(x) }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// order `l`, $\exp(-|x|) i_l(x)$
/// 
/// Binds the function
/// [`gsl_sf_bessel_il_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_il_scaled).
pub fn bessel_il_scaled(l: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_il_scaled(l, x) }
}

/// These routines compute the irregular cylindrical Bessel function of zeroth
/// order, $Y_0(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Y0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Y0).
pub fn bessel_Y0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Y0(x) }
}

/// These routines compute the regular cylindrical Bessel function of order `n`,
/// $J_n(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Jn`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Jn).
pub fn bessel_Jn(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Jn(n, x) }
}

/// These routines compute the scaled irregular modified cylindrical Bessel function
/// of zeroth order $\exp(x) K_0(x)$ for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K0_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K0_scaled).
pub fn bessel_K0_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_K0_scaled(x) }
}

/// These routines compute the scaled regular modified spherical Bessel function of
/// first order, $\exp(-|x|) i_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_i1_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_i1_scaled).
pub fn bessel_i1_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_i1_scaled(x) }
}

/// These routines compute the location of the `s`-th positive zero of the Bessel
/// function $J_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_zero_J0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_zero_J0).
pub fn bessel_zero_J0(s: u32) -> f64 {
    unsafe { bindings::gsl_sf_bessel_zero_J0(s) }
}

/// These routines compute the scaled irregular modified cylindrical Bessel function
/// of first order $\exp(x) K_1(x)$ for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K1_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K1_scaled).
pub fn bessel_K1_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_K1_scaled(x) }
}

/// These routines compute the regular modified cylindrical Bessel function of
/// zeroth order, $I_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I0).
pub fn bessel_I0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_I0(x) }
}

/// These routines compute the irregular modified cylindrical Bessel function of
/// zeroth order, $K_0(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_K0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_K0).
pub fn bessel_K0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_K0(x) }
}

/// These routines compute the regular modified cylindrical Bessel function of order
/// `n`, $I_n(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_In`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_In).
pub fn bessel_In(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_In(n, x) }
}

/// These routines compute the logarithm of the irregular modified Bessel function
/// of fractional order $\nu$, $\ln(K_\nu(x))$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_lnKnu`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_lnKnu).
pub fn bessel_ln_Knu(nu: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_lnKnu(nu, x) }
}

/// These routines compute the regular modified Bessel function of fractional order
/// $\nu$, $I_\nu(x)$ for $x>0$, $\nu>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Inu`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Inu).
pub fn bessel_Inu(nu: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Inu(nu, x) }
}

/// These routines compute the regular modified cylindrical Bessel function of first
/// order, $I_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I1).
pub fn bessel_I1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_I1(x) }
}

/// These routines compute the irregular modified cylindrical Bessel function of
/// order `n`, $K_n(x)$, for $x > 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Kn`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Kn).
pub fn bessel_Kn(n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Kn(n, x) }
}

/// These routines compute the irregular spherical Bessel function of order `l`,
/// $y_l(x)$, for $l \geq 0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_yl`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_yl).
pub fn bessel_yl(l: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_yl(l, x) }
}

/// These routines compute the irregular cylindrical Bessel function of fractional
/// order $\nu$, $Y_\nu(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_Ynu`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_Ynu).
pub fn bessel_Ynu(nu: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_Ynu(nu, x) }
}

/// These routines compute the regular cylindrical Bessel function of first order,
/// $J_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_J1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_J1).
pub fn bessel_J1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_J1(x) }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of order `l`, $\exp(x) k_l(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_kl_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_kl_scaled).
pub fn bessel_kl_scaled(l: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_kl_scaled(l, x) }
}

/// These routines compute the irregular spherical Bessel function of second order,
/// $y_2(x) = (-3/x^3 + 1/x)\cos(x) - (3/x^2)\sin(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_y2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_y2).
pub fn bessel_y2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_y2(x) }
}

/// These routines compute the regular cylindrical Bessel function of zeroth order,
/// $J_0(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_J0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_J0).
pub fn bessel_J0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_J0(x) }
}

/// These routines compute the scaled regular modified cylindrical Bessel function
/// of first order $\exp(-|x|) I_1(x)$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_I1_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_I1_scaled).
pub fn bessel_I1_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_I1_scaled(x) }
}

/// These routines compute the scaled irregular modified spherical Bessel function
/// of zeroth order, $\exp(x) k_0(x)$, for $x>0$.
/// 
/// Binds the function
/// [`gsl_sf_bessel_k0_scaled`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_bessel_k0_scaled).
pub fn bessel_k0_scaled(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_bessel_k0_scaled(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_bessel_I1_e() {
        check_result!(bessel_I1_e(0.1), 0.05006252604709269211, TEST_TOL0);
        check_result!(bessel_I1_e(2.0), 1.59063685463732906340, TEST_TOL0);
        check_result!(bessel_I1_e(100.0), 1.0683693903381624812e+42, TEST_TOL2);
    }

    #[test]
    fn test_bessel_Knu_scaled_e() {
        check_result!(bessel_Knu_scaled_e(0.0001, 10.0), 0.3916319346235421817, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(1.0, 0.001), 1000.9967345590684524, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(1.0, 1.0), 1.6361534862632582465, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(30.0, 1.0), 1.2792629867539753925e+40, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(30.0, 100.0), 10.673443449954850040, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(10.0, 1.0), 4.912296520990198599e+08, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(10.0, 100.0), 0.20578687173955779807, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(10.0, 1000.0), 0.04165905142800565788, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(10.0, 1.0e+8), 0.00012533147624060789938, TEST_TOL0);
        check_result!(bessel_Knu_scaled_e(10.2, 100.0), 0.20995808355244385075, TEST_TOL0);
    }

    #[test]
    fn test_bessel_I0_scaled_e() {
        check_result!(bessel_I0_scaled_e(0.1), 0.90710092578230109640, TEST_TOL0);
        check_result!(bessel_I0_scaled_e(2.0), 0.30850832255367103953, TEST_TOL0);
        check_result!(bessel_I0_scaled_e(100.0), 0.03994437929909668265, TEST_TOL0);
        check_result!(bessel_I0_scaled_e(65536.0), 0.0015583712551952223537, TEST_TOL0);
    }

    #[test]
    fn test_bessel_K0_e() {
        check_result!(bessel_K0_e(0.1), 2.4270690247020166125, TEST_TOL0);
        check_result!(bessel_K0_e(1.95), 0.1211226255426818887894, TEST_TOL0);
        check_result!(bessel_K0_e(2.0), 0.11389387274953343565, TEST_TOL0);
        check_result!(bessel_K0_e(100.0), 4.656628229175902019e-45, TEST_TOL2);
    }

    #[test]
    fn test_bessel_In_scaled_e() {
        check_result!(bessel_In_scaled_e(-4, 0.1), 2.3575258620054605307e-07, TEST_TOL0);
        check_result!(bessel_In_scaled_e(4, 0.1), 2.3575258620054605307e-07, TEST_TOL0);
        check_result!(bessel_In_scaled_e(5, 2.0), 0.0013297610941881578142, TEST_TOL0);
        check_result!(bessel_In_scaled_e(100, 100.0), 1.7266862628167695785e-22, TEST_TOL0);
    }

    #[test]
    fn test_bessel_zero_J0_e() {
        check_result!(bessel_zero_J0_e(1), 2.404825557695771, TEST_TOL1);
        check_result!(bessel_zero_J0_e(2), 5.520078110286304, TEST_TOL1);
        check_result!(bessel_zero_J0_e(20), 62.048469190227081, TEST_TOL1);
        check_result!(bessel_zero_J0_e(25), 77.756025630388058, TEST_TOL1);
        check_result!(bessel_zero_J0_e(100), 313.37426607752784, TEST_TOL1);
    }

    #[test]
    fn test_bessel_jl_e() {
        check_result!(bessel_jl_e(0, 0.0), 1.0, TEST_TOL0);
        check_result!(bessel_jl_e(1, 10.0), 0.07846694179875154709000, TEST_TOL0);
        check_result!(bessel_jl_e(5, 1.0), 0.00009256115861125816357, TEST_TOL0);
        check_result!(bessel_jl_e(10, 10.0), 0.06460515449256426427, TEST_TOL0);
        check_result!(bessel_jl_e(100, 100.0), 0.010880477011438336539, TEST_TOL1);
        check_result!(bessel_jl_e(2000, 1048576.0), 7.449384239168568534e-07, TEST_SQRT_TOL0);
        check_result!(bessel_jl_e(2, 900.0), -0.0011089115568832940086, TEST_TOL4);
        check_result!(bessel_jl_e(2, 15000.0), -0.00005955592033075750554, TEST_TOL4);
        check_result!(bessel_jl_e(100, 1000.0), -0.00025326311230945818285, TEST_TOL4);
        check_result!(bessel_jl_e(30, 3878.62), -0.00023285567034330878410434732790, TEST_TOL4);
    }

    #[test]
    fn test_bessel_y1_e() {
        check_result!(bessel_y1_e(0.01), -10000.499987500069444, TEST_TOL0);
        check_result!(bessel_y1_e(1.0), -1.3817732906760362241, TEST_TOL0);
        check_result!(bessel_y1_e(10.0), 0.06279282637970150586, TEST_TOL0);
        check_result!(bessel_y1_e(100.0), 0.004977424523868819543, TEST_TOL0);
        check_result!(bessel_y1_e(4294967296.0), 1.0756463271573404688e-10, TEST_SQRT_TOL0);
    }

    #[test]
    fn test_bessel_Kn_scaled_e() {
        check_result!(bessel_Kn_scaled_e(4, 0.1), 530040.2483725626207, TEST_TOL1);
        check_result!(bessel_Kn_scaled_e(5, 2.0), 69.68655087607675118, TEST_TOL0);
        check_result!(bessel_Kn_scaled_e(100, 100.0), 2.0475736731166756813e+19, TEST_TOL1);
    }

    #[test]
    fn test_bessel_Ynu_e() {
        check_result!(bessel_Ynu_e(0.0001, 1.0), 0.08813676933044478439, TEST_TOL2);
        check_result!(bessel_Ynu_e(0.0001, 10.0), 0.05570979797521875261, TEST_TOL2);
        check_result!(bessel_Ynu_e(0.75, 1.0), -0.6218694174429746383, TEST_TOL0);
        check_result!(bessel_Ynu_e(0.75, 10.0), 0.24757063446760384953, TEST_TOL0);
        check_result!(bessel_Ynu_e(1.0, 0.001), -636.6221672311394281, TEST_TOL0);
        check_result!(bessel_Ynu_e(1.0, 1.0), -0.7812128213002887165, TEST_TOL0);
        check_result!(bessel_Ynu_e(30.0, 1.0), -3.0481287832256432162e+39, TEST_TOL0);
        check_result!(bessel_Ynu_e(30.0, 100.0), 0.006138839212010033452, TEST_TOL2);
        check_result!(bessel_Ynu_e(10.0, 1.0), -1.2161801427868918929e+08, TEST_TOL0);
        check_result!(bessel_Ynu_e(10.0, 100.0), 0.05833157423641492875, TEST_TOL2);
        check_result!(bessel_Ynu_e(10.2, 100.0), 0.07169383985546287091, TEST_TOL2);
        check_result!(bessel_Ynu_e(-13.0, 4.3966088395743909188e-1), 5.4675723318993574107e+16, TEST_TOL2);
        check_result!(bessel_Ynu_e(-43.0, 3.2953184511924683369e-1), 2.115977780575035527e+84, TEST_TOL2);
        check_result!(bessel_Ynu_e(-8.0, 3.5081119129046332101e-1), -1.7982193624366780622e+9, TEST_TOL2);
        check_result!(bessel_Ynu_e(-17.0, 1.6717234250796879247e-1), 1.4040223280006382933e+31, TEST_TOL2);
        check_result!(bessel_Ynu_e(-20.0, 1.0085435516296562067e-1), -3.4253870177732383503e+42, TEST_TOL2);
        check_result!(bessel_Ynu_e(-29.0, 1.0423882905853389231e-1), 1.5633159385709367033e+66, TEST_TOL2);
        check_result!(bessel_Ynu_e(-10.0, 2.014635069337132169e-1), -1.0750753223600878559e+15, TEST_TOL2);
        check_result!(bessel_Ynu_e(-23.0, 3.3470953660313309239e-1), 2.5733184597190491073e+38, TEST_TOL2);
        check_result!(bessel_Ynu_e(-13.0, 1.796487688043502395e-2), 6.1526862287828618145e+34, TEST_TOL2);
        check_result!(bessel_Ynu_e(-30.0, 4.3851505455005663023e-1), -1.6652274019860697168e+50, TEST_TOL2);
        check_result!(bessel_Ynu_e(-31.0, 2.9134673992671269075e-1), 7.2783380837435004727e+57, TEST_TOL2);
        check_result!(bessel_Ynu_e(-28.0, 7.6696967407852829575e-1), -1.5748860170624112485e+39, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.0, 1.9829576302527197691e-2), -4.0210481843848071758e+13, TEST_TOL2);
        check_result!(bessel_Ynu_e(-26.0, 4.1513019703157674847e-1), -2.806930683617814941e+42, TEST_TOL2);
        check_result!(bessel_Ynu_e(-19.0, 1.3033500482689031834e-2), 6.9598794107689745713e+56, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0, 7.2050154387915780891e-1), -1.1846384548732517149e+2, TEST_TOL2);
        check_result!(bessel_Ynu_e(-36.0, 9.058436592111083308e-1), -7.994500359474016123e+51, TEST_TOL2);
        check_result!(bessel_Ynu_e(-37.0, 4.4893454860671838425e-2), 1.2042846052782773184e+102, TEST_TOL2);
        check_result!(bessel_Ynu_e(-25.0, 1.4253284590439372148e-1), 9.4085712788480443733e+51, TEST_TOL2);
        check_result!(bessel_Ynu_e(-35.0, 6.8075538970323047806e-1), 2.2691146737122039815e+54, TEST_TOL2);
        check_result!(bessel_Ynu_e(-9.0, 6.9978823056579836732e-2), 1.6330796519846810927e+17, TEST_TOL2);
        check_result!(bessel_Ynu_e(-30.0, 8.3903642499214990225e-1), -5.8816651774415705482e+41, TEST_TOL2);
        check_result!(bessel_Ynu_e(-43.0, 6.543891152683782553e-1), 3.2732133353307485906e+71, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0, 2.557673774862201033e-1), 2.648397359834244179, TEST_TOL2);
        check_result!(bessel_Ynu_e(-21.0, 2.3961944728579204361e-1), 1.7416186098234671613e+37, TEST_TOL2);
        check_result!(bessel_Ynu_e(-13.0, 3.7732868931080794955e-1), 3.9857279826655584502e+17, TEST_TOL2);
        check_result!(bessel_Ynu_e(-33.0, 3.8478064679256785588e-1), 3.5113798569950397588e+58, TEST_TOL2);
        check_result!(bessel_Ynu_e(-50.0, 8.3045728127372512419e-1), -2.3665632218557611716e+81, TEST_TOL2);
        check_result!(bessel_Ynu_e(-33.0, 3.4391840270211572065e-1), 1.4268698281593046704e+60, TEST_TOL2);
        check_result!(bessel_Ynu_e(-23.0, 4.4699489157302132678e-1), 3.3214969951131331346e+35, TEST_TOL2);
        check_result!(bessel_Ynu_e(-23.0, 8.6894302936681315656e-1), 7.6600878858980858273e+28, TEST_TOL2);
        check_result!(bessel_Ynu_e(-24.0, 2.3997647040322801696e-1), -1.0382177146462655416e+44, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0, 9.7688590680055575385e-1), -3.6396008802115138586e+1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-20.0, 8.9090003293395823104e-1), -4.1352523331280200421e+23, TEST_TOL2);
        check_result!(bessel_Ynu_e(-37.0, 9.4057990133775869779e-1), 1.5795116292393394486e+53, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0, 2.6641070579761597867e-1), 2.5520015187867893496, TEST_TOL2);
        check_result!(bessel_Ynu_e(-7.0, 9.8878610880180753494e-1), 3.3070534106551277147e+4, TEST_TOL2);
        check_result!(bessel_Ynu_e(-42.0, 3.1042396387236895292e-1), -1.0201733293438510046e+83, TEST_TOL2);
        check_result!(bessel_Ynu_e(-18.0, 6.8722270094498481586e-1), -2.556940038138305942e+22, TEST_TOL2);
        check_result!(bessel_Ynu_e(-16.0, 1.6113707818439830316e-2), -1.3203947711040991975e+45, TEST_TOL2);
        check_result!(bessel_Ynu_e(-31.0, 3.0720392023079679622e-1), 1.4078366532641984096e+57, TEST_TOL2);
        check_result!(bessel_Ynu_e(-8.0, 3.4957196590626821859e-1), -1.8497964339362348508e+9, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0, 9.9805150837159144851e-1), -3.35276446088454154e+1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-8.0, 8.4538260733781281419e-1), -1.6151126896511724021e+6, TEST_TOL2);
        check_result!(bessel_Ynu_e(-21.0, 7.8935512795174026579e-1), 2.3510763338498465525e+26, TEST_TOL2);
        check_result!(bessel_Ynu_e(-32.0, 3.4859401756229795902e-1), -4.9775956770188671631e+57, TEST_TOL2);
        check_result!(bessel_Ynu_e(-28.0, 9.2530929240293814377e-1), -8.2429457360543127639e+36, TEST_TOL2);
        check_result!(bessel_Ynu_e(-32.0, 5.7276907866586953775e-1), -6.2608710158624288778e+50, TEST_TOL2);
        check_result!(bessel_Ynu_e(-16.0, 8.6364032267280037696e-1), -2.8833961453891338766e+17, TEST_TOL2);
        check_result!(bessel_Ynu_e(-24.0, 8.1065458967451038451e-2), -2.1283114054434655701e+55, TEST_TOL2);
        check_result!(bessel_Ynu_e(-17.0, 6.946415511078842723e-1), 4.3084587997971578431e+20, TEST_TOL2);
        check_result!(bessel_Ynu_e(-14.0, 2.7619565270505476723e-1), -2.1634745597658813187e+21, TEST_TOL2);
        check_result!(bessel_Ynu_e(-7.0, 8.579791601466317093e-2), 8.5741016888820088808e+11, TEST_TOL2);
        check_result!(bessel_Ynu_e(-50.0, 7.4681490716561041968e-1), -4.7757514561562387813e+83, TEST_TOL2);
        check_result!(bessel_Ynu_e(-30.0, 9.6631424224612452556e-1), -8.5159643935229761718e+39, TEST_TOL2);
        check_result!(bessel_Ynu_e(-14.0, 2.6745819874317492788e-1), -3.3928529652241947717e+21, TEST_TOL2);
        check_result!(bessel_Ynu_e(-22.0, 3.6947309321414497419e-1), -2.2270901208692386374e+35, TEST_TOL2);
        check_result!(bessel_Ynu_e(-49.0, 2.3389602803779083655e-2), 1.8412401963717225375e+155, TEST_TOL2);
        check_result!(bessel_Ynu_e(-48.0, 7.7482504299905127354e-1), -4.8369236019321535917e+78, TEST_TOL2);
        check_result!(bessel_Ynu_e(-30.0, 6.5337950709230049969e-1), -1.0639517934361092802e+45, TEST_TOL2);
        check_result!(bessel_Ynu_e(-29.0, 5.4117488569210959438e-1), 2.8262145810872807871e+45, TEST_TOL2);
        check_result!(bessel_Ynu_e(-50.0, 9.4038774092645791075e-1), -4.7322361571978865664e+78, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0, 5.627399919447310892e-1), 1.3310580822914015523, TEST_TOL2);
        check_result!(bessel_Ynu_e(-35.0, 7.9925999507829895011e-2), 8.2224704007424584626e+86, TEST_TOL2);
        check_result!(bessel_Ynu_e(-10.0, 2.8875439067692705135e-3), -2.9351293887447581255e+33, TEST_TOL2);
        check_result!(bessel_Ynu_e(-31.0, 2.7645446745852278572e-1), 3.7015590625912763344e+58, TEST_TOL2);
        check_result!(bessel_Ynu_e(-12.0, 8.5374968606378578252e-1), -3.528575811542778544e+11, TEST_TOL2);
        check_result!(bessel_Ynu_e(-11.0, 2.2997159465452075155e-1), 2.4894373648370412225e+16, TEST_TOL2);
        check_result!(bessel_Ynu_e(-47.0, 5.8616043503503357359e-1), 1.9763554381308657733e+82, TEST_TOL2);
        check_result!(bessel_Ynu_e(-24.0, 8.4884849730214263521e-1), -7.1046392480541118407e+30, TEST_TOL2);
        check_result!(bessel_Ynu_e(-24.0, 5.3943176031162412158e-1), -3.7580440656007599181e+35, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.0, 9.3387179309343112648e-1), 7.0202042170209205589, TEST_TOL2);
        check_result!(bessel_Ynu_e(-20.0, 5.5061917642540109891e-1), -6.2122754555889405012e+27, TEST_TOL2);
        check_result!(bessel_Ynu_e(-26.0, 2.9315675715515822781e-1), -2.3771210812024680281e+46, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.0, 8.9079715253593128884e-1), -5.091626137833991034e+3, TEST_TOL2);
        check_result!(bessel_Ynu_e(-30.0, 9.1220250697048036241e-2), -4.7597279133940392868e+70, TEST_TOL2);
        check_result!(bessel_Ynu_e(-20.0, 6.2008856084878849548e-3), -5.7473876046110789338e+66, TEST_TOL2);
        check_result!(bessel_Ynu_e(-23.0, 7.1620177578706903472e-1), 6.5166498927440582853e+30, TEST_TOL2);
        check_result!(bessel_Ynu_e(-29.0, 5.1124413385957329246e-1), 1.4711351369242792138e+46, TEST_TOL2);
        check_result!(bessel_Ynu_e(-23.0, 6.7405321505832900803e-1), 2.627743272609579819e+31, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.7408502287167772557e+1, 4.3891036254061936271e-2), 4.469714143570654497e+41, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0369289750688261206e+1, 8.617077861907621132e-1), -5.8595434076664310948e+60, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.6398263821779503158, 5.7108775125890870648e-1), -5.2034221685931678753e+1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.0743837616631487936e+1, 2.2372123946196517647e-1), 1.345588718040376054e+37, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.7297952956642177128e+1, 2.8440280848533973972e-1), 4.1115735370678699359e+27, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.3507072230824139103e+1, 9.7008738913549467403e-1), -2.202372544321863968e+11, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.4663106025697138284e+1, 9.6739483983540323655e-1), 1.4235309894056948387e+67, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.7450767905208691173e+1, 5.2288309478685515663e-3), 5.3855142705411361919e+179, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.8736914274754280581e+1, 4.4318942692773250336e-1), 1.1773204343097478161e+27, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.4560791710276042462e+1, 5.6595841783863809163e-1), 3.3572924118396673305e+55, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.8723165418996042381e+1, 2.4003201116391659975e-1), 6.9471556080056038434e+54, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.1780816480084454714e+1, 3.5000033756039229564e-1), -1.0833823624917670056e+57, TEST_TOL2);
        check_result!(bessel_Ynu_e(-8.3572537806135923827, 8.994859317735841446e-1), -1.1774337302489088463e+6, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.9179499652113791027e+1, 7.3466044408596358731e-1), 1.6517722024836217581e+82, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0204535104461498585e+1, 1.3316368076269799096e-1), -2.2341067840937606376e+93, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.9597169419874779916e+1, 8.3895848736941180651e-1), -6.2662143656762004053e+79, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.2228113163441444643e+1, 7.2360070820350912315e-1), -4.2511898289085272856e+12, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.3252347726870680973e+1, 4.7042383176138740544e-2), 2.2194603264543665848e+121, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.8363275112470146292e+1, 3.5406545166014987925e-1), -3.248353813678515594e+95, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.4031313414732363799e+1, 1.7077658851102128503e-1), -4.3273454620971397041e+47, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.4994220322161396103e+1, 7.6004498361697560048e-2), 4.5976914154279708742e+87, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.5842451866339926986e+1, 7.1655842470093644641e-1), -2.7708334780004236249e+18, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.3303969013414183336e+1, 3.4261981308825774017e-1), 1.2252074327758681488e+84, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.9620214546900609668e+1, 8.9559048893071335969e-2), -2.5960278365111086141e+69, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.9267775155252921091e+1, 4.9173394917277714574e-1), 2.4927765540337657002e+68, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.085805422677201981e+1, 7.1844728658692273953e-2), 3.6253005874483288876e+21, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.3205222720657600449e+1, 2.0938011160695718605e-1), 1.7097593469426311991e+93, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.1761832688338363731e-1, 4.0692821730479031008e-1), -3.9929527887838440231e-1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.4176152886105712673e+1, 7.3083748089885469319e-1), -3.8375532995220739134e+51, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.4363999308889822481e+1, 2.2964635433363436307e-1), -6.7651597483976020212e+22, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.5945782535187393546e+1, 5.5004988492253222851e-1), -3.0929200048995927397e+20, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.6539439893219625339e+1, 5.4022213494661757887e-1), 4.3099923396955639095e+39, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.9104819423206076833e+1, 8.7581079115207012305e-1), 2.4523357879118971691e+58, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.1344422530419629852e+1, 1.8412292138063948156e-1), 6.9306682864864401354e+17, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.2479004807998664153e+1, 7.628052499161305046e-1), -2.5492681017463054894e+66, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.7234292208462683278e+1, 5.6929354834881763134e-1), 1.4969074140226347429e+41, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.9852923491811760298e+1, 2.1764339448558498796e-2), -6.237969203121440244e+88, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.9470737712526065987e+1, 9.1397839227827761003e-1), 4.5960647034986138864e+38, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.4879459756439547254e+1, 8.7694253508024822462e-1), 1.0188843810269963844e+32, TEST_TOL2);
        check_result!(bessel_Ynu_e(-8.6562240771489400173e-1, 6.4882619355798588927e-1), 1.1287689391130420057, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.4096305256724256786e+1, 1.1837901256079790395e-1), -4.9304578916106509819e+26, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.3739240782592000796e+1, 1.1830951764837136725e-1), -5.1263258788046229045e+25, TEST_TOL2);
        check_result!(bessel_Ynu_e(-7.7683272384817803185e-1, 2.0897180603218726575e-1), 1.881590189544752977, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.9007825894244022613e+1, 8.0804305816981973945e-1), 7.9534669124448258162e+79, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.224872379992519031, 9.4716012050013901355e-1), 5.2709059549031973037e-1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-5.0807600398060325807e+1, 6.1902119795226148946e-1), 2.8318147090250448397e+89, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.2130610423140110162e+1, 7.2184881647444607846e-1), -7.0593986791573883029e+67, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.1186300874143057513e+1, 1.3944550368322800884e-1), 1.8718282789345414439e+95, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.1777582295815773824e+1, 7.6178874271077561762e-1), -8.5399447567722731535e+27, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.2185912810368133222e+1, 3.959510541558671016e-1), -1.8100903248905368685e+56, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.2976006083946226988e+1, 4.5739390828369379657e-1), 1.4034454523109119588e+78, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.0412232215064606945e+1, 6.7510758855896918145e-1), -6.8761190595978173779e+44, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.8388188389747281955e+1, 8.9233979909724013718e-1), -3.3498674090203423756e+20, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.4298840569257253701e+1, 6.8042862360863559591e-1), -3.0013837000776612869e+33, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.8834735272673504063e+1, 4.2324469410479691518e-1), 1.2310766881467564871e+70, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.6091840934431606344e+1, 1.7508918790902548661e-1), -2.946550933505149671e+104, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.9754264394942756728, 1.558717798933954111e-2), -4.5906289772187980725e+3, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.7168594342366560374e+1, 6.976373865080374073e-1), 1.8851114888349680045e+58, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.5379944292029245754e+1, 5.3150471205968938472e-2), 4.1473845726043547937e+125, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.153181439556533474e+1, 1.8204366094125528818e-1), -4.1102104914739582156e+17, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.9939680334734766385e+1, 8.132277926604580844e-1), -4.9623796120191838477e+81, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.5986707652967270442e+1, 5.5665988728905782182e-1), -3.058579118813851873e+59, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.7046620273111013832, 1.059530133767196237e-1), 2.8547617693483259231e+10, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.154578607200052944e+1, 4.5282605452993561522e-1), -4.6204765000245865931e+31, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.3955292370868357326e+1, 5.3774303067450442349e-1), -3.285152670384139907e+35, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.2209824848466094111e+1, 5.9504757613868441576e-1), -3.8856592501211432238e+50, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0825185544715207495e+1, 7.5398610386439322567e-1), 2.3035172984979776888e+64, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.1910588368134692781e+1, 5.1283495159288225414e-1), -1.3438429765742294157e+52, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.7113819006700078322, 4.0393307486388760775e-1), 3.8345193040174093796e+6, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.8693610985720627242e+1, 3.4777341593350738095e-1), 1.2557150749869300709e+50, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.3640245743314574591e+1, 9.6025287507519664839e-1), -3.8943763839931542689e+28, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.8083409800395899615e+1, 1.4391402197265332926e-1), -9.9126133627220300984e+113, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.8105478052324389543e+1, 4.891514010318381566e-1), -1.2271238830087142465e+66, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.5311996069559305006e+1, 9.5766172213158543809e-1), 3.7577671642248517421e+31, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.0646798215967974621e+1, 9.3310976056897272996e-1), 1.5913395046591416208e+41, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.9802221662384415773e+1, 8.0558698100484531078e-1), -1.3410097300803423969e+61, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.6307175589532141033e+1, 8.5581965580057553716e-1), -8.2913043528862589862e+72, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0008133602203088381e+1, 8.8260963451522902618e-1), -4.3184875964228242059e+8, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.2307910802466458785e+1, 6.782808128445430142e-1), -1.4119756731193528077e+69, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0130438814666196845e+1, 2.1277791687648788154e-1), -1.02690311217639001e+15, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.9533803019889111726e+1, 3.9334897500975715385e-1), -6.4704382293897894852e+28, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.3895956090712434265e+1, 8.0661260505451797979e-1), -4.3815146159956971318e+14, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.4635165127833994625e+1, 3.7945739040607276532e-1), 1.5929485514781295968e+20, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.8560967136543055351e+1, 7.9600892022399284277e-1), 1.1465774145795382248e+39, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.2108119261932190115, 8.7806601388332832487e-1), -7.4701969028629161594e+3, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.9950977953230716109, 5.8978625225697869088e-1), 1.1808677382965817331e+6, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.2481246429363894157e+1, 7.2704916561624886813e-1), -7.5667536238826222942e+11, TEST_TOL2);
        check_result!(bessel_Ynu_e(-5.020982518214251176e+1, 7.5109544046044664159e-1), -7.9040859853121739692e+83, TEST_TOL2);
        check_result!(bessel_Ynu_e(-8.9473034048334154025, 6.4213370686611774774e-2), 2.6029884691597775763e+17, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.834075672586642874e+1, 6.4258947909888861139e-1), -5.8405580195454874797e+61, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.4411381704857813062e+1, 1.2743783697917866926e-1), -3.1773798568513309515e+105, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.1687391291035845538, 7.8786722297686215495e-1), -1.4219092995559599082e+4, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0704293950958482287e+1, 3.8531626603275482749e-1), 1.5743246247238302576e+13, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0571445654620545043e+1, 3.4876986634505808892e-1), 7.0385234557024272162e+76, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.3563863522003561189, 9.7999452783586835833e-1), -3.1893996030425450694e+1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.8492269674910402783e+1, 1.7152753186318347442e-1), -6.0249928142043118768e+32, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.5363228484567149913e+1, 3.0542644930515794217e-1), 1.3211507552477009856e+44, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.3771144161000489935e+1, 7.3346736008344405886e-1), -8.3124988094143057824e+14, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.3761246233591030521e+1, 5.4968519162734822572e-1), -6.0578774841300955332e+34, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.2634961325802757898e+1, 6.6471162507224558099e-1), 3.2082973398317013106e+30, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.6563404270777516422e+1, 7.5273096421024420905e-1), 3.7592589106971418602e+75, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0848764148512049678e+1, 9.2286052929496267966e-1), 3.2439553455961291223e+9, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.5455854142576014389e+1, 7.8971992208889605774e-1), 1.3378630820768277448e+52, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.2013968530863113871e+1, 1.9744029176545173315e-1), -1.584787479000820067e+19, TEST_TOL2);
        check_result!(bessel_Ynu_e(-5.6750075448691398076, 8.0444727938729491316e-1), -2.10533382223663811e+3, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.8429830608986305295e+1, 8.2140563165026789468e-1), -1.1405415108194473062e+21, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.346426407446347328e+1, 8.5111501099434108204e-1), 8.7606083372225198285e+28, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.9593848458448579212e+1, 1.2641237816367966816e-1), -6.4135079973931874203e+64, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.7086231381348987822e+1, 3.9893860083544831831e-1), 7.515835122388342095e+24, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.8484417257406457732, 6.0748406712157845873e-1), -2.7380610022426281822, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.1822890333552359573e+1, 5.0020129878524173631e-1), -1.7231227888225767619e+52, TEST_TOL2);
        check_result!(bessel_Ynu_e(-6.2862753931112845805, 7.9353453932163968388e-1), -1.342401423092805412e+4, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.5508352825172743548, 7.3200379508076835206e-1), -4.0497851981930665643e-1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-7.6325669923999446014, 6.3959118408264715884e-2), -8.0591625416199259092e+13, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.7217355888852626548e+1, 8.2439297912634420605e-1), 4.2640264465349061973e+55, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.9216632463077931365e+1, 2.1243165065073566651e-2), 7.2753341407348563956e+121, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.0310494244455808579e+1, 5.6409661087945220564e-1), -8.0236717760932647636e+27, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.2561355227860967802e+1, 6.1429304168687688441e-1), 1.095939957156354534e+71, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.348280958534689123e+1, 1.3878791555680588964e-1), 1.4127166630922246712e+47, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.266908051943509346, 9.1043177038683039605e-1), 7.9342040586370876107, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.6453677594081999426e+1, 7.3766130024218803361e-1), -2.8496103761067379068e+18, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.0292370302033138277e+1, 2.7545919896373212095e-1), -9.9081192322574702806e+13, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.788826873303230097e+1, 2.2855276522975392381e-1), -4.2068889321668479948e+53, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.1401875376696767087e+1, 6.6522142928175599046e-2), 5.4702914219176701268e+108, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.9306765177660380865e+1, 2.0291343278160599375e-1), 4.3752403877947450222e+34, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.9778989699515361126e+1, 5.0069816606055756877e-2), -5.0144871285761887467e+77, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.3484350943085112332e+1, 2.5580340114657331489e-2), 8.660979419407748338e+32, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.8885950309653823971e+1, 2.705230773944354062e-1), 7.0729128795400209404e+102, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.6140505254948262834e+1, 5.635769797194365065e-1), -1.691181704671511378e+39, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.36353791993838395e+1, 7.8601342847547894355e-1), -4.184570564713370597e+30, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.8043892058273381078e+1, 9.6661631483018731188e-1), -5.2799069238255343804e+54, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.0627792568626186656e+1, 2.7801894968851888201e-1), 4.6951098613000862734e+34, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.7821819326414013127e+1, 7.0072053744106375522e-1), -3.2969189837617817178e+59, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.1895350862490616295e+1, 6.3819566879323744549e-1), -7.4877627933750167273e+12, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.1564822793166119652e+1, 8.7089052441788146841e-1), -4.362095115786654673e+62, TEST_TOL2);
        check_result!(bessel_Ynu_e(-5.6988133455245986782, 1.0513161628614675752e-1), -2.6315584241050780179e+8, TEST_TOL2);
        check_result!(bessel_Ynu_e(-5.04236829179067909, 2.1606414592833118122e-2), 6.6460848251254935943e+10, TEST_TOL2);
        check_result!(bessel_Ynu_e(-8.5815569416794001464e-1, 3.2800854133020344342e-1), 1.7278337309129555492, TEST_TOL2);
        check_result!(bessel_Ynu_e(-7.4540158508402344121, 3.3591301584669540366e-1), 4.6958147873473343296e+7, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.1342269410012557907e+1, 7.2248859912133702972e-1), 8.4210802265866850829e+65, TEST_TOL2);
        check_result!(bessel_Ynu_e(-4.0890888214378787004e+1, 4.6015468690527406659e-2), 1.5760625185685568109e+114, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.4949019201278440062e+1, 5.5379096162817544569e-1), 2.4017690399425079403e+57, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.5581474535040863099, 8.6545078146345742122e-1), -4.2518863856194071801e-1, TEST_TOL2);
        check_result!(bessel_Ynu_e(-1.6450819423469368818e+1, 4.0968007844676920681e-1), -4.7316334255824816328e+22, TEST_TOL2);
        check_result!(bessel_Ynu_e(-2.4534051152698170766e+1, 9.2462004307970597256e-1), 8.0009048223739294629e+29, TEST_TOL2);
        check_result!(bessel_Ynu_e(-9.5846016059057764355, 1.4980322293854772757e-1), -7.3726783694247075753e+14, TEST_TOL2);
        check_result!(bessel_Ynu_e(-3.1840530425923939329e+1, 9.3847997261021697482e-1), -3.8745468671462878671e+43, TEST_TOL2);
    }

    #[test]
    fn test_bessel_K1_scaled_e() {
        check_result!(bessel_K1_scaled_e(0.1), 10.890182683049696574, TEST_TOL0);
        check_result!(bessel_K1_scaled_e(1.95), 1.050086915104152747182, TEST_TOL0);
        check_result!(bessel_K1_scaled_e(2.0), 1.0334768470686885732, TEST_TOL0);
        check_result!(bessel_K1_scaled_e(6.0), 0.5421759102771335382849, TEST_TOL0);
        check_result!(bessel_K1_scaled_e(100.0), 0.1257999504795785293, TEST_TOL0);
    }

    #[test]
    fn test_bessel_zero_Jnu_e() {
        check_result!(bessel_zero_Jnu_e(0.0, 1), 2.404825557695771, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(0.0, 2), 5.520078110286304, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(0.0, 20), 62.048469190227081, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(0.0, 25), 77.756025630388058, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(0.0, 100), 313.37426607752784, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 1), 4.4934094579090641, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 1), 8.7714838159599540, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 2), 7.7252518369377072, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 2), 12.338604197466944, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 3), 10.904121659428900, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 3), 15.700174079711671, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 4), 14.066193912831473, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 4), 18.980133875179921, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 5), 17.220755271930768, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 5), 22.217799896561268, TEST_SQRT_TOL0);
        check_result!(bessel_zero_Jnu_e(8.0, 5), 26.266814641176644, TEST_SQRT_TOL0);
        check_result!(bessel_zero_Jnu_e(20.0, 5), 41.413065513892636, TEST_SQRT_TOL0);
        check_result!(bessel_zero_Jnu_e(1.5, 6), 20.371302959287563, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 6), 25.430341154222704, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(8.0, 6), 29.545659670998550, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 7), 23.519452498689007, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 7), 28.626618307291138, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(8.0, 7), 32.795800037341462, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 8), 26.666054258812674, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 8), 31.811716724047763, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(10.0, 8), 38.761807017881651, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 9), 29.811598790892959, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 9), 34.988781294559295, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(10.0, 9), 42.004190236671805, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(1.5, 10), 32.956389039822477, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 10), 38.159868561967132, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 10), 52.017241278881633, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 11), 41.326383254047406, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 11), 55.289204146560061, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 12), 44.4893191232197314, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 12), 58.5458289043850856, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 13), 47.6493998066970948, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 13), 61.7897598959450550, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 14), 50.8071652030063595, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 14), 65.0230502510422545, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 15), 53.9630265583781707, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 15), 68.2473219964207837, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 16), 57.1173027815042647, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 16), 71.4638758850226630, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 17), 60.2702450729428077, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 17), 74.6737687121404241, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 18), 63.4220540458757799, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 18), 77.8778689734863729, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 19), 66.5728918871182703, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 19), 81.0768977206328326, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(5.0, 20), 69.722891161716742, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(15.0, 20), 84.271459069716442, TEST_TOL1);
        check_result!(bessel_zero_Jnu_e(23.0, 11), 65.843393469524653, TEST_TOL6);
        check_result!(bessel_zero_Jnu_e(30.0, 11), 74.797306585175426, TEST_TOL6);
        check_result!(bessel_zero_Jnu_e(32.0, 15), 90.913637691861741, TEST_TOL6);
        check_result!(bessel_zero_Jnu_e(50.0, 15), 113.69747988073942, TEST_TOL6);
        check_result!(bessel_zero_Jnu_e(5.0, 22), 76.020793430591605, TEST_TOL2);
        check_result!(bessel_zero_Jnu_e(10.0, 22), 83.439189796105756, TEST_TOL3);
        check_result!(bessel_zero_Jnu_e(12.0, 22), 86.345496520534055, TEST_TOL6);
        check_result!(bessel_zero_Jnu_e(100.0, 22), 199.82150220122519, TEST_TOL4);
        check_result!(bessel_zero_Jnu_e(500.0, 22), 649.34132440891735, TEST_TOL2);
    }

    #[test]
    fn test_bessel_K0_scaled_e() {
        check_result!(bessel_K0_scaled_e(0.1), 2.6823261022628943831, TEST_TOL0);
        check_result!(bessel_K0_scaled_e(1.95), 0.8513330938802157074894, TEST_TOL0);
        check_result!(bessel_K0_scaled_e(2.0), 0.8415682150707714179, TEST_TOL0);
        check_result!(bessel_K0_scaled_e(6.0), 0.50186313086214003217346, TEST_TOL0);
        check_result!(bessel_K0_scaled_e(100.0), 0.1251756216591265789, TEST_TOL0);
    }

    #[test]
    fn test_bessel_zero_J1_e() {
        check_result!(bessel_zero_J1_e(1), 3.831705970207512, TEST_TOL2);
        check_result!(bessel_zero_J1_e(2), 7.015586669815619, TEST_TOL2);
        check_result!(bessel_zero_J1_e(20), 63.61135669848124, TEST_TOL2);
        check_result!(bessel_zero_J1_e(25), 79.32048717547630, TEST_TOL2);
        check_result!(bessel_zero_J1_e(100), 314.9434728377672, TEST_TOL2);
    }

    #[test]
    fn test_bessel_Inu_scaled_e() {
        check_result!(bessel_Inu_scaled_e(0.0001, 10.0), 0.12783333709581669672, TEST_TOL0);
        check_result!(bessel_Inu_scaled_e(1.0, 0.001), 0.0004995003123542213370, TEST_TOL0);
        check_result!(bessel_Inu_scaled_e(1.0, 1.0), 0.20791041534970844887, TEST_TOL0);
        check_result!(bessel_Inu_scaled_e(30.0, 1.0), 1.3021094983785914437e-42, TEST_TOL0);
        check_result!(bessel_Inu_scaled_e(30.0, 100.0), 0.0004486987756920986146, TEST_TOL3);
        check_result!(bessel_Inu_scaled_e(10.0, 1.0), 1.0127529864692066036e-10, TEST_TOL0);
        check_result!(bessel_Inu_scaled_e(10.0, 100.0), 0.024176682718258828365, TEST_TOL3);
        check_result!(bessel_Inu_scaled_e(10.2, 100.0), 0.023691628843913810043, TEST_TOL3);
    }

    #[test]
    fn test_bessel_i2_scaled_e() {
        check_result!(bessel_i2_scaled_e(0.0), 0.0, TEST_TOL0);
        check_result!(bessel_i2_scaled_e(0.1), 0.0006036559400239012567, TEST_TOL0);
        check_result!(bessel_i2_scaled_e(2.0), 0.0476185434029034785100, TEST_TOL0);
        check_result!(bessel_i2_scaled_e(100.0), 0.0048515000000000000000, TEST_TOL0);
    }

    #[test]
    fn test_bessel_I0_e() {
        check_result!(bessel_I0_e(0.1), 1.0025015629340956014, TEST_TOL0);
        check_result!(bessel_I0_e(2.0), 2.2795853023360672674, TEST_TOL0);
        check_result!(bessel_I0_e(100.0), 1.0737517071310738235e+42, TEST_TOL2);
    }

    #[test]
    fn test_bessel_Y0_e() {
        check_result!(bessel_Y0_e(0.1), -1.5342386513503668441, TEST_TOL0);
        check_result!(bessel_Y0_e(2.0), 0.5103756726497451196, TEST_TOL0);
        check_result!(bessel_Y0_e(256.0), -0.03381290171792454909, TEST_TOL0);
        check_result!(bessel_Y0_e(4294967296.0), 3.657903190017678681e-06, TEST_SQRT_TOL0);
    }

    #[test]
    fn test_bessel_yl_e() {
        check_result!(bessel_yl_e(0, 0.01), -99.995000041666528, TEST_TOL0);
        check_result!(bessel_yl_e(0, 1.0), -0.54030230586813972, TEST_TOL0);
        check_result!(bessel_yl_e(1, 10.0), 0.062792826379701506, TEST_TOL0);
        check_result!(bessel_yl_e(5, 1.0), -999.44034339223641, TEST_TOL0);
        check_result!(bessel_yl_e(10, 0.01), -6.5473079797378378e+30, TEST_TOL0);
        check_result!(bessel_yl_e(10, 10.0), -0.172453672088057849, TEST_TOL0);
        check_result!(bessel_yl_e(100, 1.0), -6.6830794632586775e+186, TEST_TOL1);
        check_result!(bessel_yl_e(100, 100.0), -0.0229838504915622811, TEST_TOL1);
        check_result!(bessel_yl_e(2000, 1048576.0), 5.9545201447146155e-07, TEST_SQRT_TOL0);
    }

    #[test]
    fn test_bessel_Yn_e() {
        check_result!(bessel_Yn_e(4, 0.1), -305832.29793353160319, TEST_TOL1);
        check_result!(bessel_Yn_e(5, 2.0), -9.935989128481974981, TEST_TOL0);
        check_result!(bessel_Yn_e(100, 100.0), -0.16692141141757650654, TEST_TOL0);
        check_result!(bessel_Yn_e(100, 4294967296.0), 3.657889671577715808e-06, TEST_SQRT_TOL0);
        check_result!(bessel_Yn_e(1000, 4294967296.0), 3.656551321485397501e-06, 2.0e-05);
        check_result!(bessel_Yn_e(2, 15000.0), -0.006185217273358617849, TEST_TOL4);
    }

    #[test]
    fn test_bessel_J0_e() {
        check_result!(bessel_J0_e(0.1), 0.99750156206604003230, TEST_TOL0);
        check_result!(bessel_J0_e(2.0), 0.22389077914123566805, TEST_TOL0);
        check_result!(bessel_J0_e(100.0), 0.019985850304223122424, TEST_TOL0);
        check_result!(bessel_J0_e(1.0e+10), 2.1755917502468917269e-06, TEST_SQRT_TOL0);
    }

    #[test]
    fn test_bessel_Jnu_e() {
        check_result!(bessel_Jnu_e(0.0001, 1.0), 0.7652115411876708497, TEST_TOL2);
        check_result!(bessel_Jnu_e(0.0001, 10.0), -0.2459270166445205, TEST_TOL2);
        check_result!(bessel_Jnu_e(0.0009765625, 10.0), -0.2458500798634692, TEST_TOL2);
        check_result!(bessel_Jnu_e(0.75, 1.0), 0.5586524932048917478, TEST_TOL2);
        check_result!(bessel_Jnu_e(0.75, 10.0), -0.04968928974751508135, TEST_TOL2);
        check_result!(bessel_Jnu_e(1.0, 0.001), 0.0004999999375000026, TEST_TOL0);
        check_result!(bessel_Jnu_e(1.0, 1.0), 0.4400505857449335160, TEST_TOL0);
        check_result!(bessel_Jnu_e(1.75, 1.0), 0.168593922545763170103, TEST_TOL1);
        check_result!(bessel_Jnu_e(30.0, 1.0), 3.482869794251482902e-42, TEST_TOL0);
        check_result!(bessel_Jnu_e(30.0, 100.0), 0.08146012958117222297, TEST_TOL1);
        check_result!(bessel_Jnu_e(10.0, 1.0), 2.6306151236874532070e-10, TEST_TOL0);
        check_result!(bessel_Jnu_e(10.0, 100.0), -0.05473217693547201474, TEST_TOL2);
        check_result!(bessel_Jnu_e(10.2, 100.0), -0.03548919161046526864, TEST_TOL2);
        check_result!(bessel_Jnu_e(2.0, 900.0), -0.019974345269680646400, TEST_TOL4);
        check_result!(bessel_Jnu_e(2.0, 15000.0), -0.0020455820181216382666, TEST_TOL4);
        check_result!(bessel_Jnu_e(-13.0, 4.3966088395743909188e-1), -4.4808688873945295916e-19, TEST_TOL2);
        check_result!(bessel_Jnu_e(-43.0, 3.2953184511924683369e-1), -3.4985116870357066158e-87, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.0, 3.5081119129046332101e-1), 2.2148387185334257376e-11, TEST_TOL2);
        check_result!(bessel_Jnu_e(-17.0, 1.6717234250796879247e-1), -1.3336696368048418208e-33, TEST_TOL2);
        check_result!(bessel_Jnu_e(-20.0, 1.0085435516296562067e-1), 4.6463938513369299663e-45, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 1.0423882905853389231e-1), -7.0211488877617616588e-69, TEST_TOL2);
        check_result!(bessel_Jnu_e(-10.0, 2.014635069337132169e-1), 2.9614218635575180136e-17, TEST_TOL2);
        check_result!(bessel_Jnu_e(-23.0, 3.3470953660313309239e-1), -5.3786691977970313226e-41, TEST_TOL2);
        check_result!(bessel_Jnu_e(-13.0, 1.796487688043502395e-2), -3.9796275104402232636e-37, TEST_TOL2);
        check_result!(bessel_Jnu_e(-30.0, 4.3851505455005663023e-1), 6.3723816221242007043e-53, TEST_TOL2);
        check_result!(bessel_Jnu_e(-31.0, 2.9134673992671269075e-1), -1.4108323502121482121e-60, TEST_TOL2);
        check_result!(bessel_Jnu_e(-28.0, 7.6696967407852829575e-1), 7.2211466723046636958e-42, TEST_TOL2);
        check_result!(bessel_Jnu_e(-6.0, 1.9829576302527197691e-2), 1.3193561299244410609e-15, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 4.1513019703157674847e-1), 4.362149422492811209e-45, TEST_TOL2);
        check_result!(bessel_Jnu_e(-19.0, 1.3033500482689031834e-2), -2.4071043287214877014e-59, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0, 7.2050154387915780891e-1), 6.8377203227324865568e-4, TEST_TOL2);
        check_result!(bessel_Jnu_e(-36.0, 9.058436592111083308e-1), 1.1063535538518134862e-54, TEST_TOL2);
        check_result!(bessel_Jnu_e(-37.0, 4.4893454860671838425e-2), -7.1436404620419702996e-105, TEST_TOL2);
        check_result!(bessel_Jnu_e(-25.0, 1.4253284590439372148e-1), -1.3532982149810467304e-54, TEST_TOL2);
        check_result!(bessel_Jnu_e(-35.0, 6.8075538970323047806e-1), -4.0087398199591044218e-57, TEST_TOL2);
        check_result!(bessel_Jnu_e(-9.0, 6.9978823056579836732e-2), -2.1657760307485265867e-19, TEST_TOL2);
        check_result!(bessel_Jnu_e(-30.0, 8.3903642499214990225e-1), 1.8046736761082165751e-44, TEST_TOL2);
        check_result!(bessel_Jnu_e(-43.0, 6.543891152683782553e-1), -2.2618181452671187564e-74, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.0, 2.557673774862201033e-1), -1.2684081505646766845e-1, TEST_TOL2);
        check_result!(bessel_Jnu_e(-21.0, 2.3961944728579204361e-1), -8.7037441094405713341e-40, TEST_TOL2);
        check_result!(bessel_Jnu_e(-13.0, 3.7732868931080794955e-1), -6.1458679923678486123e-20, TEST_TOL2);
        check_result!(bessel_Jnu_e(-33.0, 3.8478064679256785588e-1), -2.7471851206170506902e-61, TEST_TOL2);
        check_result!(bessel_Jnu_e(-50.0, 8.3045728127372512419e-1), 2.6904315506244885678e-84, TEST_TOL2);
        check_result!(bessel_Jnu_e(-33.0, 3.4391840270211572065e-1), -6.7604474418522862789e-63, TEST_TOL2);
        check_result!(bessel_Jnu_e(-23.0, 4.4699489157302132678e-1), -4.1674520864249550489e-38, TEST_TOL2);
        check_result!(bessel_Jnu_e(-23.0, 8.6894302936681315656e-1), -1.8080036072447165775e-31, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 2.3997647040322801696e-1), 1.2775332144705955465e-46, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0, 9.7688590680055575385e-1), 2.260676935437026869e-3, TEST_TOL2);
        check_result!(bessel_Jnu_e(-20.0, 8.9090003293395823104e-1), 3.8525697232471553813e-26, TEST_TOL2);
        check_result!(bessel_Jnu_e(-37.0, 9.4057990133775869779e-1), -5.4483632006108552908e-56, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.0, 2.6641070579761597867e-1), -1.3202706692617745567e-1, TEST_TOL2);
        check_result!(bessel_Jnu_e(-7.0, 9.8878610880180753494e-1), -1.3892626129288932172e-6, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 3.1042396387236895292e-1), 7.4291433893935351027e-86, TEST_TOL2);
        check_result!(bessel_Jnu_e(-18.0, 6.8722270094498481586e-1), 6.9210950943508574463e-25, TEST_TOL2);
        check_result!(bessel_Jnu_e(-16.0, 1.6113707818439830316e-2), 1.5066992425677873707e-47, TEST_TOL2);
        check_result!(bessel_Jnu_e(-31.0, 3.0720392023079679622e-1), -7.2938615192106070364e-60, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.0, 3.4957196590626821859e-1), 2.153068469114375124e-11, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0, 9.9805150837159144851e-1), 2.4578749047519916152e-3, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.0, 8.4538260733781281419e-1), 2.4776262290872395403e-8, TEST_TOL2);
        check_result!(bessel_Jnu_e(-21.0, 7.8935512795174026579e-1), -6.4516652247824702208e-29, TEST_TOL2);
        check_result!(bessel_Jnu_e(-32.0, 3.4859401756229795902e-1), 1.9985100102959434086e-60, TEST_TOL2);
        check_result!(bessel_Jnu_e(-28.0, 9.2530929240293814377e-1), 1.3798987861611642315e-39, TEST_TOL2);
        check_result!(bessel_Jnu_e(-32.0, 5.7276907866586953775e-1), 1.5890407703711501803e-53, TEST_TOL2);
        check_result!(bessel_Jnu_e(-16.0, 8.6364032267280037696e-1), 6.9097436254460436398e-20, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 8.1065458967451038451e-2), 6.2316950805227520253e-58, TEST_TOL2);
        check_result!(bessel_Jnu_e(-17.0, 6.946415511078842723e-1), -4.3495405983099048944e-23, TEST_TOL2);
        check_result!(bessel_Jnu_e(-14.0, 2.7619565270505476723e-1), 1.0511271746088180311e-23, TEST_TOL2);
        check_result!(bessel_Jnu_e(-7.0, 8.579791601466317093e-2), -5.3039175204559594309e-14, TEST_TOL2);
        check_result!(bessel_Jnu_e(-50.0, 7.4681490716561041968e-1), 1.3331741219055661824e-86, TEST_TOL2);
        check_result!(bessel_Jnu_e(-30.0, 9.6631424224612452556e-1), 1.2465815577059566852e-42, TEST_TOL2);
        check_result!(bessel_Jnu_e(-14.0, 2.6745819874317492788e-1), 6.7024985048974981757e-24, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 3.6947309321414497419e-1), 6.4975710352073715961e-38, TEST_TOL2);
        check_result!(bessel_Jnu_e(-49.0, 2.3389602803779083655e-2), -3.5281224467005794333e-158, TEST_TOL2);
        check_result!(bessel_Jnu_e(-48.0, 7.7482504299905127354e-1), 1.3711857024107478291e-81, TEST_TOL2);
        check_result!(bessel_Jnu_e(-30.0, 6.5337950709230049969e-1), 9.9749347191786840601e-48, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 5.4117488569210959438e-1), -3.8843890347204927665e-48, TEST_TOL2);
        check_result!(bessel_Jnu_e(-50.0, 9.4038774092645791075e-1), 1.3455212768163778034e-81, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.0, 5.627399919447310892e-1), -2.703780920058947009e-1, TEST_TOL2);
        check_result!(bessel_Jnu_e(-35.0, 7.9925999507829895011e-2), -1.1060656306690664139e-89, TEST_TOL2);
        check_result!(bessel_Jnu_e(-10.0, 2.8875439067692705135e-3), 1.0844833648855202142e-35, TEST_TOL2);
        check_result!(bessel_Jnu_e(-31.0, 2.7645446745852278572e-1), -2.7740931384652548129e-61, TEST_TOL2);
        check_result!(bessel_Jnu_e(-12.0, 8.5374968606378578252e-1), 7.5366644001760905687e-14, TEST_TOL2);
        check_result!(bessel_Jnu_e(-11.0, 2.2997159465452075155e-1), -1.1626579415654480052e-18, TEST_TOL2);
        check_result!(bessel_Jnu_e(-47.0, 5.8616043503503357359e-1), -3.4270544836018351049e-85, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 8.4884849730214263521e-1), 1.8679667366331791405e-33, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 5.3943176031162412158e-1), 3.5300994115064965228e-38, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.0, 9.3387179309343112648e-1), -1.6062668879215016378e-2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-20.0, 5.5061917642540109891e-1), 2.5629166990385734238e-30, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 2.9315675715515822781e-1), 5.1505442739915338391e-49, TEST_TOL2);
        check_result!(bessel_Jnu_e(-6.0, 8.9079715253593128884e-1), 1.0539758479683914316e-5, TEST_TOL2);
        check_result!(bessel_Jnu_e(-30.0, 9.1220250697048036241e-2), 2.2291985671052015068e-73, TEST_TOL2);
        check_result!(bessel_Jnu_e(-49.0, 3.2934552845357396486e-1), -6.7628009346158039786e-102, TEST_TOL2);
        check_result!(bessel_Jnu_e(-20.0, 6.2008856084878849548e-3), 2.7691703032688977815e-69, TEST_TOL2);
        check_result!(bessel_Jnu_e(-23.0, 7.1620177578706903472e-1), -2.124755495743639839e-33, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 5.1124413385957329246e-1), -7.462204354283390559e-49, TEST_TOL2);
        check_result!(bessel_Jnu_e(-23.0, 6.7405321505832900803e-1), -5.2689769204867120958e-34, TEST_TOL2);
        check_result!(bessel_Jnu_e(-31.0, 7.6828173704641609615e-2), -1.600162678935095954e-78, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0, 7.8546641070654814244e-1), 9.610534504452577567e-4, TEST_TOL2);
        check_result!(bessel_Jnu_e(-39.0, 7.8708523270381710591e-1), -7.8237015591341025437e-63, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.0, 1.1973778137874968338e-1), 4.0918170073170305674e-15, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 7.9699790929323855652e-1), 1.4309765990568672215e-30, TEST_TOL2);
        check_result!(bessel_Jnu_e(-11.0, 4.4800161319259052168e-1), -1.7773520138988139169e-15, TEST_TOL2);
        check_result!(bessel_Jnu_e(-39.0, 9.2637892745877041811e-1), -4.4944192865215894733e-60, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 4.6475617572623309956e-1), -4.7026155154566357963e-50, TEST_TOL2);
        check_result!(bessel_Jnu_e(-25.0, 4.6435749048262147537e-1), -8.9952935327704632698e-42, TEST_TOL2);
        check_result!(bessel_Jnu_e(-28.0, 9.6511079209639008185e-1), 4.4842768640542298547e-39, TEST_TOL2);
        check_result!(bessel_Jnu_e(-36.0, 3.8411791760130458261e-2), 4.296403338839098526e-104, TEST_TOL2);
        check_result!(bessel_Jnu_e(-6.0, 2.891145018145052606e-1), 1.2636012998902815302e-8, TEST_TOL2);
        check_result!(bessel_Jnu_e(-43.0, 3.0866754076112185885e-1), -2.1010663709383681337e-88, TEST_TOL2);
        check_result!(bessel_Jnu_e(-44.0, 8.3972445789951961023e-1), 9.7813493638738341846e-72, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 8.2307293342676789357e-1), 2.9041436661554638719e-30, TEST_TOL2);
        check_result!(bessel_Jnu_e(-12.0, 9.8857250941935924585e-1), 4.357505306871049656e-13, TEST_TOL2);
        check_result!(bessel_Jnu_e(-47.0, 9.9901153675544108048e-1), -2.6090278787375958472e-74, TEST_TOL2);
        check_result!(bessel_Jnu_e(-44.0, 2.8286777063559952163e-1), 1.5832896249242322418e-92, TEST_TOL2);
        check_result!(bessel_Jnu_e(-10.0, 9.7638349947419439863e-1), 2.0735913941162063742e-10, TEST_TOL2);
        check_result!(bessel_Jnu_e(-36.0, 4.6452631984005212745e-1), 4.0190510760125038996e-65, TEST_TOL2);
        check_result!(bessel_Jnu_e(-25.0, 8.2954737598010312336e-1), -1.7855017422981225812e-35, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 4.2326977999795604681e-1), -3.1249531389372439734e-51, TEST_TOL2);
        check_result!(bessel_Jnu_e(-9.0, 6.339167576518494852e-1), -8.8082994072251150317e-11, TEST_TOL2);
        check_result!(bessel_Jnu_e(-27.0, 7.4535258299077637954e-1), -2.4368032520208918805e-40, TEST_TOL2);
        check_result!(bessel_Jnu_e(-45.0, 8.0998976704970278418e-1), -1.8024726219976677441e-74, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 5.3437414610693284794e-1), 2.8159486268019058346e-38, TEST_TOL2);
        check_result!(bessel_Jnu_e(-19.0, 3.6987646459831328184e-1), -9.7200835901252686409e-32, TEST_TOL2);
        check_result!(bessel_Jnu_e(-49.0, 4.7308684164506190021e-1), -3.4438316393709775799e-94, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 8.4616849424460901479e-1), 1.4469107537397962381e-67, TEST_TOL2);
        check_result!(bessel_Jnu_e(-39.0, 8.6819804427642418616e-1), -3.5837055310896411954e-61, TEST_TOL2);
        check_result!(bessel_Jnu_e(-28.0, 4.1490163307722590922e-1), 2.448154551562710141e-49, TEST_TOL2);
        check_result!(bessel_Jnu_e(-25.0, 6.312651668273163642e-1), -1.9374739456138691106e-38, TEST_TOL2);
        check_result!(bessel_Jnu_e(-32.0, 5.3525470388026220677e-1), 1.8191207037881755277e-54, TEST_TOL2);
        check_result!(bessel_Jnu_e(-13.0, 9.2605599515408535679e-3), -7.2212651931099339154e-41, TEST_TOL2);
        check_result!(bessel_Jnu_e(-37.0, 4.8798589647640992562e-1), -1.5614996577959031121e-66, TEST_TOL2);
        check_result!(bessel_Jnu_e(-37.0, 2.2096551301466541766e-2), -2.9050812034192451665e-116, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 1.8092873560926168207e-1), 1.0575388628113044972e-95, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 4.2954143428969324083e-1), 1.7857221060719548205e-36, TEST_TOL2);
        check_result!(bessel_Jnu_e(-41.0, 1.7704659657689619579e-1), -2.017674513721498041e-93, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.0, 8.8755155653793053987e-1), -1.3862799262620708632e-2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.0, 6.7617657704638521874e-1), -3.1913053561511127823e-1, TEST_TOL2);
        check_result!(bessel_Jnu_e(-34.0, 6.4464304038438048178e-1), 6.4622314654558520448e-56, TEST_TOL2);
        check_result!(bessel_Jnu_e(-41.0, 7.2625991432244163042e-1), -2.7337118024791538344e-68, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.0, 6.357184822423154937e-1), -5.2186206749718741014e-3, TEST_TOL2);
        check_result!(bessel_Jnu_e(-6.0, 8.4999778579632777264e-1), 7.9757193518631922586e-6, TEST_TOL2);
        check_result!(bessel_Jnu_e(-47.0, 4.9270801771231759268e-1), -9.7743102162756354643e-89, TEST_TOL2);
        check_result!(bessel_Jnu_e(-44.0, 3.0382431918975667824e-1), 3.6749344196700669304e-91, TEST_TOL2);
        check_result!(bessel_Jnu_e(-21.0, 8.3709075395163832807e-1), -2.2120291813240017972e-28, TEST_TOL2);
        check_result!(bessel_Jnu_e(-13.0, 2.2926361048823468174e-1), -9.4684470339645238166e-23, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.0, 7.7595731076113971064e-1), 7.1557648504788709828e-2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-13.0, 8.3155666144468474085e-1), -1.760187305034807398e-15, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 8.9229820511590331545e-1), 1.8952192929209682492e-36, TEST_TOL2);
        check_result!(bessel_Jnu_e(-9.0, 4.3091918823985208523e-1), -2.7448142505229531657e-12, TEST_TOL2);
        check_result!(bessel_Jnu_e(-45.0, 7.6232573842954975111e-1), -1.177044451508791199e-75, TEST_TOL2);
        check_result!(bessel_Jnu_e(-47.0, 8.5370192783467777094e-1), -1.6168449888151601463e-77, TEST_TOL2);
        check_result!(bessel_Jnu_e(-17.0, 7.0917926374340919579e-1), -6.1835780259077271289e-23, TEST_TOL2);
        check_result!(bessel_Jnu_e(-48.0, 5.3177634068823620691e-1), 1.9544175507861216812e-89, TEST_TOL2);
        check_result!(bessel_Jnu_e(-20.0, 8.6236563774769292261e-1), 2.0102896278761019978e-26, TEST_TOL2);
        check_result!(bessel_Jnu_e(-47.0, 1.9398034049650767996e-1), -9.1928516850183758066e-108, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 1.9059407555598432968e-1), 3.0818305203000064476e-44, TEST_TOL2);
        check_result!(bessel_Jnu_e(-28.0, 1.177497340192600765e-1), 1.1853471604859571152e-64, TEST_TOL2);
        check_result!(bessel_Jnu_e(-44.0, 5.4411719229619710879e-1), 5.0099597095462268336e-80, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 8.4096736569723091858e-1), 4.6598891827094030103e-30, TEST_TOL2);
        check_result!(bessel_Jnu_e(-14.0, 5.7336031747513286455e-1), 2.8892068362904543886e-19, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 4.3701161637218456507e-1), 2.2572337854881401663e-40, TEST_TOL2);
        check_result!(bessel_Jnu_e(-16.0, 5.5950502438849852065e-1), 6.6952184074205206877e-23, TEST_TOL2);
        check_result!(bessel_Jnu_e(-15.0, 7.2135709344094709909e-1), -1.724936657760223936e-19, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 8.9503358993826252397e-1), 1.5285906153951174029e-66, TEST_TOL2);
        check_result!(bessel_Jnu_e(-34.0, 8.8842662562391397862e-1), 3.5115599466639988263e-51, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 7.1820429322328243568e-1), 3.3906734873299682373e-35, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 6.9983283703407621113e-1), 4.9840979255532732037e-71, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 3.4956023377394930027e-1), -1.2169918834421082937e-53, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 3.1780802172157483391e-1), 1.0816638066322224274e-43, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 6.6747313617191922586e-1), 6.8258422034194326368e-72, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 9.8281798409069473598e-1), -1.2641970706335426485e-40, TEST_TOL2);
        check_result!(bessel_Jnu_e(-48.0, 9.3933236478420901552e-1), 1.4126090419557425431e-77, TEST_TOL2);
        check_result!(bessel_Jnu_e(-33.0, 7.9969562813573452357e-1), -8.3521436185818779821e-51, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0, 1.749832037277650746e-1), 2.4377505307831309992e-6, TEST_TOL2);
        check_result!(bessel_Jnu_e(-12.0, 7.4928517324325015606e-1), 1.578984980873870348e-14, TEST_TOL2);
        check_result!(bessel_Jnu_e(-50.0, 6.8668643284952988717e-1), 2.0060641365741957198e-88, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 4.628453813124784986e-1), 7.3802979097358757591e-44, TEST_TOL2);
        check_result!(bessel_Jnu_e(-17.0, 1.3398796901656815413e-1), -3.1005014564142463477e-35, TEST_TOL2);
        check_result!(bessel_Jnu_e(-21.0, 4.3589737907768555406e-1), -2.4897620016130572781e-34, TEST_TOL2);
        check_result!(bessel_Jnu_e(-49.0, 9.3471214811043877923e-1), -1.0635172631183545319e-79, TEST_TOL2);
        check_result!(bessel_Jnu_e(-12.0, 6.2842914244476056474e-1), 1.919020727030761691e-15, TEST_TOL2);
        check_result!(bessel_Jnu_e(-44.0, 3.9902397275715061045e-1), 5.9381636725852262522e-86, TEST_TOL2);
        check_result!(bessel_Jnu_e(-38.0, 7.4844742354073981342e-1), 1.1452741249870059158e-61, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 1.7024143837455386901e-1), 2.5696433432212310518e-45, TEST_TOL2);
        check_result!(bessel_Jnu_e(-35.0, 2.4663778723047911984e-1), -1.4846380265631517846e-72, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 3.4675532474808813305e-1), 7.7576502065450687145e-84, TEST_TOL2);
        check_result!(bessel_Jnu_e(-42.0, 9.7015007293565236242e-1), 4.5073367850753509233e-65, TEST_TOL2);
        check_result!(bessel_Jnu_e(-18.0, 5.8064934044500015204e-1), 3.3392049842730194442e-26, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0, 8.9712869996285071984e-1), 1.6201373653351794808e-3, TEST_TOL2);
        check_result!(bessel_Jnu_e(-14.0, 4.7509593095794841351e-1), 2.0819777331649343154e-20, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 1.9971440277743206076e-1), 1.5567772398263651029e-48, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.0, 3.1326041354180815314e-1), 1.2166506426643152762e-2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-21.0, 7.1780328035546876532e-1), -8.7820775346034440302e-30, TEST_TOL2);
        check_result!(bessel_Jnu_e(-40.0, 3.4783624449708255548e-1), 5.0330128895858604038e-79, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 4.6025606383259078342e-1), 7.8278566159609528116e-40, TEST_TOL2);
        check_result!(bessel_Jnu_e(-37.0, 6.8329209514074967672e-1), -4.0018049034521909865e-61, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 3.1644447969459523952e-1), 3.757803139189680586e-48, TEST_TOL2);
        check_result!(bessel_Jnu_e(-49.0, 3.8814659649103508174e-1), -2.1178321069354253493e-98, TEST_TOL2);
        check_result!(bessel_Jnu_e(-33.0, 2.2586340634075651258e-1), -6.3690605699693325702e-69, TEST_TOL2);
        check_result!(bessel_Jnu_e(-31.0, 1.1833425544176035201e-1), -1.0457450400633015896e-72, TEST_TOL2);
        check_result!(bessel_Jnu_e(-7.0, 4.716233225505345007e-1), -7.9892591292002198427e-9, TEST_TOL2);
        check_result!(bessel_Jnu_e(-28.0, 4.0216249780484400656e-1), 1.0224868057823447281e-49, TEST_TOL2);
        check_result!(bessel_Jnu_e(-39.0, 2.1728515555094074309e-1), -1.2424793343150735922e-84, TEST_TOL2);
        check_result!(bessel_Jnu_e(-10.0, 1.5286805658821812372e-1), 1.8744497113230339685e-18, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.0, 4.2012489177724585853e-1), 2.1740379601921820516e-2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.0, 5.4168160735476556955e-1), -3.2509626190739798323e-3, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 6.0999547254418081401e-1), 9.6515399655293906821e-41, TEST_TOL2);
        check_result!(bessel_Jnu_e(-31.0, 9.3599472437451867441e-1), -7.236873645285246215e-45, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.0, 8.9238535456317991508e-2), 9.9477909077321557346e-4, TEST_TOL2);
        check_result!(bessel_Jnu_e(-41.0, 3.3286697432119768766e-1), -3.5168501713472066379e-82, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 1.3103200887095798302e-2), 4.1630610278945554747e-84, TEST_TOL2);
        check_result!(bessel_Jnu_e(-7.0, 6.8545576155223653312e-1), -1.0860095433456484207e-7, TEST_TOL2);
        check_result!(bessel_Jnu_e(-33.0, 7.4597656684747976078e-1), -8.4232256181114982289e-52, TEST_TOL2);
        check_result!(bessel_Jnu_e(-35.0, 9.5265851504353628581e-1), -5.1260638475279101845e-52, TEST_TOL2);
        check_result!(bessel_Jnu_e(-5.0, 1.9993324513780069188e-2), -8.319296787329444617e-13, TEST_TOL2);
        check_result!(bessel_Jnu_e(-15.0, 7.291071285552115835e-2), -2.0411952376466778385e-34, TEST_TOL2);
        check_result!(bessel_Jnu_e(-5.0, 4.307852573603263607e-1), -3.8336545021181925733e-6, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.0, 3.0719264454074178501e-1), 7.6627991262305533713e-12, TEST_TOL2);
        check_result!(bessel_Jnu_e(-7.0, 2.9261260328577001029e-1), -2.8395431884068098274e-10, TEST_TOL2);
        check_result!(bessel_Jnu_e(-36.0, 3.4285828911893011822e-1), 7.1807133181285014617e-70, TEST_TOL2);
        check_result!(bessel_Jnu_e(-22.0, 2.1687887653368606307e-1), 5.2860475778514174667e-43, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 7.2816755037040510323e-1), 4.7187086299885949165e-35, TEST_TOL2);
        check_result!(bessel_Jnu_e(-18.0, 2.0826276232560462604e-2), 3.2368741843295077202e-52, TEST_TOL2);
        check_result!(bessel_Jnu_e(-9.0, 6.8082174052201672454e-1), -1.6719854980400483279e-10, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 1.1998114825675920571e-1), 4.2119340347581322841e-59, TEST_TOL2);
        check_result!(bessel_Jnu_e(-9.0, 5.9197600088654039875e-1), -4.7631865156190005935e-11, TEST_TOL2);
        check_result!(bessel_Jnu_e(-30.0, 1.2367288101522705215e-1), 2.0588316029270585207e-69, TEST_TOL2);
        check_result!(bessel_Jnu_e(-7.0, 8.3266930505292536647e-1), -4.2096524602233328394e-7, TEST_TOL2);
        check_result!(bessel_Jnu_e(-18.0, 4.360196631312459384e-1), 1.9281550661128359168e-28, TEST_TOL2);
        check_result!(bessel_Jnu_e(-13.0, 9.8501660515145040901e-1), -1.5833136710018445626e-14, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 9.3281324180154613247e-1), -2.7828395455119501545e-41, TEST_TOL2);
        check_result!(bessel_Jnu_e(-50.0, 8.9831019278310796215e-1), 1.3646576617083900982e-82, TEST_TOL2);
        check_result!(bessel_Jnu_e(-34.0, 8.1153956230382506493e-1), 1.6192058088789772833e-52, TEST_TOL2);
        check_result!(bessel_Jnu_e(-15.0, 9.5908894233909785027e-1), -1.2293883538807523551e-17, TEST_TOL2);
        check_result!(bessel_Jnu_e(-28.0, 4.7478353398916835208e-1), 1.0667274838088242221e-47, TEST_TOL2);
        check_result!(bessel_Jnu_e(-26.0, 3.1425431663890729964e-1), 3.137014371489532261e-48, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 6.8861856868877100233e-1), -4.2000859317520628674e-45, TEST_TOL2);
        check_result!(bessel_Jnu_e(-16.0, 6.9807655407582355921e-1), 2.3026948238804970982e-21, TEST_TOL2);
        check_result!(bessel_Jnu_e(-15.0, 1.9223628937777433793e-1), -4.2201734817670464106e-28, TEST_TOL2);
        check_result!(bessel_Jnu_e(-24.0, 7.91209811831343471e-1), 3.458241440092889033e-34, TEST_TOL2);
        check_result!(bessel_Jnu_e(-12.0, 2.8881796002183274727e-1), 1.7143390913163291276e-19, TEST_TOL2);
        check_result!(bessel_Jnu_e(-34.0, 3.6891378721647167497e-1), 3.7139793083014182422e-64, TEST_TOL2);
        check_result!(bessel_Jnu_e(-20.0, 8.4841223828616526898e-1), 1.4510812436551651903e-26, TEST_TOL2);
        check_result!(bessel_Jnu_e(-23.0, 2.2490195812594682131e-1), -5.7468688920782767025e-45, TEST_TOL2);
        check_result!(bessel_Jnu_e(-18.0, 2.2504144134842978217e-1), 1.3048322081397375779e-33, TEST_TOL2);
        check_result!(bessel_Jnu_e(-41.0, 8.9085721717774902491e-1), -1.1841910084346823163e-64, TEST_TOL2);
        check_result!(bessel_Jnu_e(-14.0, 3.5776817082613807574e-1), 3.9325021938284721675e-22, TEST_TOL2);
        check_result!(bessel_Jnu_e(-17.0, 4.6898364389788035003e-1), -5.492570150236103145e-26, TEST_TOL2);
        check_result!(bessel_Jnu_e(-6.0, 7.4085998179632632531e-1), 3.5186865149767756957e-6, TEST_TOL2);
        check_result!(bessel_Jnu_e(-29.0, 8.1247678941673114604e-1), -5.0783189409391835819e-43, TEST_TOL2);
        check_result!(bessel_Jnu_e(-6.0, 1.7590874156867732351e-2), 6.4299450335557031571e-16, TEST_TOL2);
        check_result!(bessel_Jnu_e(-6.0, 4.1122931368227349961e-2), 1.0494595145859932593e-13, TEST_TOL2);
        check_result!(bessel_Jnu_e(-18.0, 3.4357780610013843947e-2), 2.6519427947417673311e-48, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.0, 7.2901630769663700817e-1), 7.6159005881302088369e-9, TEST_TOL2);
        check_result!(bessel_Jnu_e(-40.0, 6.2434787548617066655e-1), 7.297739135890827566e-69, TEST_TOL2);
        check_result!(bessel_Jnu_e(-37.0, 2.5717302633809380684e-1), -7.9671811532819427071e-77, TEST_TOL2);
        check_result!(bessel_Jnu_e(-12.0, 4.4871088635019795091e-1), 3.3823657137507787902e-17, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.7408502287167772557e+1, 4.3891036254061936271e-2), -1.5118966152679114528e+42, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0369289750688261206e+1, 8.617077861907621132e-1), 1.3458137581188368176e+61, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.6398263821779503158, 5.7108775125890870648e-1), -1.1073338178875224514e+2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.0743837616631487936e+1, 2.2372123946196517647e-1), 1.3987244312547157439e+37, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.7297952956642177128e+1, 2.8440280848533973972e-1), -5.5832331287880973953e+27, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.3507072230824139103e+1, 9.7008738913549467403e-1), -9.9108981827284991851e+12, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.4663106025697138284e+1, 9.6739483983540323655e-1), 2.5305841722999151766e+67, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.7450767905208691173e+1, 5.2288309478685515663e-3), -3.4541920228396234708e+180, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.8736914274754280581e+1, 4.4318942692773250336e-1), 1.2783291424623089209e+27, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.4560791710276042462e+1, 5.6595841783863809163e-1), 1.7364781221343685309e+56, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.8723165418996042381e+1, 2.4003201116391659975e-1), 8.229650479070536446e+54, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.1780816480084454714e+1, 3.5000033756039229564e-1), -8.9158096963672457852e+56, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.3572537806135923827, 8.994859317735841446e-1), 2.4471474432717596765e+6, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.9179499652113791027e+1, 7.3466044408596358731e-1), -1.0446080588162613503e+82, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.0204535104461498585e+1, 1.3316368076269799096e-1), 1.6723180404777533538e+93, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.9597169419874779916e+1, 8.3895848736941180651e-1), -1.9885394381212418599e+80, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.2228113163441444643e+1, 7.2360070820350912315e-1), 3.7033741801434815187e+12, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.3252347726870680973e+1, 4.7042383176138740544e-2), -2.2524439080867705956e+121, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.8363275112470146292e+1, 3.5406545166014987925e-1), 7.0915928368505654149e+95, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.4031313414732363799e+1, 1.7077658851102128503e-1), 4.2707681524978432343e+46, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.4994220322161396103e+1, 7.6004498361697560048e-2), 8.3491267575601512811e+85, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.5842451866339926986e+1, 7.1655842470093644641e-1), -1.4956016465164596038e+18, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.3303969013414183336e+1, 3.4261981308825774017e-1), -1.7313464383524562686e+84, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.9620214546900609668e+1, 8.9559048893071335969e-2), -6.5439278427877993956e+69, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.9267775155252921091e+1, 4.9173394917277714574e-1), -2.7879726255003962141e+68, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.085805422677201981e+1, 7.1844728658692273953e-2), 1.7330833141098585591e+21, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.3205222720657600449e+1, 2.0938011160695718605e-1), -1.2855953290893319419e+93, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.1761832688338363731e-1, 4.0692821730479031008e-1), 1.0616114810207300625, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.4176152886105712673e+1, 7.3083748089885469319e-1), 2.3708170326600879156e+51, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.7495316392965243753e+1, 3.6374757654471397858e-1), -2.4050181588419272908e+26, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.4363999308889822481e+1, 2.2964635433363436307e-1), 1.4858445128296594446e+23, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.5945782535187393546e+1, 5.5004988492253222851e-1), -5.3196903529172738965e+19, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.6539439893219625339e+1, 5.4022213494661757887e-1), 3.4606719889912371036e+40, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.9104819423206076833e+1, 8.7581079115207012305e-1), -8.3806822204633705862e+57, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.1344422530419629852e+1, 1.8412292138063948156e-1), -1.3032526695489281999e+18, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.2479004807998664153e+1, 7.628052499161305046e-1), 3.8593605090529013702e+67, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.7234292208462683278e+1, 5.6929354834881763134e-1), -1.3560087920173394791e+41, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.9852923491811760298e+1, 2.1764339448558498796e-2), -3.1065720979758225192e+88, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.9470737712526065987e+1, 9.1397839227827761003e-1), -4.9854244578384505794e+39, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.4879459756439547254e+1, 8.7694253508024822462e-1), 4.0540656704233640216e+31, TEST_TOL2);
        check_result!(bessel_Jnu_e(-8.6562240771489400173e-1, 6.4882619355798588927e-1), 9.5827819637209987022e-2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.4096305256724256786e+1, 1.1837901256079790395e-1), 1.5389662008468777023e+26, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.3739240782592000796e+1, 1.1830951764837136725e-1), -5.4851415830067607572e+25, TEST_TOL2);
        check_result!(bessel_Jnu_e(-7.7683272384817803185e-1, 2.0897180603218726575e-1), 1.3452855819917342033, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.9007825894244022613e+1, 8.0804305816981973945e-1), -1.9558153171413640836e+78, TEST_TOL3);
        check_result!(bessel_Jnu_e(-1.224872379992519031, 9.4716012050013901355e-1), -8.7507643021583199242e-1, TEST_TOL2);
        check_result!(bessel_Jnu_e(-5.0807600398060325807e+1, 6.1902119795226148946e-1), 1.9558680407989173708e+89, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.2130610423140110162e+1, 7.2184881647444607846e-1), 3.0709609117301881893e+67, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.1186300874143057513e+1, 1.3944550368322800884e-1), -1.2405415201132534983e+95, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.1777582295815773824e+1, 7.6178874271077561762e-1), -7.1748063501973138583e+27, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.2185912810368133222e+1, 3.959510541558671016e-1), 1.196451653185591802e+56, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.2976006083946226988e+1, 4.5739390828369379657e-1), 1.0599129365585919882e+77, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.0412232215064606945e+1, 6.7510758855896918145e-1), 2.4302601636670462267e+45, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.8388188389747281955e+1, 8.9233979909724013718e-1), 9.1410432331502484426e+20, TEST_TOL2);
        check_result!(bessel_Jnu_e(-2.4298840569257253701e+1, 6.8042862360863559591e-1), 4.0995648850574613979e+33, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.8834735272673504063e+1, 4.2324469410479691518e-1), 7.0355133597135130631e+69, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.6091840934431606344e+1, 1.7508918790902548661e-1), 8.7456315137421979067e+103, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.9754264394942756728, 1.558717798933954111e-2), -3.551027943747170162e+2, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.7168594342366560374e+1, 6.976373865080374073e-1), -1.1036447967023475572e+58, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.5379944292029245754e+1, 5.3150471205968938472e-2), -1.0469743921754287032e+126, TEST_TOL2);
        check_result!(bessel_Jnu_e(-1.153181439556533474e+1, 1.8204366094125528818e-1), -4.0986515168430307785e+18, TEST_TOL2);
        check_result!(bessel_Jnu_e(-4.9939680334734766385e+1, 8.132277926604580844e-1), -9.5179038651143567503e+80, TEST_TOL2);
        check_result!(bessel_Jnu_e(-3.5986707652967270442e+1, 5.5665988728905782182e-1), -1.27797927382078249e+58, TEST_TOL3);
        check_result!(bessel_Jnu_e(-6.7046620273111013832, 1.059530133767196237e-1), 3.8106055649273069958e+10, TEST_TOL2);
    }

    #[test]
    fn test_bessel_k1_scaled_e() {
        check_result!(bessel_k1_scaled_e(0.1), 172.78759594743862812, TEST_TOL0);
        check_result!(bessel_k1_scaled_e(2.0), 1.1780972450961724644, TEST_TOL0);
        check_result!(bessel_k1_scaled_e(100.0), 0.015865042900628455854, TEST_TOL0);
    }

    #[test]
    fn test_bessel_ln_Knu_e() {
        check_result!(bessel_ln_Knu_e(0.0001, 1.0e-100), 5.439794449319847, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(0.0001, 0.0001), 2.232835507214331, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(0.0001, 10.0), -10.93743282256098, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(1.0, 1.0e-100), 230.2585092994045, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(1.0, 1.0e-10), 23.025850929940456840, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(1.0, 0.001), 6.907751517131146, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(1.0, 1.0), -0.5076519482107523309, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(30.0, 1.0e-100), 6999.113586185543475, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(30.0, 1.0), 91.34968784026325464, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(30.0, 100.0), -97.63224126416760932, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(100.0, 1.0e-100), 23453.606706185466825, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(100.0, 1.0), 427.7532510250188083, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(100.0, 100.0), -55.53422771502921431, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(1000.0, 1.0e-100), 236856.183755993135, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(10000.0, 1.0e-100), 2.39161558914890695e+06, TEST_TOL0);
        check_result!(bessel_ln_Knu_e(180.0, 2.2), 735.1994170369583930752590258, TEST_TOL1);
        check_result!(bessel_ln_Knu_e(3500.5, 1500.0), 1731.220077116482710070986699, TEST_TOL1);
    }

    #[test]
    fn test_bessel_j0_e() {
        check_result!(bessel_j0_e(-10.0), -0.05440211108893698134, TEST_TOL0);
        check_result!(bessel_j0_e(0.001), 0.9999998333333416667, TEST_TOL0);
        check_result!(bessel_j0_e(1.0), 0.84147098480789650670, TEST_TOL0);
        check_result!(bessel_j0_e(10.0), -0.05440211108893698134, TEST_TOL0);
        check_result!(bessel_j0_e(100.0), -0.005063656411097587937, TEST_TOL1);
        check_result!(bessel_j0_e(1048576.0), 3.1518281938718287624e-07, TEST_TOL2);
        check_result!(bessel_j0_e(1.0e18), -9.9296932074040507620955e-19, TEST_TOL0);
        check_result!(bessel_j0_e(1.0e20), -6.4525128526578084420581e-21, TEST_TOL0);
    }

    #[test]
    fn test_bessel_Knu_e() {
        check_result!(bessel_Knu_e(0.0001, 0.001), 7.023689431812884141, TEST_TOL0);
        check_result!(bessel_Knu_e(0.0001, 10.0), 0.000017780062324654874306, TEST_TOL0);
        check_result!(bessel_Knu_e(1.0, 0.001), 999.9962381560855743, TEST_TOL0);
        check_result!(bessel_Knu_e(1.0, 1.0), 0.6019072301972345747, TEST_TOL0);
        check_result!(bessel_Knu_e(10.0, 0.001), 1.8579455483904008064e+38, TEST_TOL0);
        check_result!(bessel_Knu_e(10.0, 1.0), 1.8071328990102945469e+08, TEST_TOL0);
        check_result!(bessel_Knu_e(10.0, 100.0), 7.655427977388100611e-45, TEST_TOL2);
        check_result!(bessel_Knu_e(10.2, 100.0), 7.810600225948217841e-45, TEST_TOL2);
        check_result!(bessel_Knu_e(30.0, 1.0), 4.706145526783626883e+39, TEST_TOL1);
        check_result!(bessel_Knu_e(30.0, 100.0), 3.970602055959398739e-43, TEST_TOL2);
    }

    #[test]
    fn test_bessel_il_scaled_e() {
        check_result!(bessel_il_scaled_e(0, 0.0), 1.0, TEST_TOL0);
        check_result!(bessel_il_scaled_e(1, 0.0), 0.0, TEST_TOL0);
        check_result!(bessel_il_scaled_e(4, 0.001), 1.0571434341190365013e-15, TEST_TOL0);
        check_result!(bessel_il_scaled_e(4, 0.1), 9.579352242057134927e-08, TEST_TOL1);
        check_result!(bessel_il_scaled_e(5, 2.0), 0.0004851564602127540059, TEST_TOL0);
        check_result!(bessel_il_scaled_e(5, 100.0), 0.004300446777500000000, TEST_TOL1);
        check_result!(bessel_il_scaled_e(100, 100.0), 1.3898161964299132789e-23, TEST_TOL0);
    }

    #[test]
    fn test_bessel_i0_scaled_e() {
        check_result!(bessel_i0_scaled_e(0.0), 1.0, TEST_TOL0);
        check_result!(bessel_i0_scaled_e(0.1), 0.9063462346100907067, TEST_TOL0);
        check_result!(bessel_i0_scaled_e(2.0), 0.24542109027781645493, TEST_TOL0);
        check_result!(bessel_i0_scaled_e(100.0), 0.005000000000000000000, TEST_TOL0);
    }

    #[test]
    fn test_bessel_K1_e() {
        check_result!(bessel_K1_e(0.1), 9.853844780870606135, TEST_TOL0);
        check_result!(bessel_K1_e(1.95), 0.1494001409315894276793, TEST_TOL0);
        check_result!(bessel_K1_e(2.0), 0.13986588181652242728, TEST_TOL0);
        check_result!(bessel_K1_e(100.0), 4.679853735636909287e-45, TEST_TOL2);
    }

    #[test]
    fn test_bessel_Kn_e() {
        check_result!(bessel_Kn_e(4, 0.1), 479600.2497925682849, TEST_TOL1);
        check_result!(bessel_Kn_e(5, 2.0), 9.431049100596467443, TEST_TOL0);
        check_result!(bessel_Kn_e(100, 100.0), 7.617129630494085416e-25, TEST_TOL2);
    }

    #[test]
    fn test_bessel_Inu_e() {
        check_result!(bessel_Inu_e(0.0001, 10.0), 2815.7166269770030352, TEST_TOL0);
        check_result!(bessel_Inu_e(1.0, 0.001), 0.0005000000625000026042, TEST_TOL0);
        check_result!(bessel_Inu_e(1.0, 1.0), 0.5651591039924850272, TEST_TOL0);
        check_result!(bessel_Inu_e(30.0, 1.0), 3.539500588106447747e-42, TEST_TOL0);
        check_result!(bessel_Inu_e(30.0, 100.0), 1.2061548704498434006e+40, TEST_TOL2);
        check_result!(bessel_Inu_e(10.0, 1.0), 2.7529480398368736252e-10, TEST_TOL0);
        check_result!(bessel_Inu_e(10.0, 100.0), 6.498975524720147799e+41, TEST_TOL2);
        check_result!(bessel_Inu_e(10.2, 100.0), 6.368587361287030443e+41, TEST_TOL2);
    }

    #[test]
    fn test_bessel_j2_e() {
        check_result!(bessel_j2_e(-10.0), 0.07794219362856244547, TEST_TOL0);
        check_result!(bessel_j2_e(0.01), 6.666619047751322551e-06, TEST_TOL0);
        check_result!(bessel_j2_e(1.0), 0.06203505201137386110, TEST_TOL0);
        check_result!(bessel_j2_e(10.0), 0.07794219362856244547, TEST_TOL0);
        check_result!(bessel_j2_e(100.0), 0.004803441652487953480, TEST_TOL1);
        check_result!(bessel_j2_e(1048576.0), -3.1518539455252413111e-07, TEST_TOL3);
        check_result!(bessel_j2_e(-1.0e22), 5.23214785395139e-23, TEST_TOL2);
        check_result!(bessel_j2_e(-1.0e21), 7.449501119831337e-22, TEST_TOL2);
        check_result!(bessel_j2_e(-1.0e20), 7.639704044417282e-21, TEST_TOL2);
        check_result!(bessel_j2_e(-1.0e19), -3.749051695507179e-20, TEST_TOL2);
        check_result!(bessel_j2_e(1.0e19), -3.749051695507179e-20, TEST_TOL2);
        check_result!(bessel_j2_e(1.0e20), 7.639704044417282e-21, TEST_TOL2);
        check_result!(bessel_j2_e(1.0e21), 7.449501119831337e-22, TEST_TOL2);
        check_result!(bessel_j2_e(1.0e22), 5.23214785395139e-23, TEST_TOL2);
    }

    #[test]
    fn test_bessel_In_e() {
        check_result!(bessel_In_e(4, 0.1), 2.6054690212996573677e-07, TEST_TOL0);
        check_result!(bessel_In_e(5, 2.0), 0.009825679323131702321, TEST_TOL0);
        check_result!(bessel_In_e(100, 100.0), 4.641534941616199114e+21, TEST_TOL2);
    }

    #[test]
    fn test_bessel_k0_scaled_e() {
        check_result!(bessel_k0_scaled_e(0.1), 15.707963267948966192, TEST_TOL0);
        check_result!(bessel_k0_scaled_e(2.0), 0.7853981633974483096, TEST_TOL0);
        check_result!(bessel_k0_scaled_e(100.0), 0.015707963267948966192, TEST_TOL0);
    }

    #[test]
    fn test_bessel_Jn_e() {
        check_result!(bessel_Jn_e(4, 0.1), 2.6028648545684032338e-07, TEST_TOL0);
        check_result!(bessel_Jn_e(5, 2.0), 0.007039629755871685484, TEST_TOL0);
        check_result!(bessel_Jn_e(10, 20.0), 0.18648255802394508321, TEST_TOL0);
        check_result!(bessel_Jn_e(100, 100.0), 0.09636667329586155967, TEST_TOL0);
        check_result!(bessel_Jn_e(2, 900.0), -0.019974345269680646400, TEST_TOL4);
        check_result!(bessel_Jn_e(2, 15000.0), -0.0020455820181216382666, TEST_TOL4);
        check_result!(bessel_Jn_e(0, 1.0e+10), 2.1755917502468917269e-06, TEST_SQRT_TOL0);
        check_result!(bessel_Jn_e(1, 1.0e+10), -7.676508175684157103e-06, TEST_TOL4);
        check_result!(bessel_Jn_e(0, 20000.0), 0.00556597490495494615709982972, TEST_TOL4);
        check_result!(bessel_Jn_e(45, 900.0), 0.02562434700634278108, TEST_TOL0);
    }

    #[test]
    fn test_bessel_j1_e() {
        check_result!(bessel_j1_e(-10.0), -0.07846694179875154709, TEST_TOL0);
        check_result!(bessel_j1_e(0.01), 0.003333300000119047399, TEST_TOL0);
        check_result!(bessel_j1_e(1.0), 0.30116867893975678925, TEST_TOL0);
        check_result!(bessel_j1_e(10.0), 0.07846694179875154709, TEST_TOL0);
        check_result!(bessel_j1_e(100.0), -0.008673825286987815220, TEST_TOL0);
        check_result!(bessel_j1_e(1048576.0), -9.000855242905546158e-07, TEST_TOL0);
        check_result!(bessel_j1_e(1.0e18), -1.183719902187107336049e-19, TEST_TOL0);
    }

    #[test]
    fn test_bessel_I1_scaled_e() {
        check_result!(bessel_I1_scaled_e(0.1), 0.04529844680880932501, TEST_TOL0);
        check_result!(bessel_I1_scaled_e(2.0), 0.21526928924893765916, TEST_TOL0);
        check_result!(bessel_I1_scaled_e(100.0), 0.03974415302513025267, TEST_TOL0);
        check_result!(bessel_I1_scaled_e(65536.0), 0.0015583593657207350452, TEST_TOL0);
    }

    #[test]
    fn test_bessel_kl_scaled_e() {
        check_result!(bessel_kl_scaled_e(4, 1.0/256.0), 1.8205599816961954439e+14, TEST_TOL0);
        check_result!(bessel_kl_scaled_e(4, 1.0/8.0), 6.1173217814406597530e+06, TEST_TOL0);
        check_result!(bessel_kl_scaled_e(5, 2.0), 138.10735829492005119, TEST_TOL0);
        check_result!(bessel_kl_scaled_e(100, 100.0), 3.985930768060258219e+18, TEST_TOL1);
    }

    #[test]
    fn test_bessel_y2_e() {
        check_result!(bessel_y2_e(0.01), -3.0000500012499791668e+06, TEST_TOL0);
        check_result!(bessel_y2_e(1.0), -3.605017566159968955, TEST_TOL0);
        check_result!(bessel_y2_e(10.0), -0.06506930499373479347, TEST_TOL0);
        check_result!(bessel_y2_e(100.0), 0.008772511458592903927, TEST_TOL0);
        check_result!(bessel_y2_e(4294967296.0), -2.0649445123857054207e-10, TEST_SQRT_TOL0);
    }

    #[test]
    fn test_bessel_i1_scaled_e() {
        check_result!(bessel_i1_scaled_e(0.0), 0.0, TEST_TOL0);
        check_result!(bessel_i1_scaled_e(0.1), 0.030191419289002226846, TEST_TOL0);
        check_result!(bessel_i1_scaled_e(2.0), 0.131868364583275317610, TEST_TOL0);
        check_result!(bessel_i1_scaled_e(100.0), 0.004950000000000000000, TEST_TOL0);
    }

    #[test]
    fn test_bessel_y0_e() {
        check_result!(bessel_y0_e(0.001), -999.99950000004166670, TEST_TOL0);
        check_result!(bessel_y0_e(1.0), -0.5403023058681397174, TEST_TOL0);
        check_result!(bessel_y0_e(10.0), 0.08390715290764524523, TEST_TOL0);
        check_result!(bessel_y0_e(100.0), -0.008623188722876839341, TEST_TOL0);
        check_result!(bessel_y0_e(65536.0), 0.000011014324202158573930, TEST_TOL0);
        check_result!(bessel_y0_e(4294967296.0), 2.0649445131370357007e-10, TEST_SQRT_TOL0);
    }

    #[test]
    fn test_bessel_k2_scaled_e() {
        check_result!(bessel_k2_scaled_e(0.1), 5199.335841691107810, TEST_TOL0);
        check_result!(bessel_k2_scaled_e(2.0), 2.5525440310417070063, TEST_TOL0);
        check_result!(bessel_k2_scaled_e(100.0), 0.016183914554967819868, TEST_TOL0);
    }

    #[test]
    fn test_bessel_J1_e() {
        check_result!(bessel_J1_e(0.1), 0.04993752603624199756, TEST_TOL0);
        check_result!(bessel_J1_e(2.0), 0.57672480775687338720, TEST_TOL0);
        check_result!(bessel_J1_e(100.0), -0.07714535201411215803, TEST_TOL0);
        check_result!(bessel_J1_e(1.0e+10), -7.676508175684157103e-06, TEST_TOL4);
    }

    #[test]
    fn test_bessel_Y1_e() {
        check_result!(bessel_Y1_e(0.1), -6.45895109470202698800, TEST_TOL0);
        check_result!(bessel_Y1_e(2.0), -0.10703243154093754689, TEST_TOL0);
        check_result!(bessel_Y1_e(100.0), -0.020372312002759793305, TEST_TOL0);
        check_result!(bessel_Y1_e(4294967296.0), 0.000011612249378370766284, TEST_TOL4);
    }
}