/*
    special/hyperg.rs
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

/// These routines compute the confluent hypergeometric function
/// 
/// $${}_1F_1(m,n,x) = M(m,n,x)$$
/// 
/// for integer parameters `m`, `n`.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_1F1_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_1F1_int_e).
pub fn hyperg_1F1_int_e(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_1F1_int_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the renormalized Gauss hypergeometric function
/// 
/// $${}_2F_1(a_R + i a_I, a_R - i a_I, c, x) / \Gamma(c)$$
/// 
/// for $|x| < 1$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1_conj_renorm_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1_conj_renorm_e).
pub fn hyperg_2F1_conj_renorm_e(ar: f64, ai: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_2F1_conj_renorm_e(ar, ai, c, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Gauss hypergeometric function
/// 
/// $${}_2F_1(a,b,c,x) = F(a,b,c,x)$$
/// 
/// for $|x| < 1$.
/// 
/// If the arguments $(a,b,c,x)$ are too close to a singularity then the function
/// can return the error code `GSL_EMAXITER` when the series approximation converges
/// too slowly. This occurs in the region of $x = 1$, $c - a - b = m$ for integer m.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1_e).
pub fn hyperg_2F1_e(a: f64, b: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_2F1_e(a, b, c, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the hypergeometric function
/// 
/// $${}_0F_1(c,x)$$
/// 
/// Binds the function
/// [`gsl_sf_hyperg_0F1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_0F1_e).
pub fn hyperg_0F1_e(c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_0F1_e(c, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the confluent hypergeometric function $U(a,b,x)$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_U_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_U_e).
pub fn hyperg_U_e(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_U_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the confluent hypergeometric function $U(m,n,x)$ for
/// integer parameters `m`, `n`.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_U_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_U_int_e).
pub fn hyperg_U_int_e(m: i32, n: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_U_int_e(m, n, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the hypergeometric function
/// 
/// $${}_2F_0(a,b,x)$$
/// 
/// The series representation is a divergent hypergeometric series.
/// 
/// However, for $x < 0$ we have
/// 
/// $${}_2F_0(a,b,x) = (-1/x)^a U(a,1+a-b,-1/x)$$
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F0_e).
pub fn hyperg_2F0_e(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_2F0_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the Gauss hypergeometric function
/// 
/// $${}_2F_1(a_R + i a_I, aR - i aI, c, x)$$
/// 
/// with complex parameters for $|x| < 1$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1_conj_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1_conj_e).
pub fn hyperg_2F1_conj_e(ar: f64, ai: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_2F1_conj_e(ar, ai, c, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the confluent hypergeometric function
/// 
/// $${}_1F_1(a,b,x) = M(a,b,x)$$
/// 
/// for general parameters `a`, `b`.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_1F1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_1F1_e).
pub fn hyperg_1F1_e(a: f64, b: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_1F1_e(a, b, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the renormalized Gauss hypergeometric function
/// 
/// $${}_2F_1(a,b,c,x) / \Gamma(c)$$
/// 
/// for $|x| < 1$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1_renorm_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1_renorm_e).
pub fn hyperg_2F1_renorm_e(a: f64, b: f64, c: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_hyperg_2F1_renorm_e(a, b, c, x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the confluent hypergeometric function $U(m,n,x)$ for
/// integer parameters `m`, `n`.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_U_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_U_int).
pub fn hyperg_U_int(m: i32, n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_U_int(m, n, x) }
}

/// These routines compute the confluent hypergeometric function
/// 
/// $${}_1F_1(a,b,x) = M(a,b,x)$$
/// 
/// for general parameters `a`, `b`.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_1F1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_1F1).
pub fn hyperg_1F1(a: f64, b: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_1F1(a, b, x) }
}

/// These routines compute the hypergeometric function
/// 
/// $${}_0F_1(c,x)$$
/// 
/// Binds the function
/// [`gsl_sf_hyperg_0F1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_0F1).
pub fn hyperg_0F1(c: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_0F1(c, x) }
}

/// These routines compute the renormalized Gauss hypergeometric function
/// 
/// $${}_2F_1(a_R + i a_I, a_R - i a_I, c, x) / \Gamma(c)$$
/// 
/// for $|x| < 1$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1_conj_renorm`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1_conj_renorm).
pub fn hyperg_2F1_conj_renorm(ar: f64, ai: f64, c: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_2F1_conj_renorm(ar, ai, c, x) }
}

/// These routines compute the Gauss hypergeometric function
/// 
/// $${}_2F_1(a_R + i a_I, aR - i aI, c, x)$$
/// 
/// with complex parameters for $|x| < 1$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1_conj`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1_conj).
pub fn hyperg_2F1_conj(ar: f64, ai: f64, c: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_2F1_conj(ar, ai, c, x) }
}

/// These routines compute the Gauss hypergeometric function
/// 
/// $${}_2F_1(a,b,c,x) = F(a,b,c,x)$$
/// 
/// for $|x| < 1$.
/// 
/// If the arguments $(a,b,c,x)$ are too close to a singularity then the function
/// can return the error code `GSL_EMAXITER` when the series approximation converges
/// too slowly. This occurs in the region of $x = 1$, $c - a - b = m$ for integer m.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1).
pub fn hyperg_2F1(a: f64, b: f64, c: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_2F1(a, b, c, x) }
}

/// These routines compute the confluent hypergeometric function $U(a,b,x)$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_U`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_U).
pub fn hyperg_U(a: f64, b: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_U(a, b, x) }
}

/// These routines compute the hypergeometric function
/// 
/// $${}_2F_0(a,b,x)$$
/// 
/// The series representation is a divergent hypergeometric series.
/// 
/// However, for $x < 0$ we have
/// 
/// $${}_2F_0(a,b,x) = (-1/x)^a U(a,1+a-b,-1/x)$$
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F0).
pub fn hyperg_2F0(a: f64, b: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_2F0(a, b, x) }
}

/// These routines compute the confluent hypergeometric function
/// 
/// $${}_1F_1(m,n,x) = M(m,n,x)$$
/// 
/// for integer parameters `m`, `n`.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_1F1_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_1F1_int).
pub fn hyperg_1F1_int(m: i32, n: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_1F1_int(m, n, x) }
}

/// These routines compute the renormalized Gauss hypergeometric function
/// 
/// $${}_2F_1(a,b,c,x) / \Gamma(c)$$
/// 
/// for $|x| < 1$.
/// 
/// Binds the function
/// [`gsl_sf_hyperg_2F1_renorm`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_hyperg_2F1_renorm).
pub fn hyperg_2F1_renorm(a: f64, b: f64, c: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_hyperg_2F1_renorm(a, b, c, x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_hyperg_0F1_e() {
        check_result!(hyperg_0F1_e(1.0, 0.5), 1.5660829297563505373, TEST_TOL0);
        check_result!(hyperg_0F1_e(5.0, 0.5), 1.1042674404828684574, TEST_TOL1);
        check_result!(hyperg_0F1_e(100.0, 30.0), 1.3492598639485110176, TEST_TOL2);
        check_result!(hyperg_0F1_e(-0.5, 3.0), -39.29137997543434276, TEST_TOL1);
        check_result!(hyperg_0F1_e(-100.5, 50.0), 0.6087930289227538496, TEST_TOL3);
        check_result!(hyperg_0F1_e(1.0, -5.0), -0.3268752818235339109, TEST_TOL0);
        check_result!(hyperg_0F1_e(-0.5, -5.0), -4.581634759005381184, TEST_TOL1);
    }

    #[test]
    fn test_hyperg_1F1_e() {
        check_result!(hyperg_1F1_e(1.0, 1.5, 1.0), 2.0300784692787049755, TEST_TOL0);
        check_result!(hyperg_1F1_e(1.0, 1.5, 10.0), 6172.859561078406855, TEST_TOL0);
        check_result!(hyperg_1F1_e(1.0, 1.5, 100.0), 2.3822817898485692114e+42, TEST_TOL1);
        check_result!(hyperg_1F1_e(1.0, 1.5, 500.0), 5.562895351723513581e+215, TEST_TOL2);
        check_result!(hyperg_1F1_e(1.5, 2.5, 1.0), 1.8834451238277954398, TEST_TOL0);
        check_result!(hyperg_1F1_e(1.5, 2.5, 10.0), 3128.7352996840916381, TEST_TOL1);
        check_result!(hyperg_1F1_e(10.0, 1.1, 1.0), 110.17623733873889579, TEST_TOL1);
        check_result!(hyperg_1F1_e(10.0, 1.1, 10.0), 6.146657975268385438e+09, TEST_TOL1);
        check_result!(hyperg_1F1_e(10.0, 1.1, 100.0), 9.331833897230312331e+55, TEST_TOL2);
        check_result!(hyperg_1F1_e(10.0, 1.1, 500.0), 4.519403368795715843e+235, TEST_TOL2);
        check_result!(hyperg_1F1_e(10.0, 50.1, 2.0), 1.5001295507968071788, TEST_TOL0);
        check_result!(hyperg_1F1_e(10.0, 50.1, 10.0), 8.713385849265044908, TEST_TOL0);
        check_result!(hyperg_1F1_e(10.0, 50.1, 100.0), 5.909423932273380330e+18, TEST_TOL2);
        check_result!(hyperg_1F1_e(10.0, 50.1, 500.0), 9.740060618457198900e+165, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, 1.0), 5.183531067116809033e+07, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, 10.0), 1.6032649110096979462e+28, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, 100.0), 1.1045151213192280064e+110, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 50.1, 1.0), 7.222953133216603757, TEST_TOL1);
        check_result!(hyperg_1F1_e(100.0, 50.1, 10.0), 1.0998696410887171538e+08, TEST_TOL1);
        check_result!(hyperg_1F1_e(100.0, 50.1, 100.0), 7.235304862322283251e+63, TEST_TOL2);
        check_result!(hyperg_1F1_e(1.0, 1.5, -1.0), 0.5380795069127684191, TEST_TOL0);
        check_result!(hyperg_1F1_e(1.0, 1.5, -10.0), 0.05303758099290164485, TEST_TOL1);
        check_result!(hyperg_1F1_e(1.0, 1.5, -100.0), 0.005025384718759852803, TEST_TOL1);
        check_result!(hyperg_1F1_e(1.0, 1.5, -500.0), 0.0010010030151059555322, TEST_TOL1);
        check_result!(hyperg_1F1_e(1.0, 1.1, -500.0), 0.00020036137599690208265, TEST_TOL1);
        check_result!(hyperg_1F1_e(10.0, 1.1, -1.0), 0.07227645648935938168, TEST_TOL1);
        check_result!(hyperg_1F1_e(10.0, 1.1, -10.0), 0.0003192415409695588126, TEST_TOL1);
        check_result!(hyperg_1F1_e(10.0, 1.1, -500.0), -3.400379216707701408e-23, TEST_TOL2);
        check_result!(hyperg_1F1_e(50.0, 1.1, -100.0), 4.632883869540640460e-24, TEST_SQRT_TOL0);
        check_result!(hyperg_1F1_e(50.0, 1.1, -110.0), 5.642684651305310023e-26, 0.03);
        check_result!(hyperg_1F1_e(100.0, 1.1, -1.0), 0.0811637344096042096, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, -10.0), 0.00025945610092231574387, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, -50.0), 2.4284830988994084452e-13, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, -90.0), 2.4468224638378426461e-22, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, -99.0), 1.0507096272617608461e-23, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, -100.0), 1.8315497474210138602e-24, TEST_TOL2);
        check_result!(hyperg_1F1_e(100.0, 1.1, -101.0), -2.3916306291344452490e-24, 0.04);
        check_result!(hyperg_1F1_e(100.0, 1.1, -110.0), -4.517581986037732280e-26, TEST_TOL0);
        check_result!(hyperg_1F1_e(100.0, 10.1, -220.0), -4.296130300021696573e-64, TEST_TOL1);
        check_result!(hyperg_1F1_e(-10.0, -10.1, 10.0), 10959.603204633058116, TEST_TOL1);
        check_result!(hyperg_1F1_e(-10.0, -10.1, 1000.0), 2.0942691895502242831e+23, TEST_TOL2);
        check_result!(hyperg_1F1_e(-10.0, -100.1, 10.0), 2.6012036337980078062, TEST_TOL1);
        check_result!(hyperg_1F1_e(-1000.0, -1000.1, 10.0), 22004.341698908631636, TEST_TOL3);
        check_result!(hyperg_1F1_e(-1000.0, -1000.1, 200.0), 7.066514294896245043e+86, TEST_TOL3);
        check_result!(hyperg_1F1_e(-8.1, -10.1, -10.0), 0.00018469685276347199258, TEST_TOL0);
        check_result!(hyperg_1F1_e(-8.1, -1000.1, -10.0), 0.9218280185080036020, TEST_TOL0);
        check_result!(hyperg_1F1_e(-10.0, -5.1, 1.0), 16.936141866089601635, TEST_TOL2);
        check_result!(hyperg_1F1_e(-10.0, -5.1, 10.0), 771534.0349543820541, TEST_TOL2);
        check_result!(hyperg_1F1_e(-10.0, -5.1, 100.0), 2.2733956505084964469e+17, TEST_TOL2);
        check_result!(hyperg_1F1_e(-100.0, -50.1, -1.0), 0.13854540373629275583, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, -50.1, -10.0), -9.142260314353376284e+19, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, -50.1, -100.0), -1.7437371339223929259e+87, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, -50.1, 1.0), 7.516831748170351173, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, -50.1, 10.0), 1.0551632286359671976e+11, TEST_SQRT_TOL0);
        check_result!(hyperg_1F1_e(-100.0, -50.1, 50.0), -7.564755600940346649e+36, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, -50.1, 100.0), 4.218776962675977e+55, TEST_TOL3);
        check_result!(hyperg_1F1_e(-10.5, -8.1, 0.1), 1.1387201443786421724, TEST_TOL0);
        check_result!(hyperg_1F1_e(-10.5, -11.1, 1.0), 2.5682766147138452362, TEST_TOL1);
        check_result!(hyperg_1F1_e(-100.5, -80.1, 10.0), 355145.4517305220603, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.5, -102.1, 10.0), 18678.558725244365016, TEST_TOL1);
        check_result!(hyperg_1F1_e(-100.5, -500.1, 10.0), 7.342209011101454, TEST_TOL0);
        check_result!(hyperg_1F1_e(-100.5, -500.1, 100.0), 1.2077443075367177662e+8, TEST_TOL1);
        check_result!(hyperg_1F1_e(-500.5, -80.1, 2.0), 774057.8541325341699, TEST_TOL4);
        check_result!(hyperg_1F1_e(100.0, -10.1, 1.0), -2.1213846338338567395e+12, TEST_TOL0);
        check_result!(hyperg_1F1_e(100.0, -10.1, 10.0), -6.624849346145112398e+39, TEST_TOL0);
        check_result!(hyperg_1F1_e(100.0, -10.1, 100.0), -1.2413466759089171904e+129, TEST_TOL0);
        check_result!(hyperg_1F1_e(100.0, -10.1, -1.0), 34456.29405305551691, TEST_TOL0);
        check_result!(hyperg_1F1_e(100.0, -10.1, -10.0), -7.809224251467710833e+07, TEST_TOL0);
        check_result!(hyperg_1F1_e(100.0, -10.1, -100.0), -5.214065452753988395e-07, TEST_TOL0);
        check_result!(hyperg_1F1_e(-100.0, 1.1, 1.0), 0.21519810496314438414, TEST_TOL2);
        check_result!(hyperg_1F1_e(-100.0, 1.1, 10.0), 8.196123715597869948, TEST_TOL1);
        check_result!(hyperg_1F1_e(-100.0, 1.1, 100.0), -1.4612966715976530293e+20, TEST_TOL1);
        check_result!(hyperg_1F1_e(-100.0, 20.1, 1.0), 0.0021267655527278456412, TEST_TOL2);
        check_result!(hyperg_1F1_e(-100.0, 20.1, 10.0), 2.0908665169032186979e-11, TEST_TOL2);
        check_result!(hyperg_1F1_e(-100.0, 20.1, 100.0), -0.04159447537001340412, TEST_TOL2);
        check_result!(hyperg_1F1_e(-100.0, 1.1, -1.0), 2.1214770215694685282e+07, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, 1.1, -10.0), 1.0258848879387572642e+24, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, 1.1, -100.0), 1.1811367147091759910e+67, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, 50.1, -1.0), 6.965259317271427390, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, 50.1, -10.0), 1.0690052487716998389e+07, TEST_TOL3);
        check_result!(hyperg_1F1_e(-100.0, 50.1, -100.0), 6.889644435777096248e+36, TEST_TOL3);
        check_result!(hyperg_1F1_e(-2.05, 1.0, 5.05), 3.79393389516785e+00, TEST_TOL3);
        check_result!(hyperg_1F1_e(-26.0, 2.0, 100.0), 1.444786781107436954e+19, TEST_TOL3);
        check_result!(hyperg_1F1_e(-26.1, 2.0, 100.0), 1.341557199575986995e+19, TEST_TOL3);
        check_result!(hyperg_1F1_e(1.2, 1.1e-15, 1.5), 8254503159672429.02, TEST_TOL3);
        check_result!(hyperg_1F1_e(1.0, 1000000.5, 0.8e6 + 0.5), 4.999922505099443804e+00, TEST_TOL3);
        check_result!(hyperg_1F1_e(1.0, 1000000.5, 1001000.5), 3480.3699557431856166, TEST_TOL4);
        check_result!(hyperg_1F1_e(1.1, 1000000.5, 1001000.5), 7304.6126942641350122, TEST_TOL3);
        check_result!(hyperg_1F1_e(0.9, 1000000.5, 1001000.5), 1645.4879293475410982, TEST_TOL3);
        check_result!(hyperg_1F1_e(-1.1, 1000000.5, 1001000.5), -5.30066488697455e-04, TEST_TOL3);
        check_result!(hyperg_1F1_e(1.5, 1000000.5, 0.8e6 + 0.5), 11.18001288977894650469927615, TEST_TOL4);
        check_result!(hyperg_1F1_e(-37.8, 2.01, 103.58), -6.21927211009e17, TEST_TOL1);
        check_result!(hyperg_1F1_e(-1.0, -1.0, 0.1), 1.1, TEST_TOL0);
    }

    #[test]
    fn test_hyperg_1F1_int_e() {
        check_result!(hyperg_1F1_int_e(1, 1, 0.5), 1.6487212707001281468, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(1, 2, 500.0), 2.8071844357056748215e+214, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(1, 2, -500.0), 0.002, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(8, 1, 0.5), 13.108875178030540372, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 1, 1.0), 131.63017574352619931, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 1, 10.0), 8.514625476546280796e+09, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 1, 100.0), 1.5671363646800353320e+56, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(10, 20, 1.0), 1.6585618002669675465, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(10, 20, 10.0), 265.26686430340188871, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(10, 20, 100.0), 3.640477355063227129e+34, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(10, 100, 1.0), 1.1056660194025527099, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, 10.0), 2.8491063634727594206, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, 40.0), 133.85880835831230986, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, 80.0), 310361.16228011433406, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, 100.0), 8.032171336754168282e+07, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(10, 100, 500.0), 7.633961202528731426e+123, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 1, 1.0), 6.892842729046469965e+07, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(100, 1, 10.0), 2.4175917112200409098e+28, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(100, 1, 100.0), 1.9303216896309102993e+110, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 200, 1.0), 1.6497469106162459226, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 200, 10.0), 157.93286197349321981, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 200, 100.0), 2.1819577501255075240e+24, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 200, 400.0), 3.728975529926573300e+119, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 400, 10.0), 12.473087623658878813, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(100, 400, 100.0), 9.071230376818550241e+11, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(100, 400, 150.0), 7.160949515742170775e+18, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(100, 400, 200.0), 2.7406690412731576823e+26, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 400, 300.0), 6.175110613473276193e+43, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 400, 400.0), 1.1807417662711371440e+64, TEST_TOL3);
        check_result!(hyperg_1F1_int_e(100, 400, 600.0), 2.4076076354888886030e+112, TEST_TOL3);
        check_result!(hyperg_1F1_int_e(10, 1, -1.0), 0.11394854824644542810, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 1, -10.0), 0.0006715506365396127863, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(10, 1, -100.0), -4.208138537480269868e-32, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(10, 50, -1.0), 0.820006196079380, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, -10.0), 0.38378859043466243, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, -100.0), 0.0008460143401464189061, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, -500.0), 1.1090822141973655929e-08, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(10, 100, -10000.0), 5.173783508088272292e-21, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(50, 1, -90.0), -1.6624258547648311554e-21, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(50, 1, -100.0), 4.069661775122048204e-24, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(50, 1, -110.0), 1.0072444993946236025e-25, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 10, -100.0), -2.7819353611733941962e-37, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 1, -90.0), 7.501705041159802854e-22, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(100, 1, -100.0), 6.305128893152291187e-25, TEST_TOL3);
        check_result!(hyperg_1F1_int_e(100, 1, -110.0), -7.007122115422439755e-26, TEST_TOL3);
        check_result!(hyperg_1F1_int_e(100, 10, -100.0), -2.7819353611733941962e-37, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(200, 50, -1.0), 0.016087060191732290813, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(200, 50, -300.0), -4.294975979706421471e-121, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(200, 100, -1.0), 0.13397521083325179687, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(200, 100, -10.0), 5.835134393749807387e-10, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(200, 100, -100.0), 4.888460453078914804e-74, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(200, 100, -500.0), -1.4478509059582015053e-195, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(-1, 1, 2.0), -1.0, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-1, -2, 2.0), 2.0, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-2, -3, 2.0), 3.0, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, 1, 1.0), 0.4189459325396825397, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, 1, 10.0), 27.984126984126984127, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, 1, 100.0), 9.051283795429571429e+12, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-100, 20, 1.0), 0.0020203016320697069566, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -20, 1.0), 1.6379141878548080173, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -20, 10.0), 78.65202404521289970, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -20, 100.0), 4.416169713262624315e+08, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -100, 1.0), 1.1046713999681950919, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -100, 10.0), 2.6035952191039006838, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -100, 100.0), 1151.6852040836932392, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-100, -200, 1.0), 1.6476859702535324743, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-100, -200, 10.0), 139.38026829540687270, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-100, -200, 100.0), 1.1669433576237933752e+19, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(-10, -20, -1.0), 0.6025549561148035735, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -20, -10.0), 0.00357079636732993491, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(-10, -20, -100.0), 1.64284868563391159e-35, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(-10, -100, -1.0), 0.90442397250313899, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-10, -100, -10.0), 0.35061515251367215, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(-10, -100, -100.0), 8.19512187960476424e-09, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(-100, -200, -1.0), 0.6061497939628952629, TEST_TOL0);
        check_result!(hyperg_1F1_int_e(-100, -200, -10.0), 0.0063278543908877674, TEST_TOL1);
        check_result!(hyperg_1F1_int_e(-100, -200, -100.0), 4.34111795007336552e-25, TEST_TOL2);
        check_result!(hyperg_1F1_int_e(-1, -1, 0.1), 1.1, TEST_TOL0);
    }

    #[test]
    fn test_hyperg_2F1_conj_e() {
        check_result!(hyperg_2F1_conj_e(1.0, 1.0, 1.0, 0.5), 3.352857095662929028, TEST_TOL0);
        check_result!(hyperg_2F1_conj_e(8.0, 8.0, 1.0, 0.5), 1.7078067538891293983e+09, TEST_TOL0);
        check_result!(hyperg_2F1_conj_e(8.0, 8.0, 5.0, 0.5), 285767.15696901140627, TEST_TOL1);
        check_result!(hyperg_2F1_conj_e(8.0, 8.0, 1.0, -0.5), 0.007248196261471276276, TEST_TOL3);
        check_result!(hyperg_2F1_conj_e(8.0, 8.0, 5.0, -0.5), 0.00023301916814505902809, TEST_TOL3);
        check_result!(hyperg_2F1_conj_e(25.0, 25.0, 1.0, -0.5), 5.1696944096e-06, TEST_SQRT_TOL0);
    }

    #[test]
    fn test_hyperg_2F1_conj_renorm_e() {
        check_result!(hyperg_2F1_conj_renorm_e(9.0, 9.0, -1.5, 0.99), 5.912269095984229412e+49, TEST_TOL2);
        check_result!(hyperg_2F1_conj_renorm_e(9.0, 9.0, -1.5, -0.99), 0.10834020229476124874, TEST_TOL2);
        check_result!(hyperg_2F1_conj_renorm_e(5.0, 5.0, -1.0, 0.5), 1.4885106335357933625e+08, TEST_TOL2);
        check_result!(hyperg_2F1_conj_renorm_e(5.0, 5.0, -10.0, 0.5), 7.968479361426355095e+21, TEST_TOL2);
        check_result!(hyperg_2F1_conj_renorm_e(5.0, 5.0, -100.0, 0.5), 3.1113180227052313057e+208, TEST_TOL3);
    }

    #[test]
    fn test_hyperg_2F1_e() {
        check_result!(hyperg_2F1_e(1.0, 1.0, 1.0, 0.5), 2.0, TEST_TOL0);
        check_result!(hyperg_2F1_e(8.0, 8.0, 1.0, 0.5), 12451584.0, TEST_TOL0);
        check_result!(hyperg_2F1_e(8.0, -8.0, 1.0, 0.5), 0.13671875, TEST_TOL0);
        check_result!(hyperg_2F1_e(8.0, -8.1, 1.0, 0.5), 0.14147385378899930422, TEST_TOL4);
        check_result!(hyperg_2F1_e(8.0, -8.0, 1.0, -0.5), 4945.136718750000000, TEST_TOL0);
        check_result!(hyperg_2F1_e(8.0, -8.0, -5.5, 0.5), -906.6363636363636364, TEST_TOL0);
        check_result!(hyperg_2F1_e(8.0, -8.0, -5.5, -0.5), 24565.363636363636364, TEST_TOL0);
        check_result!(hyperg_2F1_e(8.0, 8.0, 1.0, -0.5), -0.006476312098196747669, TEST_TOL2);
        check_result!(hyperg_2F1_e(8.0, 8.0, 5.0, 0.5), 4205.714285714285714, TEST_TOL0);
        check_result!(hyperg_2F1_e(8.0, 8.0, 5.0, -0.5), 0.0028489656290296436616, TEST_TOL2);
        check_result!(hyperg_2F1_e(9.0, 9.0, 1.0, 0.99), 1.2363536673577259280e+38, TEST_TOL2);
        check_result!(hyperg_2F1_e(9.0, 9.0, -1.5, 0.99), 3.796186436458346579e+46, TEST_TOL2);
        check_result!(hyperg_2F1_e(9.0, 9.0, -1.5, -0.99), 0.14733409946001025146, TEST_TOL1);
        check_result!(hyperg_2F1_e(9.0, 9.0, -8.5, 0.99), -1.1301780432998743440e+65, TEST_TOL2);
        check_result!(hyperg_2F1_e(9.0, 9.0, -8.5, -0.99), -8.856462606575344483, TEST_TOL1);
        check_result!(hyperg_2F1_e(9.0, 9.0, -21.5, 0.99), 2.0712920991876073253e+95, TEST_TOL3);
        check_result!(hyperg_2F1_e(9.0, 9.0, -21.5, -0.99), -74.30517015382249216, TEST_TOL2);
        check_result!(hyperg_2F1_e(9.0, 9.0, -100.5, 0.99), -3.186778061428268980e+262, TEST_TOL3);
        check_result!(hyperg_2F1_e(9.0, 9.0, -100.5, -0.99), 2.4454358338375677520, TEST_TOL1);
        check_result!(hyperg_2F1_e(25.0, 25.0, 1.0, -0.5), -2.9995530823639545027e-06, TEST_SQRT_TOL0);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, 1.0 + 1.0/64.0), 3.17175539044729373926, TEST_TOL3);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, 1.0 + 1.0/128.0), 3.59937243502024563424, TEST_TOL2);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, 1.0 + 1.0/256.0), 4.03259299524392504369, TEST_TOL1);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, 1.0 + 1.0/1024.0), 4.90784159359675398250, TEST_TOL1);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, 1.0 + 1.0/65536.0), 7.552266033399683914, TEST_TOL1);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, 1.0 + 1.0/16777216.0), 11.08235454026043830363, TEST_TOL1);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, -1.0 + 1.0/1024.0), 0.762910940909954974527, TEST_TOL0);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, -1.0 + 1.0/65536.0), 0.762762124908845424449, TEST_TOL0);
        check_result!(hyperg_2F1_e(1.5, 0.5, 2.0, -1.0 + 1.0/1048576.0), 0.762759911089064738044, TEST_TOL0);
        check_result!(hyperg_2F1_e(1.5, 0.5, 3.0, 1.0), 1.6976527263135502482014268, TEST_TOL2);
        check_result!(hyperg_2F1_e(0.1, -2.7, -1.5, 1.0), 1.059766766063610122925, TEST_TOL2);
        check_result!(hyperg_2F1_e(0.0, -2.0, -4.0, 0.5), 1.0, TEST_TOL2);
        check_result!(hyperg_2F1_e(-10.34, 2.05, 3.05, 0.1725), 0.310473552213130010351006093079548, TEST_TOL2);
        check_result!(hyperg_2F1_e(-9.99999999999, 2.05, 3.05, 0.1725), 0.32141934630197487540298837643890, TEST_TOL2);
        check_result!(hyperg_2F1_e(11.0, -1.0, 11.0/2.0, 0.125), 0.75, TEST_TOL2);
        check_result!(hyperg_2F1_e(-0.2, 8.8, 10.0, 0.8), 0.77998971427681563, TEST_TOL1);
        check_result!(hyperg_2F1_e(-0.2, 9.8, 11.0, 0.8), 0.77574573497387267, TEST_TOL0);
        check_result!(hyperg_2F1_e(3.5, -0.5, 5.0, 0.9), 0.5923981284370653465208973272, TEST_TOL2);
        check_result!(hyperg_2F1_e(-1.0, -10.0, 1.0, 0.5), 6.0, TEST_TOL0);
        check_result!(hyperg_2F1_e(3.23191, -4.0229, 8.02291, 0.5), 0.4300243900348170646, TEST_TOL2);
    }

    #[test]
    fn test_hyperg_2F1_renorm_e() {
        check_result!(hyperg_2F1_renorm_e(1.0, 1.0, 1.0, 0.5), 2.0, TEST_TOL0);
        check_result!(hyperg_2F1_renorm_e(8.0, 8.0, 1.0, 0.5), 12451584.0, TEST_TOL0);
        check_result!(hyperg_2F1_renorm_e(8.0, -8.0, 1.0, 0.5), 0.13671875, TEST_TOL0);
        check_result!(hyperg_2F1_renorm_e(8.0, -8.0, 1.0, -0.5), 4945.13671875, TEST_TOL0);
        check_result!(hyperg_2F1_renorm_e(8.0, -8.0, -5.5, 0.5), -83081.19167659493609, TEST_TOL2);
        check_result!(hyperg_2F1_renorm_e(8.0, -8.0, -5.5, -0.5), 2.2510895952730178518e+06, TEST_TOL2);
        check_result!(hyperg_2F1_renorm_e(8.0, 8.0, 5.0, 0.5), 175.2380952380952381, TEST_TOL1);
        check_result!(hyperg_2F1_renorm_e(9.0, 9.0, -1.5, 0.99), 1.6063266334913066551e+46, TEST_TOL2);
        check_result!(hyperg_2F1_renorm_e(9.0, 9.0, -1.5, -0.99), 0.06234327316254516616, TEST_TOL2);
        check_result!(hyperg_2F1_renorm_e(5.0, 5.0, -1.0, 0.5), 4949760.0, TEST_TOL1);
        check_result!(hyperg_2F1_renorm_e(5.0, 5.0, -10.0, 0.5), 139408493229637632000.0, TEST_TOL2);
        check_result!(hyperg_2F1_renorm_e(5.0, 5.0, -100.0, 0.5), 3.0200107544594411315e+206, TEST_TOL3);
    }

    #[test]
    fn test_hyperg_U_e() {
        check_result!(hyperg_U_e(0.0001, 0.0001, 0.0001), 1.0000576350699863577, TEST_TOL1);
        check_result!(hyperg_U_e(0.0001, 0.0001, 1.0), 0.9999403679233247536, TEST_TOL0);
        check_result!(hyperg_U_e(0.0001, 0.0001, 100.0), 0.9995385992657260887, TEST_TOL0);
        check_result!(hyperg_U_e(0.0001, 1.0, 0.0001), 1.0009210608660065989, TEST_TOL2);
        check_result!(hyperg_U_e(0.0001, 1.0, 1.0), 0.9999999925484179084, TEST_TOL2);
        check_result!(hyperg_U_e(0.0001, 10.0, 1.0), 13.567851006281412726, TEST_TOL3);
        check_result!(hyperg_U_e(0.0001, 10.0, 10.0), 0.9999244381454633265, TEST_TOL0);
        check_result!(hyperg_U_e(0.0001, 100.0, 98.0), 0.9998517867411840044, TEST_TOL2);
        check_result!(hyperg_U_e(0.0001, 1000.0, 999.0), 0.9997195294193261604, TEST_TOL2);
        check_result!(hyperg_U_e(0.0001, 1000.0, 1100.0), 0.9995342990014584713, TEST_TOL1);
        check_result!(hyperg_U_e(0.5, 1000.0, 800.0), 9.103916020464797207e+08, TEST_TOL2);
        check_result!(hyperg_U_e(0.5, 1000.0, 998.0), 0.21970269691801966806, TEST_TOL2);
        check_result!(hyperg_U_e(0.5, 0.5, 1.0), 0.7578721561413121060, TEST_TOL2);
        check_result!(hyperg_U_e(1.0, 0.0001, 0.0001), 0.9992361337764090785, TEST_TOL1);
        check_result!(hyperg_U_e(1.0, 0.0001, 1.0), 0.4036664068111504538, TEST_TOL2);
        check_result!(hyperg_U_e(1.0, 0.0001, 100.0), 0.009805780851264329587, TEST_TOL1);
        check_result!(hyperg_U_e(1.0, 1.2, 2.0), 0.3835044780075602550, TEST_TOL1);
        check_result!(hyperg_U_e(1.0, -0.0001, 1.0), 0.4036388693605999482, TEST_TOL1);
        check_result!(hyperg_U_e(8.0, 10.5, 1.0), 27.981926466707438538, TEST_TOL1);
        check_result!(hyperg_U_e(8.0, 10.5, 10.0), 2.4370135607662056809e-8, TEST_TOL0);
        check_result!(hyperg_U_e(8.0, 10.5, 100.0), 1.1226567526311488330e-16, TEST_TOL2);
        check_result!(hyperg_U_e(10.0, -2.5, 10.0), 6.734690720346560349e-14, TEST_TOL1);
        check_result!(hyperg_U_e(10.0, 2.5, 10.0), 6.787780794037971638e-13, TEST_TOL0);
        check_result!(hyperg_U_e(10.0, 2.5, 50.0), 2.4098720076596087125e-18, TEST_TOL0);
        check_result!(hyperg_U_e(-10.5, 1.1, 1.0), -3.990841457734147e+6, TEST_TOL2);
        check_result!(hyperg_U_e(-10.5, 1.1, 10.0), 1.307472052129343e+8, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 1.1, 50.0), 3.661978424114088e+16, TEST_TOL0);
        check_result!(hyperg_U_e(-10.5, 1.1, 90.0), 8.09469542130868e+19, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 1.1, 99.0), 2.546328328942063e+20, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 1.1, 100.0), 2.870463201832814e+20, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 1.1, 200.0), 8.05143453769373e+23, TEST_TOL2);
        check_result!(hyperg_U_e(-10.5, 10.1, 0.1), -3.043016255306515e+20, TEST_TOL2);
        check_result!(hyperg_U_e(-10.5, 10.1, 1.0), -3.194745265896115e+12, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 10.1, 4.0), -6.764203430361954e+07, TEST_TOL2);
        check_result!(hyperg_U_e(-10.5, 10.1, 10.0), -2.067399425480545e+09, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 10.1, 50.0), 4.661837330822824e+14, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 100.4, 10.0), -6.805460513724838e+66, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 100.4, 50.0), -2.081052558162805e+18, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 100.4, 80.0), 2.034113191014443e+14, TEST_TOL2);
        check_result!(hyperg_U_e(-10.5, 100.4, 100.0), 6.85047268436107e+13, TEST_TOL1);
        check_result!(hyperg_U_e(-10.5, 100.4, 200.0), 1.430815706105649e+20, TEST_TOL2);
        check_result!(hyperg_U_e(-19.5, 82.1, 10.0), 5.464313196201917432e+60, TEST_TOL2);
        check_result!(hyperg_U_e(-50.5, 100.1, 10.0), -5.5740216266953e+126, TEST_TOL1);
        check_result!(hyperg_U_e(-50.5, 100.1, 40.0), 5.937463786613894e+91, TEST_TOL1);
        check_result!(hyperg_U_e(-50.5, 100.1, 50.0), -1.631898534447233e+89, TEST_TOL1);
        check_result!(hyperg_U_e(-50.5, 100.1, 70.0), 3.249026971618851e+84, TEST_TOL2);
        check_result!(hyperg_U_e(-50.5, 100.1, 100.0), 1.003401902126641e+85, TEST_TOL1);
        check_result!(hyperg_U_e(-2.0, 4.0, 1.0), 11.0, TEST_TOL0);
        check_result!(hyperg_U_e(-2.0, 0.5, 3.14), 1.1896, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, 0.5, 1.13), -1.3631, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, 0.5, 0.0), 0.75, TEST_TOL2);
        check_result!(hyperg_U_e(0.0, 0.0, -0.1), 1.0, TEST_TOL0);
        check_result!(hyperg_U_e(-1.0, 0.0, -0.1), -0.1, TEST_TOL0);
        check_result!(hyperg_U_e(-2.0, 0.0, -0.1), 0.21, TEST_TOL0);
        check_result!(hyperg_U_e(-3.0, 0.0, -0.1), -0.661, TEST_TOL0);
        check_result!(hyperg_U_e(-4.0, 0.0, -0.1), 2.7721, TEST_TOL0);
        check_result!(hyperg_U_e(-5.0, 0.0, -0.1), -14.52201, TEST_TOL0);
        check_result!(hyperg_U_e(-6.0, 0.0, -0.1), 91.230301, TEST_TOL0);
        check_result!(hyperg_U_e(0.0, 1.0, -0.1), 1.0, TEST_TOL0);
        check_result!(hyperg_U_e(-1.0, 1.0, -0.1), -1.1, TEST_TOL0);
        check_result!(hyperg_U_e(-2.0, 1.0, -0.1), 2.41, TEST_TOL1);
        check_result!(hyperg_U_e(-3.0, 1.0, -0.1), -7.891, TEST_TOL2);
        check_result!(hyperg_U_e(-4.0, 1.0, -0.1), 34.3361, TEST_TOL2);
        check_result!(hyperg_U_e(-5.0, 1.0, -0.1), -186.20251, TEST_TOL2);
        check_result!(hyperg_U_e(-6.0, 1.0, -0.1), 1208.445361, TEST_TOL2);
        check_result!(hyperg_U_e(1.0, 2.0, -0.1), -10.0, TEST_TOL0);
        check_result!(hyperg_U_e(0.0, 2.0, -0.1), 1.0, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, 2.0, -0.1), -2.1, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, 2.0, -0.1), 6.61, TEST_TOL2);
        check_result!(hyperg_U_e(-3.0, 2.0, -0.1), -27.721, TEST_TOL2);
        check_result!(hyperg_U_e(-4.0, 2.0, -0.1), 145.2201, TEST_TOL2);
        check_result!(hyperg_U_e(-5.0, 2.0, -0.1), -912.30301, TEST_TOL2);
        check_result!(hyperg_U_e(-6.0, 2.0, -0.1), 6682.263421, TEST_TOL2);
        check_result!(hyperg_U_e(2.0, 3.0, -0.1), 100.0, TEST_TOL0);
        check_result!(hyperg_U_e(1.0, 3.0, -0.1), 90.0, TEST_TOL2);
        check_result!(hyperg_U_e(0.0, 3.0, -0.1), 1.0, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, 3.0, -0.1), -3.10, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, 3.0, -0.1), 12.81, TEST_TOL2);
        check_result!(hyperg_U_e(-3.0, 3.0, -0.1), -66.151, TEST_TOL2);
        check_result!(hyperg_U_e(-4.0, 3.0, -0.1), 409.8241, TEST_TOL2);
        check_result!(hyperg_U_e(-5.0, 3.0, -0.1), -2961.42351, TEST_TOL2);
        check_result!(hyperg_U_e(-6.0, 3.0, -0.1), 24450.804481, TEST_TOL2);
        check_result!(hyperg_U_e(3.0, 4.0, -0.1), -1000.0, TEST_TOL0);
        check_result!(hyperg_U_e(2.0, 4.0, -0.1), -1900.0, TEST_TOL2);
        check_result!(hyperg_U_e(1.0, 4.0, -0.1), -1810.0, TEST_TOL2);
        check_result!(hyperg_U_e(0.0, 4.0, -0.1), 1.0, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, 4.0, -0.1), -4.10, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, 4.0, -0.1), 21.01, TEST_TOL2);
        check_result!(hyperg_U_e(-3.0, 4.0, -0.1), -129.181, TEST_TOL2);
        check_result!(hyperg_U_e(-4.0, 4.0, -0.1), 926.5481, TEST_TOL2);
        check_result!(hyperg_U_e(-5.0, 4.0, -0.1), -7594.16401, TEST_TOL2);
        check_result!(hyperg_U_e(-6.0, 4.0, -0.1), 70015.788541, TEST_TOL2);
        check_result!(hyperg_U_e(0.0, -1.0, -0.1), 1.0, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, -1.0, -0.1), 0.9, TEST_TOL0);
        check_result!(hyperg_U_e(-2.0, -1.0, -0.1), 0.01, TEST_TOL0);
        check_result!(hyperg_U_e(-3.0, -1.0, -0.1), -0.031, TEST_TOL0);
        check_result!(hyperg_U_e(-4.0, -1.0, -0.1), 0.1281, TEST_TOL0);
        check_result!(hyperg_U_e(-5.0, -1.0, -0.1), -0.66151, TEST_TOL0);
        check_result!(hyperg_U_e(-6.0, -1.0, -0.1), 4.098241, TEST_TOL0);
        check_result!(hyperg_U_e(0.0, -2.0, -0.1), 1.0, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, -2.0, -0.1), 1.9, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, -2.0, -0.1), 1.81, TEST_TOL2);
        check_result!(hyperg_U_e(-3.0, -2.0, -0.1), -0.001, TEST_TOL0);
        check_result!(hyperg_U_e(-4.0, -2.0, -0.1), 0.0041, TEST_TOL0);
        check_result!(hyperg_U_e(-5.0, -2.0, -0.1), -0.02101, TEST_TOL0);
        check_result!(hyperg_U_e(-6.0, -2.0, -0.1), 0.129181, TEST_TOL0);
        check_result!(hyperg_U_e(0.0, -3.0, -0.1), 1.0, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, -3.0, -0.1), 2.9, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, -3.0, -0.1), 5.61, TEST_TOL2);
        check_result!(hyperg_U_e(-3.0, -3.0, -0.1), 5.429, TEST_TOL2);
        check_result!(hyperg_U_e(-4.0, -3.0, -0.1), 0.0001, TEST_TOL0);
        check_result!(hyperg_U_e(-5.0, -3.0, -0.1), -0.00051, TEST_TOL0);
        check_result!(hyperg_U_e(-6.0, -3.0, -0.1), 0.003121, TEST_TOL0);
        check_result!(hyperg_U_e(0.0, -4.0, -0.1), 1.0, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, -4.0, -0.1), 3.9, TEST_TOL2);
        check_result!(hyperg_U_e(-2.0, -4.0, -0.1), 11.41, TEST_TOL2);
        check_result!(hyperg_U_e(-3.0, -4.0, -0.1), 22.259, TEST_TOL2);
        check_result!(hyperg_U_e(-4.0, -4.0, -0.1), 21.7161, TEST_TOL2);
        check_result!(hyperg_U_e(-6.0, -4.0, -0.1), 0.000061, TEST_TOL0);
        check_result!(hyperg_U_e(-7.0, -4.0, -0.1), -0.0004341, TEST_TOL0);
        check_result!(hyperg_U_e(-3.0, 0.5, -0.5), -9.5, TEST_TOL2);
        check_result!(hyperg_U_e(-8.0, 0.5, -0.5), 180495.0625, TEST_TOL2);
        check_result!(hyperg_U_e(-8.0, 1.5, -0.5), 827341.0625, TEST_TOL2);
        check_result!(hyperg_U_e(-8.0, 1.5, -10.0), 7.162987810253906e9, TEST_TOL2);
        check_result!(hyperg_U_e(3.0, 6.0, -0.5), -296.0, TEST_TOL2);
        check_result!(hyperg_U_e(3.0, 7.0, -0.5), 2824.0, TEST_TOL2);
        check_result!(hyperg_U_e(5.0, 12.0, -1.7), -153.262676210016018065768591104, TEST_TOL2);
        check_result!(hyperg_U_e(0.0, 0.0, -0.5), 1.0, TEST_TOL0);
        check_result!(hyperg_U_e(0.0, 1.0, -0.5), 1.0, TEST_TOL0);
        check_result!(hyperg_U_e(0.0, 1.0, -0.001), 1.0, TEST_TOL0);
        check_result!(hyperg_U_e(-1.0, 0.99, -0.1), -1.09, TEST_TOL2);
        check_result!(hyperg_U_e(-1.0, 0.0, -0.5), -0.5, TEST_TOL0);
        check_result!(hyperg_U_e(-2.0, 0.0, -0.5), 1.25, TEST_TOL0);
        check_result!(hyperg_U_e(-7.0, 0.0, -0.1), -668.2263421, TEST_TOL0);
        check_result!(hyperg_U_e(4.11, 0.11, 6.4), 6.422378238765078623739153038e-5, TEST_TOL2);
        check_result!(hyperg_U_e(5.0, 4.0, 6.4), 3.2586223825343211136628535e-05, TEST_TOL2);
        check_result!(hyperg_U_e(2.2, 1.2, 8.7), 5.7250017539318661177749625e-03, TEST_TOL2);
        check_result!(hyperg_U_e(2.0, -6.4, 1.0), 1.2141502795806162484648638e-02, TEST_TOL2);
    }

    #[test]
    fn test_hyperg_U_int_e() {
        check_result!(hyperg_U_int_e(1, 1, 0.0001), 8.634088070212725330, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 1, 0.01), 4.078511443456425847, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 1, 0.5), 0.9229106324837304688, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 1, 2.0), 0.3613286168882225847, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 1, 100.0), 0.009901942286733018406, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 1, 1000.0), 0.0009990019940238807150, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 8, 0.01), 7.272361203006010000e+16, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 8, 1.0), 1957.0, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 8, 5.0), 1.042496, TEST_TOL1);
        check_result!(hyperg_U_int_e(1, 8, 8.0), 0.3207168579101562500, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 8, 50.0), 0.022660399001600000000, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 8, 100.0), 0.010631236727200000000, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 8, 1000.0), 0.0010060301203607207200, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 20, 1.0), 1.7403456103284421000e+16, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 20, 20.0), 0.22597813610531052969, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 50, 1.0), 3.374452117521520758e+61, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 50, 50.0), 0.15394136814987651785, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 100, 0.1), 1.0418325171990852858e+253, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, 100, 1.0), 2.5624945006073464385e+154, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, 100, 50.0), 3.0978624160896431391e+07, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, 100, 100.0), 0.11323192555773717475, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 100, 200.0), 0.009715680951406713589, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 100, 1000.0), 0.0011085142546061528661, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, 1000, 2000.0), 0.0009970168547036318206, TEST_TOL0);
        check_result!(hyperg_U_int_e(1, -1, 1.0), 0.29817368116159703717, TEST_TOL1);
        check_result!(hyperg_U_int_e(1, -1, 10.0), 0.07816669698940409380, TEST_TOL1);
        check_result!(hyperg_U_int_e(1, -10, 1.0), 0.08271753756946041959, TEST_TOL1);
        check_result!(hyperg_U_int_e(1, -10, 5.0), 0.06127757419425055261, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -10, 10.0), 0.04656199948873187212, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -10, 20.0), 0.031606421847946077709, TEST_TOL1);
        check_result!(hyperg_U_int_e(1, -100, 0.01), 0.009900000099999796950, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -100, 1.0), 0.009802970197050404429, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -100, 10.0), 0.009001648897173103447, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -100, 20.0), 0.008253126487166557546, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -100, 50.0), 0.006607993916432051008, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -100, 90.0), 0.005222713769726871937, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -100, 110.0), 0.004727658137692606210, TEST_TOL2);
        check_result!(hyperg_U_int_e(1, -1000, 1010.0), 0.0004971408839859245170, TEST_TOL4);
        check_result!(hyperg_U_int_e(8, 1, 0.001), 0.0007505359326875706975, TEST_TOL0);
        check_result!(hyperg_U_int_e(8, 1, 0.5), 6.449509938973479986e-06, TEST_TOL3);
        check_result!(hyperg_U_int_e(8, 1, 8.0), 6.190694573035761284e-10, TEST_TOL0);
        check_result!(hyperg_U_int_e(8, 1, 20.0), 3.647213845460374016e-12, TEST_TOL0);
        check_result!(hyperg_U_int_e(8, 8, 1.0), 0.12289755012652317578, TEST_TOL1);
        check_result!(hyperg_U_int_e(8, 8, 10.0), 5.687710359507564272e-09, TEST_TOL1);
        check_result!(hyperg_U_int_e(8, 8, 20.0), 2.8175404594901039724e-11, TEST_TOL1);
        check_result!(hyperg_U_int_e(100, 100, 0.01), 1.0099979491941914867e+196, TEST_TOL2);
        check_result!(hyperg_U_int_e(100, 100, 0.1), 1.0090713562719862833e+97, TEST_TOL2);
        check_result!(hyperg_U_int_e(100, 100, 1.0), 0.009998990209084729106, TEST_TOL2);
        check_result!(hyperg_U_int_e(100, 100, 20.0), 1.3239363905866130603e-131, TEST_TOL2);
        check_result!(hyperg_U_int_e(-10, 1, 0.01), 3.274012540759009536e+06, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 1, 1.0), 1.5202710000000000000e+06, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 1, 10.0), 1.0154880000000000000e+08, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 1, 100.0), 3.284529863685482880e+19, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 10, 1.0), 1.1043089864100000000e+11, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 100, 1.0), 1.3991152402448957897e+20, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 100, 10.0), 5.364469916567136000e+19, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 100, 100.0), 3.909797568000000000e+12, TEST_TOL0);
        check_result!(hyperg_U_int_e(-10, 100, 500.0), 8.082625576697984130e+25, TEST_TOL0);
        check_result!(hyperg_U_int_e(-50, 1, 0.01), 1.6973422555823855798e+64, TEST_TOL2);
        check_result!(hyperg_U_int_e(-50, 1, 1.0), 7.086160198304780325e+63, TEST_TOL1);
        check_result!(hyperg_U_int_e(-50, 1, 10.0), 5.332862895528712200e+65, TEST_TOL1);
        check_result!(hyperg_U_int_e(-50, 10, 1.0), -7.106713471565790573e+71, TEST_TOL1);
        check_result!(hyperg_U_int_e(-50, 100, 1.0), 2.4661377199407186476e+104, TEST_TOL1);
        check_result!(hyperg_U_int_e(-50, 10, 10.0), 5.687538583671241287e+68, TEST_TOL1);
        check_result!(hyperg_U_int_e(-50, 100, 10.0), 1.7880761664553373445e+102, TEST_TOL1);
        check_result!(hyperg_U_int_e(-90, 1, 0.01), 4.185245354032917715e+137, TEST_TOL2);
        check_result!(hyperg_U_int_e(-90, 1, 0.1), 2.4234043408007841358e+137, TEST_TOL3);
        check_result!(hyperg_U_int_e(-90, 1, 10.0), -1.8987677149221888807e+139, TEST_TOL1);
        check_result!(hyperg_U_int_e(-90, 10, 10.0), -5.682999988842066677e+143, TEST_TOL1);
        check_result!(hyperg_U_int_e(-90, 100, 10.0), 2.3410029853990624280e+189, TEST_TOL2);
        check_result!(hyperg_U_int_e(-90, 1000, 10.0), 1.9799451517572225316e+271, TEST_TOL3);
        check_result!(hyperg_U_int_e(-50, -1, 10.0), -9.083195466262584149e+64, TEST_TOL1);
        check_result!(hyperg_U_int_e(-50, -10, 10.0), -1.4418257327071634407e+62, TEST_TOL1);
        check_result!(hyperg_U_int_e(-50, -100, 0.01), 3.0838993811468983931e+93, TEST_TOL2);
        check_result!(hyperg_U_int_e(-50, -100, 10.0), 4.014552630378340665e+95, TEST_TOL2);
        check_result!(hyperg_U_int_e(-100, -100, 10.0), 2.0556466922347982030e+162, TEST_TOL2);
        check_result!(hyperg_U_int_e(-100, -200, 10.0), 1.1778399522973555582e+219, TEST_TOL2);
        check_result!(hyperg_U_int_e(-100, -200, 100.0), 9.861313408898201873e+235, TEST_TOL3);
        check_result!(hyperg_U_int_e(3, 6, -0.5), -296.0, TEST_TOL0);
        check_result!(hyperg_U_int_e(3, 7, -0.5), 2824.0, TEST_TOL0);
        check_result!(hyperg_U_int_e(5, 12, -1.7), -153.262676210016018065768591104, TEST_TOL2);
    }
}