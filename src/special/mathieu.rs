/*
    special/mathieu.rs
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

/// These routines compute the radial `j`-th kind Mathieu functions
/// $Mc_n^{(j)}(q,x)$ and $Ms_n^{(j)}(q,x)$ of order `n`.
/// 
/// The allowed values of `j` are 1 and 2. The functions for $j = 3,4$ can be
/// computed as $M_n^{(3)} = M_n^{(1)} + iM_n^{(2)}$ and $M_n^{(4)} = M_n^{(1)} -
/// iM_n^{(2)}$, where $M_n^{(j)} = Mc_n^{(j)}$ or $Ms_n^{(j)}$.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_Mc_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_Mc_e).
pub fn mathieu_Mc_e(j: i32, n: i32, q: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_mathieu_Mc_e(j, n, q, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the characteristic values $a_n(q)$, $b_n(q)$ of the
/// Mathieu functions $ce_n(q,x)$ and $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_b_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_b_e).
pub fn mathieu_b_e(n: i32, q: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_mathieu_b_e(n, q, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the characteristic values $a_n(q)$, $b_n(q)$ of the
/// Mathieu functions $ce_n(q,x)$ and $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_a_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_a_e).
pub fn mathieu_a_e(n: i32, q: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_mathieu_a_e(n, q, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the radial `j`-th kind Mathieu functions
/// $Mc_n^{(j)}(q,x)$ and $Ms_n^{(j)}(q,x)$ of order `n`.
/// 
/// The allowed values of `j` are 1 and 2. The functions for $j = 3,4$ can be
/// computed as $M_n^{(3)} = M_n^{(1)} + iM_n^{(2)}$ and $M_n^{(4)} = M_n^{(1)} -
/// iM_n^{(2)}$, where $M_n^{(j)} = Mc_n^{(j)}$ or $Ms_n^{(j)}$.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_Ms_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_Ms_e).
pub fn mathieu_Ms_e(j: i32, n: i32, q: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_mathieu_Ms_e(j, n, q, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the angular Mathieu functions $ce_n(q,x)$ and
/// $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_ce_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_ce_e).
pub fn mathieu_ce_e(n: i32, q: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_mathieu_ce_e(n, q, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the angular Mathieu functions $ce_n(q,x)$ and
/// $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_se_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_se_e).
pub fn mathieu_se_e(n: i32, q: f64, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_mathieu_se_e(n, q, x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the radial `j`-th kind Mathieu functions
/// $Mc_n^{(j)}(q,x)$ and $Ms_n^{(j)}(q,x)$ of order `n`.
/// 
/// The allowed values of `j` are 1 and 2. The functions for $j = 3,4$ can be
/// computed as $M_n^{(3)} = M_n^{(1)} + iM_n^{(2)}$ and $M_n^{(4)} = M_n^{(1)} -
/// iM_n^{(2)}$, where $M_n^{(j)} = Mc_n^{(j)}$ or $Ms_n^{(j)}$.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_Mc`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_Mc).
pub fn mathieu_Mc(j: i32, n: i32, q: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_mathieu_Mc(j, n, q, x) }
}

/// These routines compute the angular Mathieu functions $ce_n(q,x)$ and
/// $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_se`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_se).
pub fn mathieu_se(n: i32, q: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_mathieu_se(n, q, x) }
}

/// These routines compute the angular Mathieu functions $ce_n(q,x)$ and
/// $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_ce`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_ce).
pub fn mathieu_ce(n: i32, q: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_mathieu_ce(n, q, x) }
}

/// These routines compute the characteristic values $a_n(q)$, $b_n(q)$ of the
/// Mathieu functions $ce_n(q,x)$ and $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_b`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_b).
pub fn mathieu_b(n: i32, q: f64) -> f64 {
    unsafe { bindings::gsl_sf_mathieu_b(n, q) }
}

/// These routines compute the characteristic values $a_n(q)$, $b_n(q)$ of the
/// Mathieu functions $ce_n(q,x)$ and $se_n(q,x)$, respectively.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_a`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_a).
pub fn mathieu_a(n: i32, q: f64) -> f64 {
    unsafe { bindings::gsl_sf_mathieu_a(n, q) }
}

/// These routines compute the radial `j`-th kind Mathieu functions
/// $Mc_n^{(j)}(q,x)$ and $Ms_n^{(j)}(q,x)$ of order `n`.
/// 
/// The allowed values of `j` are 1 and 2. The functions for $j = 3,4$ can be
/// computed as $M_n^{(3)} = M_n^{(1)} + iM_n^{(2)}$ and $M_n^{(4)} = M_n^{(1)} -
/// iM_n^{(2)}$, where $M_n^{(j)} = Mc_n^{(j)}$ or $Ms_n^{(j)}$.
/// 
/// Binds the function
/// [`gsl_sf_mathieu_Ms`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_mathieu_Ms).
pub fn mathieu_Ms(j: i32, n: i32, q: f64, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_mathieu_Ms(j, n, q, x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_mathieu_ce_e() {
        check_result!(mathieu_ce_e(0, 0.0, 0.0), 0.7071067811865475, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 0.0, M_PI_2), 0.7071067811865475, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 5.0, 0.0), 0.04480018165188902, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 5.0, M_PI_2), 1.334848674698019, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 10.0, 0.0), 0.007626517570935782, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 10.0, M_PI_2), 1.468660470712856, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 15.0, 0.0), 0.001932508315204592, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 15.0, M_PI_2), 1.550108146686649, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 20.0, 0.0), 0.0006037438292242197, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 20.0, M_PI_2), 1.609890857395926, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 25.0, 0.0), 0.0002158630184146612, TEST_SNGL);
        check_result!(mathieu_ce_e(0, 25.0, M_PI_2), 1.657510298323475, TEST_SNGL);
        check_result!(mathieu_ce_e(1, 0.0, 0.0), 1.00000000, TEST_SNGL);
        check_result!(mathieu_ce_e(1, 5.0, 0.0), 0.2565428793223637, TEST_SNGL);
        check_result!(mathieu_ce_e(1, 10.0, 0.0), 0.05359874774717657, TEST_SNGL);
        check_result!(mathieu_ce_e(1, 15.0, 0.0), 0.01504006645382623, TEST_SNGL);
        check_result!(mathieu_ce_e(1, 20.0, 0.0), 0.005051813764712904, TEST_SNGL);
        check_result!(mathieu_ce_e(1, 25.0, 0.0), 0.001911051506657645, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 0.0, 0.0), 1.00000000, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 0.0, M_PI_2), -1.00000000, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 5.0, 0.0), 0.7352943084006845, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 5.0, M_PI_2), -0.7244881519676682, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 10.0, 0.0), 0.2458883492913189, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 10.0, M_PI_2), -0.9267592641263211, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 15.0, 0.0), 0.07879282784639313, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 15.0, M_PI_2), -1.019966226030262, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 20.0, 0.0), 0.02864894314707431, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 20.0, M_PI_2), -1.075293228779687, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 25.0, 0.0), 0.0115128663308875, TEST_SNGL);
        check_result!(mathieu_ce_e(2, 25.0, M_PI_2), -1.116278953295253, TEST_SNGL);
        check_result!(mathieu_ce_e(5, 0.0, 0.0), 1.00000000, TEST_SNGL);
        check_result!(mathieu_ce_e(5, 5.0, 0.0), 1.12480725063848, TEST_SNGL);
        check_result!(mathieu_ce_e(5, 10.0, 0.0), 1.258019941308287, TEST_SNGL);
        check_result!(mathieu_ce_e(5, 15.0, 0.0), 1.193432230413072, TEST_SNGL);
        check_result!(mathieu_ce_e(5, 20.0, 0.0), 0.9365755314226215, TEST_SNGL);
        check_result!(mathieu_ce_e(5, 25.0, 0.0), 0.6106943100506986, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 0.0, 0.0), 1.00000000, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 0.0, M_PI_2), -1.00000000, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 5.0, 0.0), 1.025995027089438, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 5.0, M_PI_2), -0.975347487235964, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 10.0, 0.0), 1.053815992100935, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 10.0, M_PI_2), -0.9516453181789554, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 15.0, 0.0), 1.084106311839221, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 15.0, M_PI_2), -0.9285480638845388, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 20.0, 0.0), 1.117788631259397, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 20.0, M_PI_2), -0.9057107845940974, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 25.0, 0.0), 1.156239918632239, TEST_SNGL);
        check_result!(mathieu_ce_e(10, 25.0, M_PI_2), -0.8826919105636903, TEST_SNGL);
        check_result!(mathieu_ce_e(15, 0.0, 0.0), 1.00000000, TEST_SNGL);
        check_result!(mathieu_ce_e(15, 5.0, 0.0), 1.011293732529566, TEST_SNGL);
        check_result!(mathieu_ce_e(15, 10.0, 0.0), 1.022878282438181, TEST_SNGL);
        check_result!(mathieu_ce_e(15, 15.0, 0.0), 1.034793652236873, TEST_SNGL);
        check_result!(mathieu_ce_e(15, 20.0, 0.0), 1.047084344162887, TEST_SNGL);
        check_result!(mathieu_ce_e(15, 25.0, 0.0), 1.059800441813937, TEST_SNGL);
    }

    #[test]
    fn test_mathieu_se_e() {
        check_result!(mathieu_se_e(1, 0.0, M_PI_2), 1.0000000, TEST_SNGL);
        check_result!(mathieu_se_e(1, 5.0, M_PI_2), 1.337433887022345, TEST_SNGL);
        check_result!(mathieu_se_e(1, 10.0, M_PI_2), 1.468755664102938, TEST_SNGL);
        check_result!(mathieu_se_e(1, 15.0, M_PI_2), 1.550115074357552, TEST_SNGL);
        check_result!(mathieu_se_e(1, 20.0, M_PI_2), 1.609891592603772, TEST_SNGL);
        check_result!(mathieu_se_e(1, 25.0, M_PI_2), 1.657510398374516, TEST_SNGL);
        check_result!(mathieu_se_e(5, 0.0, M_PI_2), 1.0000000, TEST_SNGL);
        check_result!(mathieu_se_e(5, 5.0, M_PI_2), 0.9060779302023551, TEST_SNGL);
        check_result!(mathieu_se_e(5, 10.0, M_PI_2), 0.8460384335355106, TEST_SNGL);
        check_result!(mathieu_se_e(5, 15.0, M_PI_2), 0.837949340012484, TEST_SNGL);
        check_result!(mathieu_se_e(5, 20.0, M_PI_2), 0.8635431218533667, TEST_SNGL);
        check_result!(mathieu_se_e(5, 25.0, M_PI_2), 0.8992683245108413, TEST_SNGL);
        check_result!(mathieu_se_e(15, 0.0, M_PI_2), -1.0000000, TEST_SNGL);
        check_result!(mathieu_se_e(15, 5.0, M_PI_2), -0.9889607027406357, TEST_SNGL);
        check_result!(mathieu_se_e(15, 10.0, M_PI_2), -0.9781423471832157, TEST_SNGL);
        check_result!(mathieu_se_e(15, 15.0, M_PI_2), -0.9675137031854538, TEST_SNGL);
        check_result!(mathieu_se_e(15, 20.0, M_PI_2), -0.9570452540612817, TEST_SNGL);
        check_result!(mathieu_se_e(15, 25.0, M_PI_2), -0.9467086958780897, TEST_SNGL);
    }
}