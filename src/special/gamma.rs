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


#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the Gamma function,
/// $\log(\Gamma(x))$, subject to $x$ not being a negative
/// integer or zero. For $x < 0$ the real part of $\log(\Gamma(x))$ is
/// returned, which is equivalent to $\log(|\Gamma(x)|)$. The function
/// is computed using the real Lanczos method.
///
/// Binds the [`gsl_sf_lngamma_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lngamma_e) function.
pub fn lngamma(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lngamma_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Gamma function $\Gamma(x)$, subject to $x$
/// not being a negative integer or zero. The function is computed using the real
/// Lanczos method. The maximum value of $x$ such that $\Gamma(x)$ is not
/// considered an overflow is given by the macro `GSL_SF_GAMMA_XMAX`
/// and is 171.0.
///
/// Binds the [`gsl_sf_gamma_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_e) function.
pub fn gamma(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gamma_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the regulated Gamma Function $\Gamma^\*(x)$
/// for $x > 0$. The regulated gamma function is given by,
/// $\Gamma^\*(x) &= \Gamma(x)/(\sqrt{2\pi} x^{(x-1/2)} \exp(-x))\cr
///  &= \left(1 + {1 \over 12x} + ...\right) \quad\hbox{for~} x\to \infty\cr$
/// and is a useful suggestion of Temme.
///
/// Binds the [`gsl_sf_gammastar_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gammastar_e) function.
pub fn gammastar(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gammastar_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the reciprocal of the gamma function,
/// $1/\Gamma(x)$ using the real Lanczos method.
///
/// Binds the [`gsl_sf_gammainv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gammainv_e) function.
pub fn gammainv(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gammainv_e(x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Taylor coefficient $x^n / n!$ for
/// $x \ge 0$, $n \ge 0$
///
/// Binds the [`gsl_sf_taylorcoeff_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_taylorcoeff_e) function.
pub fn taylorcoeff(n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_taylorcoeff_e(n, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the factorial $n!$. The factorial is
/// related to the Gamma function by $n! = \Gamma(n+1)$.
/// The maximum value of $n$ such that $n!$ is not
/// considered an overflow is given by the macro `GSL_SF_FACT_NMAX`
/// and is 170.
///
/// Binds the [`gsl_sf_fact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fact_e) function.
pub fn fact(n: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_fact_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the double factorial $n!! = n(n-2)(n-4) \dots$.
/// The maximum value of $n$ such that $n!!$ is not
/// considered an overflow is given by the macro `GSL_SF_DOUBLEFACT_NMAX`
/// and is 297.
///
/// Binds the [`gsl_sf_doublefact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_doublefact_e) function.
pub fn doublefact(n: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_doublefact_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the factorial of `n`,
/// $\log(n!)$. The algorithm is faster than computing
/// $\ln(\Gamma(n+1))$ via `gsl_sf_lngamma()` for $n < 170$,
/// but defers for larger `n`.
///
/// Binds the [`gsl_sf_lnfact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnfact_e) function.
pub fn lnfact(n: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lnfact_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the double factorial of `n`,
/// $\log(n!!)$.
///
/// Binds the [`gsl_sf_lndoublefact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lndoublefact_e) function.
pub fn lndoublefact(n: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lndoublefact_e(n, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of `n choose m`. This is
/// equivalent to the sum $\log(n!) - \log(m!) - \log((n-m)!)$.
///
/// Binds the [`gsl_sf_lnchoose_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnchoose_e) function.
pub fn lnchoose(n: u32, m: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lnchoose_e(n, m, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the combinatorial factor `n choose m`
/// $= n!/(m!(n-m)!)$
///
/// Binds the [`gsl_sf_choose_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_choose_e) function.
pub fn choose(n: u32, m: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_choose_e(n, m, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the Pochhammer symbol,
/// $\log((a)\_x) = \log(\Gamma(a + x)/\Gamma(a))$.
///
/// Binds the [`gsl_sf_lnpoch_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnpoch_e) function.
pub fn lnpoch(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lnpoch_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Pochhammer symbol $(a)\_x = \Gamma(a + x)/\Gamma(a)$.
/// The Pochhammer symbol is also known as the Apell symbol and
/// sometimes written as $(a,x)$. When $a$ and $a + x$
/// are negative integers or zero, the limiting value of the ratio is returned.
///
/// Binds the [`gsl_sf_poch_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_poch_e) function.
pub fn poch(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_poch_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the relative Pochhammer symbol $((a)\_x - 1)/x$
/// where $(a)\_x = \Gamma(a + x)/\Gamma(a)$.
///
/// Binds the [`gsl_sf_pochrel_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_pochrel_e) function.
pub fn pochrel(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_pochrel_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the normalized incomplete Gamma Function
/// $Q(a,x) = 1/\Gamma(a) \int\_x^\infty dt t^{(a-1)} \exp(-t)$
/// for $a > 0$, $x \ge 0$.
///
/// Binds the [`gsl_sf_gamma_inc_Q_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_inc_Q_e) function.
#[allow(non_snake_case)]
pub fn gamma_inc_Q(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gamma_inc_Q_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the complementary normalized incomplete Gamma Function
/// $P(a,x) = 1 - Q(a,x) = 1/\Gamma(a) \int\_0^x dt t^{(a-1)} \exp(-t)$
/// for $a > 0$, $x \ge 0$.
/// Note that Abramowitz & Stegun call $P(a,x)$ the incomplete gamma
/// function (section 6.5).
///
/// Binds the [`gsl_sf_gamma_inc_P_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_inc_P_e) function.
#[allow(non_snake_case)]
pub fn gamma_inc_P(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gamma_inc_P_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These functions compute the unnormalized incomplete Gamma Function
/// $\Gamma(a,x) = \int\_x^\infty dt t^{(a-1)} \exp(-t)$
/// for $a$ real and $x \ge 0$.
///
/// Binds the [`gsl_sf_gamma_inc_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_inc_e) function.
pub fn gamma_inc(a: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_gamma_inc_e(a, x, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the logarithm of the Beta Function, $\log(B(a,b))$
/// subject to $a$ and $b$ not being negative integers.
///
/// Binds the [`gsl_sf_lnbeta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnbeta_e) function.
pub fn lnbeta(a: f64, b: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_lnbeta_e(a, b, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the Beta Function, $B(a,b) = \Gamma(a)\Gamma(b)/\Gamma(a+b)$
/// subject to $a$ and $b$ not being negative integers.
///
/// Binds the [`gsl_sf_beta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_beta_e) function.
pub fn beta(a: f64, b: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_beta_e(a, b, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the normalized incomplete Beta function
/// $I\_x(a,b) = B\_x(a,b) / B(a,b)$ where
/// $B\_x(a,b) = \int\_0^x t^{a-1} (1-t)^{b-1} dt$
/// for $0 \le x \le 1$.
/// For $a > 0$, $b > 0$ the value is computed using
/// a continued fraction expansion. For all other values it is computed using
/// the relation
/// $I\_x(a,b,x) = (1/a) x^a {}\_2F\_1(a,1-b,a+1,x)/B(a,b)$
///
/// Binds the [`gsl_sf_beta_inc_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_beta_inc_e) function.
pub fn beta_inc(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_beta_inc_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::special::special_function_test::*;
    
    #[test]
    fn test_beta() {
        disable_error_handler();

        check_result(
            beta(1.0, 1.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            beta(1.0, 1.001),
            0.9990009990009990010,
            TEST_TOL0
        );

        check_result(
            beta(1.0, 5.0),
            0.2,
            TEST_TOL1
        );

        check_result(
            beta(1.0, 100.0),
            0.01,
            TEST_TOL1
        );

        check_result(
            beta(10.0, 100.0),
            2.3455339739604649879e-15,
            TEST_TOL2
        );

        check_result(
            beta(2.5, -0.1),
            -11.43621278354402041480,
            TEST_TOL2
        );

        check_result(
            beta(2.5, -1.1),
            14.555179906328753255202,
            TEST_TOL2
        );

        check_result(
            beta(-0.25, -0.1),
            -13.238937960945229110,
            TEST_TOL2
        );

        check_result(
            beta(-1.25, -0.1),
            -14.298052997820847439,
            TEST_TOL2
        );

        check_result(
            beta(-100.1, -99.1),
            -1.005181917797644630375787297e60,
            TEST_TOL3
        );

        check_result(
            beta(-100.1, 99.3),
            0.0004474258199579694011200969001,
            TEST_TOL2
        );

        check_result(
            beta(100.1, -99.3),
            1.328660939628876472028853747,
            TEST_TOL2
        );

        check_result(
            beta(-100.1, 1.2),
            0.00365530364287960795444856281,
            TEST_TOL3
        );

        check_result(
            beta(100.1, -1.2),
            1203.895236907821059270698160,
            TEST_TOL2
        );

        check_result(
            beta(-100.1, -1.2),
            -3236.073671884748847700283841,
            TEST_TOL2
        );

        check_result(
            beta(-100.001, 0.0099),
            -853.946649365611147996495177,
            TEST_TOL4
        );

        check_result(
            beta(1e-32, 1.5),
            1e32,
            TEST_TOL2
        );

        check_result(
            beta(1e-6, 0.5),
            1000001.386293677092419390336,
            TEST_TOL2
        );

        check_result(
            beta(-1.5, 0.5),
            0.0,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_beta_inc() {
        disable_error_handler();

        check_result(
            beta_inc(1.0, 1.0, 0.0),
            0.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 1.0, 1.0),
            1.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(0.1, 0.1, 1.0),
            1.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 1.0, 0.5),
            0.5,
            TEST_TOL2
        );

        check_result(
            beta_inc(0.1, 1.0, 0.5),
            0.9330329915368074160,
            TEST_TOL2
        );

        check_result(
            beta_inc(10.0, 1.0, 0.5),
            0.0009765625000000000000,
            TEST_TOL2
        );

        check_result(
            beta_inc(50.0, 1.0, 0.5),
            8.881784197001252323e-16,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 0.1, 0.5),
            0.06696700846319258402,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 10.0, 0.5),
            0.99902343750000000000,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 50.0, 0.5),
            0.99999999999999911180,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 1.0, 0.1),
            0.10,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 2.0, 0.1),
            0.19,
            TEST_TOL2
        );

        check_result(
            beta_inc(1.0, 2.0, 0.9),
            0.99,
            TEST_TOL2
        );

        check_result(
            beta_inc(50.0, 60.0, 0.5),
            0.8309072939016694143,
            TEST_TOL2
        );

        check_result(
            beta_inc(90.0, 90.0, 0.5),
            0.5,
            TEST_TOL2
        );

        check_result(
            beta_inc(500.0, 500.0, 0.6),
            0.9999999999157549630,
            TEST_TOL2
        );

        check_result(
            beta_inc(5000.0, 5000.0, 0.4),
            4.518543727260666383e-91,
            TEST_TOL5
        );

        check_result(
            beta_inc(5000.0, 5000.0, 0.6),
            1.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(5000.0, 2000.0, 0.6),
            8.445388773903332659e-89,
            TEST_TOL5
        );

        check_result(
            beta_inc(-0.1, -0.1, 1.0),
            1.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.1, -0.2, 1.0),
            1.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.2, -0.1, 1.0),
            1.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.1, -0.2, 0.5),
            0.675252001958389971991335,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.2, -0.1, 0.5),
            0.324747998041610028008665,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.1, -0.1, 0.0),
            0.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.1, -0.2, 0.0),
            0.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.2, -0.1, 0.0),
            0.0,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.1, -0.2, 0.3),
            0.7469186777964287252,
            TEST_TOL2
        );

        check_result(
            beta_inc(-0.2, -0.1, 0.3),
            0.3995299653262016818,
            TEST_TOL2
        );

        check_result(
            beta_inc(0.5, 101.5, 0.999457),
            1.0,
            TEST_TOL2
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
    
    #[test]
    fn test_doublefact() {
        disable_error_handler();

        check_result(
            doublefact(0),
            1.0,
            TEST_TOL0
        );

        check_result(
            doublefact(1),
            1.0,
            TEST_TOL0
        );

        check_result(
            doublefact(7),
            105.0,
            TEST_TOL0
        );

        check_result(
            doublefact(33),
            6.332659870762850625e+18,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_fact() {
        disable_error_handler();

        check_result(
            fact(0),
            1.0,
            TEST_TOL0
        );

        check_result(
            fact(1),
            1.0,
            TEST_TOL0
        );

        check_result(
            fact(7),
            5040.0,
            TEST_TOL0
        );

        check_result(
            fact(33),
            8.683317618811886496e+36,
            TEST_TOL0
        );
    }
    
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
    fn test_gamma_inc() {
        disable_error_handler();

        check_result(
            gamma_inc(-1.0/1048576.0, 1.0/1048576.0),
            13.285819596290624271,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-0.001, 1.0/1048576.0),
            13.381275128625328858,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-1.0, 1.0/1048576.0),
            1.0485617142715768655e+06,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-0.00001, 0.001),
            6.3317681434563592142,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-0.0001, 0.001),
            6.3338276439767189385,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-0.001, 0.001),
            6.3544709102510843793,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-0.5, 0.001),
            59.763880515942196981,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-1.0, 0.001),
            992.66896046923884234,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-3.5, 0.001),
            9.0224404490639003706e+09,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-10.5, 0.001),
            3.0083661558184815656e+30,
            TEST_TOL2
        );

        check_result(
            gamma_inc(-0.001, 0.1),
            1.8249109609418620068,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-0.5, 0.1),
            3.4017693366916154163,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-10.0, 0.1),
            8.9490757483586989181e+08,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-10.5, 0.1),
            2.6967403834226421766e+09,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-0.001, 1.0),
            0.21928612679072766340,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-0.5, 1.0),
            0.17814771178156069019,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-1.0, 1.0),
            0.14849550677592204792,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-2.5, 1.0),
            0.096556648631275160264,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-1.0, 10.0),
            3.8302404656316087616e-07,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-0.001, 10.0),
            4.1470562324807320961e-06,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-0.5, 10.0),
            1.2609042613241570681e-06,
            TEST_TOL0
        );

        check_result(
            gamma_inc(-1.0, 10.0),
            3.8302404656316087616e-07,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-10.5, 10.0),
            6.8404927328441566785e-17,
            TEST_TOL1
        );

        check_result(
            gamma_inc(-100.0, 10.0),
            4.1238327669858313997e-107,
            TEST_TOL2
        );

        check_result(
            gamma_inc(-200.0, 10.0),
            2.1614091830529343423e-207,
            TEST_TOL2
        );

        check_result(
            gamma_inc(0.0, 0.001),
            6.3315393641361493320,
            TEST_TOL0
        );

        check_result(
            gamma_inc(0.001, 0.001),
            6.3087159394864007261,
            TEST_TOL0
        );

        check_result(
            gamma_inc(1.0, 0.001),
            0.99900049983337499167,
            TEST_TOL0
        );

        check_result(
            gamma_inc(10.0, 0.001),
            362880.0,
            TEST_TOL0
        );

        check_result(
            gamma_inc(0.0, 1.0),
            0.21938393439552027368,
            TEST_TOL0
        );

        check_result(
            gamma_inc(0.001, 1.0),
            0.21948181320730279613,
            TEST_TOL1
        );

        check_result(
            gamma_inc(1.0, 1.0),
            0.36787944117144232160,
            TEST_TOL0
        );

        check_result(
            gamma_inc(10.0, 1.0),
            362879.95956592242045,
            TEST_TOL0
        );

        check_result(
            gamma_inc(100.0, 1.0),
            9.3326215443944152682e+155,
            TEST_TOL0
        );

        check_result(
            gamma_inc(0.0, 100.0),
            3.6835977616820321802e-46,
            TEST_TOL2
        );

        check_result(
            gamma_inc(0.001, 100.0),
            3.7006367674063550631e-46,
            TEST_TOL2
        );

        check_result(
            gamma_inc(1.0, 100.0),
            3.7200759760208359630e-44,
            TEST_TOL2
        );

        check_result(
            gamma_inc(10.0, 100.0),
            4.0836606309106112723e-26,
            TEST_TOL2
        );

        check_result(
            gamma_inc(100.0, 100.0),
            4.5421981208626694294e+155,
            TEST_TOL1
        );
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_gamma_inc_P() {
        disable_error_handler();

        check_result(
            gamma_inc_P(1e-100, 0.001),
            1.0,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(0.001, 0.001),
            0.9936876467088602902,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(0.001, 1.0),
            0.9997803916424144436,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(0.001, 10.0),
            0.9999999958306921828,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(1.0, 0.001),
            0.0009995001666250083319,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(1.0, 1.01),
            0.6357810204284766802,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(1.0, 10.0),
            0.9999546000702375151,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(10.0, 10.01),
            0.5433207586693410570,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(10.0, 20.0),
            0.9950045876916924128,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(1000.0, 1000.1),
            0.5054666401440661753,
            TEST_TOL2
        );

        check_result(
            gamma_inc_P(1000.0, 2000.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            gamma_inc_P(34.0, 32.0),
            0.3849626436463866776322932129,
            TEST_TOL2
        );

        check_result(
            gamma_inc_P(37.0, 3.499999999999999289e+01),
            0.3898035054195570860969333039,
            TEST_TOL2
        );

        check_result(
            gamma_inc_P(10.0, 1e-16),
            2.755731922398588814734648067e-167,
            TEST_TOL2
        );

        check_result(
            gamma_inc_P(1263131.0, 1261282.3637),
            0.04994777516935182963821362168,
            TEST_TOL4
        );

        check_result(
            gamma_inc_P(1263131.0, 1263131.0),
            0.500118321758657770672882362502514254,
            TEST_TOL4
        );

        check_result(
            gamma_inc_P(100.0, 99.0),
            0.4733043303994607,
            TEST_TOL2
        );

        check_result(
            gamma_inc_P(200.0, 199.0),
            0.4811585880878718,
            TEST_TOL2
        );

        check_result(
            gamma_inc_P(5670.0, 4574.0),
            3.063972328743934e-55,
            TEST_TOL2
        );
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_gamma_inc_Q() {
        disable_error_handler();

        check_result(
            gamma_inc_Q(0.0, 0.001),
            0.0,
            TEST_TOL0
        );

        check_result(
            gamma_inc_Q(0.001, 0.001),
            0.006312353291139709793,
            TEST_TOL0
        );

        check_result(
            gamma_inc_Q(0.001, 1.0),
            0.00021960835758555639171,
            TEST_TOL1
        );

        check_result(
            gamma_inc_Q(0.001, 2.0),
            0.00004897691783098147880,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(0.001, 5.0),
            1.1509813397308608541e-06,
            TEST_TOL1
        );

        check_result(
            gamma_inc_Q(1.0, 0.001),
            0.9990004998333749917,
            TEST_TOL0
        );

        check_result(
            gamma_inc_Q(1.0, 1.01),
            0.3642189795715233198,
            TEST_TOL0
        );

        check_result(
            gamma_inc_Q(1.0, 10.0),
            0.00004539992976248485154,
            TEST_TOL0
        );

        check_result(
            gamma_inc_Q(10.0, 10.01),
            0.4566792413306589430,
            TEST_TOL0
        );

        check_result(
            gamma_inc_Q(10.0, 100.0),
            1.1253473960842733885e-31,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(1000.0, 1000.1),
            0.4945333598559338247,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(1000.0, 2000.0),
            6.847349459614753180e-136,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(100.0, 99.0),
            0.5266956696005394,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(200.0, 199.0),
            0.5188414119121281,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(5670.0, 4574.0),
            1.0000000000000000,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(1.0e+06-1.0, 1.0e+06-2.0),
            0.50026596175224547004,
            TEST_TOL3
        );

        check_result(
            gamma_inc_Q(1.0e+06+2.0, 1.0e+06+1.0),
            0.50026596135330304336,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(1.0e+06, 1.0e+06-2.0),
            0.50066490399940144811,
            TEST_TOL2
        );

        check_result(
            gamma_inc_Q(1.0e+07, 1.0e+07-2.0),
            0.50021026104978614908,
            TEST_TOL2
        );
    }
    
    #[test]
    fn test_gammainv() {
        disable_error_handler();

        check_result(
            gammainv(1.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            gammainv(2.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            gammainv(3.0),
            0.5,
            TEST_TOL0
        );

        check_result(
            gammainv(4.0),
            1.0/6.0,
            TEST_TOL0
        );

        check_result(
            gammainv(10.0),
            1.0/362880.0,
            TEST_TOL0
        );

        check_result(
            gammainv(100.0),
            1.0715102881254669232e-156,
            TEST_TOL2
        );

        check_result(
            gammainv(0.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gammainv(-1.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gammainv(-2.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gammainv(-3.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gammainv(-4.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            gammainv(-10.5),
            -1.0/2.640121820547716316e-07,
            TEST_TOL2
        );

        check_result(
            gammainv(-11.25),
            1.0/6.027393816261931672e-08,
            TEST_TOL1
        );

        check_result(
            gammainv(-1.0+1.0/65536.0),
            -1.0/65536.42280587818970,
            TEST_TOL1
        );
    }
    
    #[test]
    fn test_gammastar() {
        disable_error_handler();

        check_result(
            gammastar(1.0e-08),
            3989.423555759890865,
            TEST_TOL1
        );

        check_result(
            gammastar(1.0e-05),
            126.17168469882690233,
            TEST_TOL0
        );

        check_result(
            gammastar(0.001),
            12.708492464364073506,
            TEST_TOL0
        );

        check_result(
            gammastar(1.5),
            1.0563442442685598666,
            TEST_TOL0
        );

        check_result(
            gammastar(3.0),
            1.0280645179187893045,
            TEST_TOL0
        );

        check_result(
            gammastar(9.0),
            1.0092984264218189715,
            TEST_TOL0
        );

        check_result(
            gammastar(11.0),
            1.0076024283104962850,
            TEST_TOL0
        );

        check_result(
            gammastar(100.0),
            1.0008336778720121418,
            TEST_TOL0
        );

        check_result(
            gammastar(1.0e+05),
            1.0000008333336805529,
            TEST_TOL0
        );

        check_result(
            gammastar(1.0e+20),
            1.0,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_lnbeta() {
        disable_error_handler();

        check_result(
            lnbeta(1.0e-8, 1.0e-8),
            19.113827924512310617,
            TEST_TOL0
        );

        check_result(
            lnbeta(1.0e-8, 0.01),
            18.420681743788563403,
            TEST_TOL0
        );

        check_result(
            lnbeta(1.0e-8, 1.0),
            18.420680743952365472,
            TEST_TOL0
        );

        check_result(
            lnbeta(1.0e-8, 10.0),
            18.420680715662683009,
            TEST_TOL0
        );

        check_result(
            lnbeta(1.0e-8, 1000.0),
            18.420680669107656949,
            TEST_TOL0
        );

        check_result(
            lnbeta(0.1, 0.1),
            2.9813614810376273949,
            TEST_TOL1
        );

        check_result(
            lnbeta(0.1, 1.0),
            2.3025850929940456840,
            TEST_TOL1
        );

        check_result(
            lnbeta(0.1, 100.0),
            1.7926462324527931217,
            TEST_TOL0
        );

        check_result(
            lnbeta(0.1, 1000.0),
            1.5619821298353164928,
            TEST_TOL0
        );

        check_result(
            lnbeta(1.0, 1.00025),
            -0.0002499687552073570,
            TEST_TOL4
        );

        check_result(
            lnbeta(1.0, 1.01),
            -0.009950330853168082848,
            TEST_TOL3
        );

        check_result(
            lnbeta(1.0, 1000.0),
            -6.907755278982137052,
            TEST_TOL0
        );

        check_result(
            lnbeta(100.0, 100.0),
            -139.66525908670663927,
            TEST_TOL2
        );

        check_result(
            lnbeta(100.0, 1000.0),
            -336.4348576477366051,
            TEST_TOL0
        );

        check_result(
            lnbeta(100.0, 1.0e+8),
            -1482.9339185256447309,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_lnchoose() {
        disable_error_handler();

        check_result(
            lnchoose(7, 3),
            3.555348061489413680,
            TEST_TOL0
        );

        check_result(
            lnchoose(5, 2),
            2.302585092994045684,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_lndoublefact() {
        disable_error_handler();

        check_result(
            lndoublefact(0),
            0.0,
            TEST_TOL0
        );

        check_result(
            lndoublefact(7),
            4.653960350157523371,
            TEST_TOL0
        );

        check_result(
            lndoublefact(33),
            43.292252022541719660,
            TEST_TOL0
        );

        check_result(
            lndoublefact(34),
            45.288575519655959140,
            TEST_TOL0
        );

        check_result(
            lndoublefact(1034),
            3075.6383796271197707,
            TEST_TOL0
        );

        check_result(
            lndoublefact(1035),
            3078.8839081731809169,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_lnfact() {
        disable_error_handler();

        check_result(
            lnfact(0),
            0.0,
            TEST_TOL0
        );

        check_result(
            lnfact(1),
            0.0,
            TEST_TOL0
        );

        check_result(
            lnfact(7),
            8.525161361065414300,
            TEST_TOL0
        );

        check_result(
            lnfact(33),
            85.05446701758151741,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_lngamma() {
        disable_error_handler();

        check_result(
            lngamma(-0.1),
            2.368961332728788655,
            TEST_TOL0
        );

        check_result(
            lngamma(-1.0/256.0),
            5.547444766967471595,
            TEST_TOL0
        );

        check_result(
            lngamma(1.0e-08),
            18.420680738180208905,
            TEST_TOL0
        );

        check_result(
            lngamma(0.1),
            2.252712651734205,
            TEST_TOL0
        );

        check_result(
            lngamma(1.0 + 1.0/256.0),
            -0.0022422226599611501448,
            TEST_TOL0
        );

        check_result(
            lngamma(2.0 + 1.0/256.0),
            0.0016564177556961728692,
            TEST_TOL0
        );

        check_result(
            lngamma(100.0),
            359.1342053695753,
            TEST_TOL0
        );

        check_result(
            lngamma(-1.0-1.0/65536.0),
            11.090348438090047844,
            TEST_TOL0
        );

        check_result(
            lngamma(-1.0-1.0/268435456.0),
            19.408121054103474300,
            TEST_TOL0
        );

        check_result(
            lngamma(-100.5),
            -364.9009683094273518,
            TEST_TOL0
        );

        check_result(
            lngamma(-100.0-1.0/65536.0),
            -352.6490910117097874,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_lnpoch() {
        disable_error_handler();

        check_result(
            lnpoch(5.0, 0.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            lnpoch(5.0, 1.0/65536.0),
            0.000022981557571259389129,
            TEST_TOL0
        );

        check_result(
            lnpoch(5.0, 1.0/256.0),
            0.005884960217985189004,
            TEST_TOL2
        );

        check_result(
            lnpoch(7.0, 3.0),
            6.222576268071368616,
            TEST_TOL0
        );

        check_result(
            lnpoch(5.0, 2.0),
            3.401197381662155375,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_poch() {
        disable_error_handler();

        check_result(
            poch(5.0, 0.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            poch(7.0, 3.0),
            504.0,
            TEST_TOL0
        );

        check_result(
            poch(5.0, 2.0),
            30.0,
            TEST_TOL1
        );

        check_result(
            poch(5.0, 1.0/256.0),
            1.0059023106151364982,
            TEST_TOL0
        );

        check_result(
            poch(-9.0, -4.0),
            1.0/17160.0,
            TEST_TOL0
        );

        check_result(
            poch(-9.0, -3.0),
            -1.0/1320.0,
            TEST_TOL0
        );

        check_result(
            poch(-9.0, -3.5),
            0.0,
            TEST_TOL0
        );

        check_result(
            poch(-9.0, 4.0),
            3024.0,
            TEST_TOL0
        );

        check_result(
            poch(-9.0, 3.0),
            -504.0,
            TEST_TOL0
        );

        check_result(
            poch(-9.0, 3.5),
            0.0,
            TEST_TOL0
        );

        check_result(
            poch(-9.0, 0.0),
            1.0,
            TEST_TOL0
        );

        check_result(
            poch(-8.0, -4.0),
            1.0/11880.0,
            TEST_TOL0
        );

        check_result(
            poch(-8.0, -3.0),
            -1.0/990.0,
            TEST_TOL0
        );

        check_result(
            poch(-8.0, 4.0),
            1680.0,
            TEST_TOL0
        );

        check_result(
            poch(-8.0, 3.0),
            -336.0,
            TEST_TOL0
        );

        check_result(
            poch(-3.0, 4.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            poch(-3.0, 3.0),
            -6.0,
            TEST_TOL2
        );

        check_result(
            poch(-4.0, 4.0),
            24.0,
            TEST_TOL2
        );

        check_result(
            poch(-3.0, 100.0),
            0.0,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_pochrel() {
        disable_error_handler();

        check_result(
            pochrel(5.0, 0.0),
            1.506117668431800472,
            TEST_TOL1
        );

        check_result(
            pochrel(7.0, 3.0),
            503.0/3.0,
            TEST_TOL0
        );

        check_result(
            pochrel(5.0, 2.0),
            29.0/2.0,
            TEST_TOL1
        );

        check_result(
            pochrel(5.0, 0.01),
            1.5186393661368275330,
            TEST_TOL2
        );

        check_result(
            pochrel(-5.5, 0.01),
            1.8584945633829063516,
            TEST_TOL1
        );

        check_result(
            pochrel(-5.5, -1.0/8.0),
            1.0883319303552135488,
            TEST_TOL1
        );

        check_result(
            pochrel(-5.5, -1.0/256.0),
            1.7678268037726177453,
            TEST_TOL1
        );

        check_result(
            pochrel(-5.5, -11.0),
            0.09090909090939652475,
            TEST_TOL0
        );
    }
    
    #[test]
    fn test_taylorcoeff() {
        disable_error_handler();

        check_result(
            taylorcoeff(10, 1.0/1048576.0),
            1.7148961854776073928e-67,
            TEST_TOL0
        );

        check_result(
            taylorcoeff(10, 1.0/1024.0),
            2.1738891788497900281e-37,
            TEST_TOL0
        );

        check_result(
            taylorcoeff(10, 1.0),
            2.7557319223985890653e-07,
            TEST_TOL0
        );

        check_result(
            taylorcoeff(10, 5.0),
            2.6911444554673721340,
            TEST_TOL0
        );

        check_result(
            taylorcoeff(10, 500.0),
            2.6911444554673721340e+20,
            TEST_TOL0
        );

        check_result(
            taylorcoeff(100, 100.0),
            1.0715102881254669232e+42,
            TEST_TOL1
        );

        check_result(
            taylorcoeff(1000, 200.0),
            2.6628790558154746898e-267,
            TEST_TOL1
        );

        check_result(
            taylorcoeff(1000, 500.0),
            2.3193170139740855074e+131,
            TEST_TOL1
        );
    }
}
