/*
    special/gamma.rs
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
use num_complex::Complex64;

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the Gamma function $\Gamma(x)$, subject to $x$
/// not being a negative integer or zero. The function is computed using the real
/// Lanczos method. The maximum value of $x$ such that $\Gamma(x)$ is not
/// considered an overflow is given by the macro `GSL_SF_GAMMA_XMAX`
/// and is 171.0.
///
/// Binds the [`gsl_sf_gamma_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_e).
pub fn gamma(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gamma_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// This routine computes $\log(\Gamma(z))$ for complex $z = z\_r + i z\_i$
/// and $z$ not a negative integer or zero, using the complex Lanczos
/// method. The returned parameters are $lnr = \log|\Gamma(z)|$ and
/// $arg = \arg(\Gamma(z))$ in $(-\pi,\pi]$. Note that the phase
/// part (`arg`) is not well-determined when $|z|$ is very large,
/// due to inevitable roundoff in restricting to $(-\pi,\pi]$. This
/// will result in a `GSL_ELOSS` error when it occurs. The absolute
/// value part (`lnr`), however, never suffers from loss of precision.
///
/// Binds the [`gsl_sf_lngamma_complex_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lngamma_complex_e).
pub fn ln_gamma_complex(z: Complex64) -> Result<ValWithError<Complex64>> {
    unsafe {
        let mut ln_r = gsl_sf_result { val: 0.0, err: 0.0 };
        let mut arg = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lngamma_complex_e(z.re, z.im, &mut ln_r, &mut arg))?;

        Ok(ValWithError {
            val: Complex64::from_polar(ln_r.val, arg.val),
            err: Complex64::from_polar(ln_r.err, arg.err),
        })
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the regulated Gamma Function $\Gamma^\*(x)$
/// for $x > 0$. The regulated gamma function is given by,
/// $$
/// \Gamma^\*(x) = \Gamma(x)/(\sqrt{2\pi} x^{(x-1/2)} \exp(-x))
///              = \left(1 + {1 \over 12x} + ...\right) \quad\hbox{for~} x\to \infty
/// $$
/// and is a useful suggestion of Temme.
///
/// Binds the [`gsl_gamma_star`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gammastar_e) function.
pub fn gamma_star(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result {val: 0.0, err: 0.0};
        GSLError::from_raw(gsl_sf_gammastar_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the reciprocal of the gamma function,
/// $1/\Gamma(x)$ using the real Lanczos method.
///
/// Binds the [`gsl_sf_gammainv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gammainv_e).
pub fn gamma_inv(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result {val: 0.0, err: 0.0};
        GSLError::from_raw(gsl_sf_gammainv_e(x, &mut result))?;
        Ok(result.into())
    }
}

pub fn gamma_complex(z: Complex64) -> Result<ValWithError<Complex64>> {
    unsafe {
        let mut ln_r = gsl_sf_result { val: 0.0, err: 0.0 };
        let mut arg = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lngamma_complex_e(z.re, z.im, &mut ln_r, &mut arg))?;

        Ok(ValWithError {
            val: Complex64::from_polar(ln_r.val.exp(), arg.val),
            err: Complex64::from_polar(ln_r.err.exp(), arg.err),
        })
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the Taylor coefficient $x^n / n!$ for
/// $x \ge 0$, $n \ge 0$
///
/// Binds the [`gsl_sf_taylorcoeff_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_taylorcoeff_e).
pub fn taylor_coefficient(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_taylorcoeff_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the factorial $n!$. The factorial is
/// related to the Gamma function by $n! = \Gamma(n+1)$.
/// The maximum value of $n$ such that $n!$ is not
/// considered an overflow is given by the macro `GSL_SF_FACT_NMAX`
/// and is 170.
///
/// Binds the [`gsl_sf_fact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fact_e).
pub fn factorial(n: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fact_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the double factorial $n!! = n(n-2)(n-4) \dots$.
/// The maximum value of $n$ such that $n!!$ is not
/// considered an overflow is given by the macro `GSL_SF_DOUBLEFACT_NMAX`
/// and is 297.
///
/// Binds the [`gsl_sf_doublefact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_doublefact_e).
pub fn double_factorial(n: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_doublefact_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the logarithm of the factorial of `n`,
/// $\log(n!)$. The algorithm is faster than computing
/// $\ln(\Gamma(n+1))$ via `gsl_sf_lngamma()` for $n < 170$,
/// but defers for larger `n`.
///
/// Binds the [`gsl_sf_lnfact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnfact_e).
pub fn ln_factorial(n: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lnfact_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the logarithm of `n choose m`. This is
/// equivalent to the sum $\log(n!) - \log(m!) - \log((n-m)!)$.
///
/// Binds the [`gsl_sf_lnchoose_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnchoose_e).
pub fn ln_choose(n: u32, m: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lnchoose_e(n, m, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the combinatorial factor `n choose m`
/// $= n!/(m!(n-m)!)$
///
/// Binds the [`gsl_sf_choose_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_choose_e).
pub fn choose(n: u32, m: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_choose_e(n, m, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// These routines compute the Hurwitz zeta function $\zeta(s,q)$ for
/// $s > 1$, $q > 0$.
///
/// Binds the [`gsl_sf_hzeta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hzeta_e).
pub fn hurwitz_zeta(s: f64, a: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hzeta_e(s, a, &mut result))?;
        Ok(result.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::special::special_function_test::*;
    
    #[test]
    fn test_gamma() {
        disable_error_handler();

        check_result(
            gamma(1.0 + 1.0/4096.0),
            0.9998591371459403421,
            TEST_TOL0
        );

        check_result(
            gamma(1.0 + 1.0/32.0),
            0.9829010992836269148,
            TEST_TOL0
        );

        check_result(
            gamma(2.0 + 1.0/256.0),
            1.0016577903733583299,
            TEST_TOL0
        );

        check_result(
            gamma(9.0),
            40320.0,
            TEST_TOL0
        );

        check_result(
            gamma(10.0),
            362880.0,
            TEST_TOL0
        );

        check_result(
            gamma(100.0),
            9.332621544394415268e+155,
            TEST_TOL2
        );

        check_result(
            gamma(170.0),
            4.269068009004705275e+304,
            TEST_TOL2
        );

        check_result(
            gamma(171.0),
            7.257415615307998967e+306,
            TEST_TOL2
        );

        check_result(
            gamma(-10.5),
            -2.640121820547716316e-07,
            TEST_TOL0
        );

        check_result(
            gamma(-1.0+1.0/65536.0),
            -65536.42280587818970,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_gamma_star() {
        disable_error_handler();

        check_result(
            gamma_star(1.0e-08),
            3989.423555759890865,
            TEST_TOL1
        );

        check_result(
            gamma_star(1.0e-05),
            126.17168469882690233,
            TEST_TOL0
        );

        check_result(
            gamma_star(0.001),
            12.708492464364073506,
            TEST_TOL0
        );

        check_result(
            gamma_star(1.5),
            1.0563442442685598666,
            TEST_TOL0
        );

        check_result(
            gamma_star(3.0),
            1.0280645179187893045,
            TEST_TOL0
        );

        check_result(
            gamma_star(9.0),
            1.0092984264218189715,
            TEST_TOL0
        );

        check_result(
            gamma_star(11.0),
            1.0076024283104962850,
            TEST_TOL0
        );

        check_result(
            gamma_star(100.0),
            1.0008336778720121418,
            TEST_TOL0
        );

        check_result(
            gamma_star(1.0e+05),
            1.0000008333336805529,
            TEST_TOL0
        );

        check_result(
            gamma_star(1.0e+20),
            1.0,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_gamma_inv() {
        disable_error_handler();

        check_result(
            gamma_inv(1.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(2.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(3.0),
            0.5,
            TEST_TOL0
        );

        check_result(
            gamma_inv(4.0),
            1.0/6.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(10.0),
            1.0/362880.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(100.0),
            1.0715102881254669232e-156,
            TEST_TOL2
        );

        check_result(
            gamma_inv(0.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(-1.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(-2.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(-3.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(-4.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gamma_inv(-10.5),
            -1.0/2.640121820547716316e-07,
            TEST_TOL2
        );

        check_result(
            gamma_inv(-11.25),
            1.0/6.027393816261931672e-08,
            TEST_TOL1
        );

        check_result(
            gamma_inv(-1.0+1.0/65536.0),
            -1.0/65536.42280587818970,
            TEST_TOL1
        );
    }
    
    #[test]
    fn test_taylor_coefficient() {
        disable_error_handler();

        check_result(
            taylor_coefficient(10, 1.0/1048576.0),
            1.7148961854776073928e-67,
            TEST_TOL0
        );

        check_result(
            taylor_coefficient(10, 1.0/1024.0),
            2.1738891788497900281e-37,
            TEST_TOL0
        );

        check_result(
            taylor_coefficient(10, 1.0),
            2.7557319223985890653e-07,
            TEST_TOL0
        );

        check_result(
            taylor_coefficient(10, 5.0),
            2.6911444554673721340,
            TEST_TOL0
        );

        check_result(
            taylor_coefficient(10, 500.0),
            2.6911444554673721340e+20,
            TEST_TOL0
        );

        check_result(
            taylor_coefficient(100, 100.0),
            1.0715102881254669232e+42,
            TEST_TOL1
        );

        check_result(
            taylor_coefficient(1000, 200.0),
            2.6628790558154746898e-267,
            TEST_TOL1
        );

        check_result(
            taylor_coefficient(1000, 500.0),
            2.3193170139740855074e+131,
            TEST_TOL1
        );
    }
    
    #[test]
    fn test_factorial() {
        disable_error_handler();

        check_result(
            factorial(0),
            1.0,
            TEST_TOL0
        );

        check_result(
            factorial(1),
            1.0,
            TEST_TOL0
        );

        check_result(
            factorial(7),
            5040.0,
            TEST_TOL0
        );

        check_result(
            factorial(33),
            8.683317618811886496e+36,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_double_factorial() {
        disable_error_handler();

        check_result(
            double_factorial(0),
            1.0,
            TEST_TOL0
        );

        check_result(
            double_factorial(1),
            1.0,
            TEST_TOL0
        );

        check_result(
            double_factorial(7),
            105.0,
            TEST_TOL0
        );

        check_result(
            double_factorial(33),
            6.332659870762850625e+18,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_ln_factorial() {
        disable_error_handler();

        check_result(
            ln_factorial(0),
            0.0,
            TEST_TOL0
        );

        check_result(
            ln_factorial(1),
            0.0,
            TEST_TOL0
        );

        check_result(
            ln_factorial(7),
            8.525161361065414300,
            TEST_TOL0
        );

        check_result(
            ln_factorial(33),
            85.05446701758151741,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_double_factorial() {
        disable_error_handler();

        check_result(
            double_factorial(0),
            0.0,
            TEST_TOL0
        );

        check_result(
            double_factorial(7),
            4.653960350157523371,
            TEST_TOL0
        );

        check_result(
            double_factorial(33),
            43.292252022541719660,
            TEST_TOL0
        );

        check_result(
            double_factorial(34),
            45.288575519655959140,
            TEST_TOL0
        );

        check_result(
            double_factorial(1034),
            3075.6383796271197707,
            TEST_TOL0
        );

        check_result(
            double_factorial(1035),
            3078.8839081731809169,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_ln_choose() {
        disable_error_handler();

        check_result(
            ln_choose(7, 3),
            3.555348061489413680,
            TEST_TOL0
        );

        check_result(
            ln_choose(5, 2),
            2.302585092994045684,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_choose() {
        disable_error_handler();

        check_result(
            choose(7, 3),
            35.0,
            TEST_TOL0
        );

        check_result(
            choose(7, 4),
            35.0,
            TEST_TOL0
        );

        check_result(
            choose(5, 2),
            10.0,
            TEST_TOL0
        );

        check_result(
            choose(5, 3),
            10.0,
            TEST_TOL0
        );

        check_result(
            choose(500, 495),
            255244687600.0,
            TEST_TOL0
        );

        check_result(
            choose(500, 5),
            255244687600.0,
            TEST_TOL0
        );

        check_result(
            choose(500, 200),
            5.054949849935532221e+144,
            TEST_TOL5
        );

        check_result(
            choose(500, 300),
            5.054949849935532221e+144,
            TEST_TOL5
        );
    }
}