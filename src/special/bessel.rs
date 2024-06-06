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


#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular cylindrical Bessel function of zeroth
/// order, $J\_0(x)$.
///
/// Binds the [`gsl_sf_bessel_J0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_J0_e).
#[allow(non_snake_case)]
pub fn bessel_J0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_J0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular cylindrical Bessel function of first
/// order, $J\_1(x)$.
///
/// Binds the [`gsl_sf_bessel_J1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_J1_e).
#[allow(non_snake_case)]
pub fn bessel_J1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_J1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular cylindrical Bessel function of
/// order `n`, $J\_n(x)$.
///
/// Binds the [`gsl_sf_bessel_Jn_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Jn_e).
#[allow(non_snake_case)]
pub fn bessel_Jn(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Jn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular cylindrical Bessel function of zeroth
/// order, $Y\_0(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_Y0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Y0_e).
#[allow(non_snake_case)]
pub fn bessel_Y0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Y0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular cylindrical Bessel function of first
/// order, $Y\_1(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_Y1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Y1_e).
#[allow(non_snake_case)]
pub fn bessel_Y1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Y1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular cylindrical Bessel function of
/// order `n`, $Y\_n(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_Yn_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Yn_e).
#[allow(non_snake_case)]
pub fn bessel_Yn(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Yn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular modified cylindrical Bessel function
/// of zeroth order, $I\_0(x)$.
///
/// Binds the [`gsl_sf_bessel_I0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_I0_e).
#[allow(non_snake_case)]
pub fn bessel_I0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular modified cylindrical Bessel function
/// of first order, $I\_1(x)$.
///
/// Binds the [`gsl_sf_bessel_I1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_I1_e).
#[allow(non_snake_case)]
pub fn bessel_I1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular modified cylindrical Bessel function
/// of order `n`, $I\_n(x)$.
///
/// Binds the [`gsl_sf_bessel_In_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_In_e).
#[allow(non_snake_case)]
pub fn bessel_In(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_In_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified cylindrical Bessel
/// function of zeroth order $\exp(-|x|) I\_0(x)$.
///
/// Binds the [`gsl_sf_bessel_I0_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_I0_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_I0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified cylindrical Bessel
/// function of first order $\exp(-|x|) I\_1(x)$.
///
/// Binds the [`gsl_sf_bessel_I1_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_I1_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_I1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_I1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified cylindrical Bessel
/// function of order `n`, $\exp(-|x|) I\_n(x)$
///
/// Binds the [`gsl_sf_bessel_In_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_In_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_In_scaled(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_In_scaled_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular modified cylindrical Bessel
/// function of zeroth order, $K\_0(x)$, for $x > 0$.
///
/// Binds the [`gsl_sf_bessel_K0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_K0_e).
#[allow(non_snake_case)]
pub fn bessel_K0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular modified cylindrical Bessel
/// function of first order, $K\_1(x)$, for $x > 0$.
///
/// Binds the [`gsl_sf_bessel_K1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_K1_e).
#[allow(non_snake_case)]
pub fn bessel_K1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular modified cylindrical Bessel
/// function of order `n`, $K\_n(x)$, for $x > 0$.
///
/// Binds the [`gsl_sf_bessel_Kn_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Kn_e).
#[allow(non_snake_case)]
pub fn bessel_Kn(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Kn_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified cylindrical Bessel
/// function of zeroth order $\exp(x) K\_0(x)$ for $x>0$.
///
/// Binds the [`gsl_sf_bessel_K0_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_K0_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_K0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified cylindrical Bessel
/// function of first order $\exp(x) K\_1(x)$ for $x>0$.
///
/// Binds the [`gsl_sf_bessel_K1_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_K1_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_K1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_K1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified cylindrical Bessel
/// function of order `n`, $\exp(x) K\_n(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_Kn_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Kn_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_Kn_scaled(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Kn_scaled_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular spherical Bessel function of zeroth
/// order, $j\_0(x) = \sin(x)/x$.
///
/// Binds the [`gsl_sf_bessel_j0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_j0_e).
pub fn bessel_j0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_j0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular spherical Bessel function of first
/// order, $j\_1(x) = (\sin(x)/x - \cos(x))/x$.
///
/// Binds the [`gsl_sf_bessel_j1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_j1_e).
pub fn bessel_j1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_j1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular spherical Bessel function of second
/// order, $j\_2(x) = ((3/x^2 - 1)\sin(x) - 3\cos(x)/x)/x$.
///
/// Binds the [`gsl_sf_bessel_j2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_j2_e).
pub fn bessel_j2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_j2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular spherical Bessel function of
/// order `l`, $j\_l(x)$, for
/// $l \geq 0$ and $x \geq 0$.
///
/// Binds the [`gsl_sf_bessel_jl_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_jl_e).
pub fn bessel_jl(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_jl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular spherical Bessel function of zeroth
/// order, $y\_0(x) = -\cos(x)/x$.
///
/// Binds the [`gsl_sf_bessel_y0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_y0_e).
pub fn bessel_y0(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_y0_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular spherical Bessel function of first
/// order, $y\_1(x) = -(\cos(x)/x + \sin(x))/x$.
///
/// Binds the [`gsl_sf_bessel_y1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_y1_e).
pub fn bessel_y1(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_y1_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular spherical Bessel function of second
/// order, $y\_2(x) = (-3/x^3 + 1/x)\cos(x) - (3/x^2)\sin(x)$.
///
/// Binds the [`gsl_sf_bessel_y2_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_y2_e).
pub fn bessel_y2(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_y2_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular spherical Bessel function of
/// order `l`, $y\_l(x)$, for
/// $l \geq 0$.
///
/// Binds the [`gsl_sf_bessel_yl_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_yl_e).
pub fn bessel_yl(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_yl_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified spherical Bessel
/// function of zeroth order, $\exp(-|x|) i\_0(x)$.
///
/// Binds the [`gsl_sf_bessel_i0_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_i0_scaled_e).
pub fn bessel_i0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_i0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified spherical Bessel
/// function of first order, $\exp(-|x|) i\_1(x)$.
///
/// Binds the [`gsl_sf_bessel_i1_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_i1_scaled_e).
pub fn bessel_i1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_i1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified spherical Bessel
/// function of second order, $\exp(-|x|) i\_2(x)$
///
/// Binds the [`gsl_sf_bessel_i2_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_i2_scaled_e).
pub fn bessel_i2_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_i2_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified spherical Bessel
/// function of order `l`, $\exp(-|x|) i\_l(x)$
///
/// Binds the [`gsl_sf_bessel_il_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_il_scaled_e).
pub fn bessel_il_scaled(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_il_scaled_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified spherical Bessel
/// function of zeroth order, $\exp(x) k\_0(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_k0_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_k0_scaled_e).
pub fn bessel_k0_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_k0_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified spherical Bessel
/// function of first order, $\exp(x) k\_1(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_k1_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_k1_scaled_e).
pub fn bessel_k1_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_k1_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified spherical Bessel
/// function of second order, $\exp(x) k\_2(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_k2_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_k2_scaled_e).
pub fn bessel_k2_scaled(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_k2_scaled_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified spherical Bessel
/// function of order `l`, $\exp(x) k\_l(x)$, for $x>0$.
///
/// Binds the [`gsl_sf_bessel_kl_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_kl_scaled_e).
pub fn bessel_kl_scaled(l: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_kl_scaled_e(l, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular cylindrical Bessel function of
/// fractional order $\nu$, $J\_\nu(x)$.
///
/// Binds the [`gsl_sf_bessel_Jnu_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Jnu_e).
#[allow(non_snake_case)]
pub fn bessel_Jnu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Jnu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular cylindrical Bessel function of
/// fractional order $\nu$, $Y\_\nu(x)$.
///
/// Binds the [`gsl_sf_bessel_Ynu_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Ynu_e).
#[allow(non_snake_case)]
pub fn bessel_Ynu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Ynu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled regular modified Bessel function of
/// fractional order $\nu$, $\exp(-|x|)I\_\nu(x)$ for $x>0$,
/// $\nu>0$.
///
/// Binds the [`gsl_sf_bessel_Inu_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Inu_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_Inu_scaled(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Inu_scaled_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regular modified Bessel function of
/// fractional order $\nu$, $I\_\nu(x)$ for $x>0$,
/// $\nu>0$.
///
/// Binds the [`gsl_sf_bessel_Inu_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Inu_e).
#[allow(non_snake_case)]
pub fn bessel_Inu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Inu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the scaled irregular modified Bessel function of
/// fractional order $\nu$, $\exp(+|x|) K\_\nu(x)$ for $x>0$,
/// $\nu>0$.
///
/// Binds the [`gsl_sf_bessel_Knu_scaled_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Knu_scaled_e).
#[allow(non_snake_case)]
pub fn bessel_Knu_scaled(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Knu_scaled_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the irregular modified Bessel function of
/// fractional order $\nu$, $K\_\nu(x)$ for $x>0$,
/// $\nu>0$.
///
/// Binds the [`gsl_sf_bessel_Knu_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_Knu_e).
#[allow(non_snake_case)]
pub fn bessel_Knu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_Knu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the irregular modified Bessel
/// function of fractional order $\nu$, $\ln(K\_\nu(x))$ for
/// $x>0$, $\nu>0$.
///
/// Binds the [`gsl_sf_bessel_lnKnu_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_lnKnu_e).
#[allow(non_snake_case)]
pub fn bessel_lnKnu(nu: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_lnKnu_e(nu, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th positive zero of
/// the Bessel function $J\_0(x)$.
///
/// Binds the [`gsl_sf_bessel_zero_J0_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_zero_J0_e).
#[allow(non_snake_case)]
pub fn bessel_zero_J0(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_zero_J0_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th positive zero of
/// the Bessel function $J\_1(x)$.
///
/// Binds the [`gsl_sf_bessel_zero_J1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_zero_J1_e).
#[allow(non_snake_case)]
pub fn bessel_zero_J1(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_zero_J1_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th positive zero of
/// the Bessel function $J\_\nu(x)$. The current implementation does not
/// support negative values of `nu`.
///
/// Binds the [`gsl_sf_bessel_zero_Jnu_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_bessel_zero_Jnu_e).
#[allow(non_snake_case)]
pub fn bessel_zero_Jnu(nu: f64, s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_bessel_zero_Jnu_e(nu, s, &mut result))?;
        Ok(result.into())
    }
}
