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
/// Binds the [`gsl_sf_lngamma_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lngamma_e).
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
/// Binds the [`gsl_sf_gamma_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_e).
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
/// Binds the [`gsl_sf_gammastar_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gammastar_e).
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
/// Binds the [`gsl_sf_gammainv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gammainv_e).
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
/// Binds the [`gsl_sf_taylorcoeff_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_taylorcoeff_e).
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
/// Binds the [`gsl_sf_fact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_fact_e).
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
/// Binds the [`gsl_sf_doublefact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_doublefact_e).
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
/// Binds the [`gsl_sf_lnfact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnfact_e).
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
/// Binds the [`gsl_sf_lndoublefact_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lndoublefact_e).
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
/// Binds the [`gsl_sf_lnchoose_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnchoose_e).
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
/// Binds the [`gsl_sf_choose_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_choose_e).
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
/// Binds the [`gsl_sf_lnpoch_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnpoch_e).
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
/// Binds the [`gsl_sf_poch_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_poch_e).
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
/// Binds the [`gsl_sf_pochrel_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_pochrel_e).
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
/// Binds the [`gsl_sf_gamma_inc_Q_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_inc_Q_e).
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
/// Binds the [`gsl_sf_gamma_inc_P_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_inc_P_e).
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
/// Binds the [`gsl_sf_gamma_inc_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_gamma_inc_e).
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
/// Binds the [`gsl_sf_lnbeta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_lnbeta_e).
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
/// Binds the [`gsl_sf_beta_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_beta_e).
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
/// Binds the [`gsl_sf_beta_inc_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_beta_inc_e).
pub fn beta_inc(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_beta_inc_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}
