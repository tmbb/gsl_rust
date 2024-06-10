/*
    special/trig.rs
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

/// This routine computes the sine of an angle `x` with an associated
/// absolute error `dx`,
/// $\sin(x \pm dx)$.
/// Note that this function is provided in the error-handling form only since
/// its purpose is to compute the propagated error.
/// 
/// Binds the function [`gsl_sf_sin_err_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_sin_err_e).
pub fn sin_err_e(x: f64, dx: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_sin_err_e(x, dx, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute $\text{sinc}(x) = \sin(\pi x) / (\pi x)$ for any
/// value of `x`.
/// 
/// Binds the function [`gsl_sf_sinc_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_sinc_e).
pub fn sinc_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_sinc_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute $\log(\sinh(x))$ for $x > 0$.
/// 
/// Binds the function [`gsl_sf_lnsinh_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lnsinh_e).
pub fn ln_sinh_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_lnsinh_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the sine function $\sin(x)$.
/// 
/// Binds the function [`gsl_sf_sin_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_sin_e).
pub fn sin_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_sin_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the cosine function $\cos(x)$.
/// 
/// Binds the function [`gsl_sf_cos_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_cos_e).
pub fn cos_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_cos_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// This routine computes the cosine of an angle `x` with an associated
/// absolute error `dx`,
/// $\cos(x \pm dx)$.
/// Note that this function is provided in the error-handling form only since
/// its purpose is to compute the propagated error.
/// 
/// Binds the function [`gsl_sf_cos_err_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_cos_err_e).
pub fn cos_err_e(x: f64, dx: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_cos_err_e(x, dx, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the hypotenuse function $\sqrt{x^2 + y^2}$
/// avoiding overflow and underflow.
/// 
/// Binds the function [`gsl_sf_hypot_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hypot_e).
pub fn hypot_e(x: f64, y: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hypot_e(x, y, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute $\log(\cosh(x))$ for any `x`.
/// 
/// Binds the function [`gsl_sf_lncosh_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lncosh_e).
pub fn ln_cosh_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_lncosh_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines force the angle `theta` to lie in the range $[0, 2\pi)$.
/// 
/// Note that the mathematical value of $2\pi$ is slightly greater
/// than `2*M_PI`, so the machine number `2*M_PI` is included in
/// the range.
/// 
/// Binds the function [`gsl_sf_angle_restrict_pos`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_angle_restrict_pos).
pub fn angle_restrict_pos(theta: f64) -> f64 {
    unsafe { bindings::gsl_sf_angle_restrict_pos(theta) }
}

/// These routines compute the sine function $\sin(x)$.
/// 
/// Binds the function [`gsl_sf_sin`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_sin).
pub fn sin(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_sin(x) }
}

/// These routines compute the cosine function $\cos(x)$.
/// 
/// Binds the function [`gsl_sf_cos`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_cos).
pub fn cos(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_cos(x) }
}

/// These routines compute $\log(\cosh(x))$ for any `x`.
/// 
/// Binds the function [`gsl_sf_lncosh`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lncosh).
pub fn ln_cosh(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_lncosh(x) }
}

/// These routines compute $\text{sinc}(x) = \sin(\pi x) / (\pi x)$ for any
/// value of `x`.
/// 
/// Binds the function [`gsl_sf_sinc`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_sinc).
pub fn sinc(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_sinc(x) }
}

/// These routines force the angle `theta` to lie in the range
/// $(-\pi,\pi]$.
/// 
/// Note that the mathematical value of $\pi$ is slightly greater
/// than `M_PI`, so the machine numbers `M_PI` and `-M_PI`
/// are included in the range.
/// 
/// Binds the function [`gsl_sf_angle_restrict_symm`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_angle_restrict_symm).
pub fn angle_restrict_symm(theta: f64) -> f64 {
    unsafe { bindings::gsl_sf_angle_restrict_symm(theta) }
}

/// These routines compute $\log(\sinh(x))$ for $x > 0$.
/// 
/// Binds the function [`gsl_sf_lnsinh`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_lnsinh).
pub fn ln_sinh(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_lnsinh(x) }
}

/// These routines compute the hypotenuse function $\sqrt{x^2 + y^2}$
/// avoiding overflow and underflow.
/// 
/// Binds the function [`gsl_sf_hypot`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hypot).
pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { bindings::gsl_sf_hypot(x, y) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_cos_e() {
        check_result!(cos_e(-10.0), -0.8390715290764524523, TEST_TOL0);
        check_result!(cos_e(1.0), 0.5403023058681397174, TEST_TOL0);
        check_result!(cos_e(1000.0), 0.5623790762907029911, TEST_TOL1);
        check_result!(cos_e(1048576.75), 0.4652971620066351799, TEST_TOL2);
        check_result!(cos_e(62831853.75), 0.7787006914966116436, TEST_TOL2);
        check_result!(cos_e(1073741822.5), -0.5601305436977716102, TEST_SQRT_TOL0);
        check_result!(cos_e(1073741824.0), 0.7867071229411881196, TEST_SQRT_TOL0);
        check_result!(cos_e(1099511627776.0), -0.9140040719915570023, 128.0*TEST_SQRT_TOL0);
    }

    #[test]
    fn test_ln_cosh_e() {
        check_result!(ln_cosh_e(0.125), 0.007792239318898252791, TEST_TOL0);
        check_result!(ln_cosh_e(1.0), 0.4337808304830271870, TEST_TOL0);
        check_result!(ln_cosh_e(5.0), 4.306898218339271555, TEST_TOL0);
        check_result!(ln_cosh_e(100.0), 99.30685281944005469, TEST_TOL0);
        check_result!(ln_cosh_e(1000.0), 999.30685281944005469, TEST_TOL0);
        check_result!(ln_cosh_e(-0.125), 0.007792239318898252791, TEST_TOL0);
        check_result!(ln_cosh_e(-1.0), 0.4337808304830271870, TEST_TOL0);
        check_result!(ln_cosh_e(-5.0), 4.306898218339271555, TEST_TOL0);
        check_result!(ln_cosh_e(-100.0), 99.30685281944005469, TEST_TOL0);
        check_result!(ln_cosh_e(-1000.0), 999.30685281944005469, TEST_TOL0);
    }

    #[test]
    fn test_ln_sinh_e() {
        check_result!(ln_sinh_e(0.1), -2.3009189815304652235, TEST_TOL0);
        check_result!(ln_sinh_e(1.0), 0.16143936157119563361, TEST_TOL0);
        check_result!(ln_sinh_e(5.0), 4.306807418479684201, TEST_TOL0);
        check_result!(ln_sinh_e(100.0), 99.30685281944005469, TEST_TOL0);
    }

    #[test]
    fn test_sin_e() {
        check_result!(sin_e(-10.0), 0.5440211108893698134, TEST_TOL0);
        check_result!(sin_e(1.0), 0.8414709848078965067, TEST_TOL0);
        check_result!(sin_e(1000.0), 0.8268795405320025603, TEST_TOL0);
        check_result!(sin_e(1048576.75), 0.8851545351115651914, TEST_TOL1);
        check_result!(sin_e(62831853.75), 0.6273955953485000827, TEST_TOL3);
        check_result!(sin_e(1073741822.5), -0.8284043541754465988, TEST_SQRT_TOL0);
        check_result!(sin_e(1073741824.0), -0.6173264150460421708, TEST_SQRT_TOL0);
        check_result!(sin_e(1073741825.5), 0.7410684679436226926, TEST_SQRT_TOL0);
        check_result!(sin_e(1099511627776.0), -0.4057050115328287198, 32.0*TEST_SQRT_TOL0);
    }

    #[test]
    fn test_sinc_e() {
        check_result!(sinc_e(1.0/1024.0), 0.9999984312693665404, TEST_TOL0);
        check_result!(sinc_e(1.0/2.0), 2.0/M_PI, TEST_TOL0);
        check_result!(sinc_e(80.5), 0.0039541600768172754, TEST_TOL0);
        check_result!(sinc_e(100.5), 0.0031672625490924445, TEST_TOL0);
        check_result!(sinc_e(1.0e+06 + 0.5), 3.18309727028927157e-07, TEST_TOL0);
    }
}