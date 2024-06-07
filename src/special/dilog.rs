/*
    special/dilog.rs
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
/// These routines compute the dilogarithm for a real argument. In Lewin’s
/// notation this is $Li\_2(x)$, the real part of the dilogarithm of a
/// real $x$. It is defined by the integral representation
/// $Li\_2(x) = - \Re \int\_0^x ds \log(1-s) / s$
/// Note that $\Im(Li\_2(x)) = 0$ for
/// $x \le 1$, and $-\pi\log(x)$ for $x > 1$.
/// Note that Abramowitz & Stegun refer to the Spence integral
/// $S(x) = Li\_2(1 - x)$ as the dilogarithm rather than $Li\_2(x)$.
///
/// Binds the [`gsl_sf_dilog_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_dilog_e) function.
pub fn dilog(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_dilog_e(x, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::special::special_function_test::*;
    
    #[test]
    fn test_dilog() {
        disable_error_handler();

        check_result(
            dilog(-3.0),
            -1.9393754207667089531,
            TEST_TOL0
        );

        check_result(
            dilog(-0.5),
            -0.4484142069236462024,
            TEST_TOL0
        );

        check_result(
            dilog(-0.001),
            -0.0009997501110486510834,
            TEST_TOL0
        );

        check_result(
            dilog(0.1),
            0.1026177910993911,
            TEST_TOL0
        );

        check_result(
            dilog(0.7),
            0.8893776242860387386,
            TEST_TOL0
        );

        check_result(
            dilog(1.0),
            1.6449340668482260,
            TEST_TOL0
        );

        check_result(
            dilog(1.5),
            2.3743952702724802007,
            TEST_TOL0
        );

        check_result(
            dilog(2.0),
            2.4674011002723397,
            TEST_TOL0
        );

        check_result(
            dilog(5.0),
            1.7837191612666306277,
            TEST_TOL0
        );

        check_result(
            dilog(11.0),
            0.3218540439999117111,
            TEST_TOL1
        );

        check_result(
            dilog(12.59),
            0.0010060918167266208634,
            TEST_TOL3
        );

        check_result(
            dilog(12.595),
            0.00003314826006436236810,
            TEST_TOL5
        );

        check_result(
            dilog(13.0),
            -0.07806971248458575855,
            TEST_TOL2
        );

        check_result(
            dilog(20.0),
            -1.2479770861745251168,
            TEST_TOL2
        );

        check_result(
            dilog(150.0),
            -9.270042702348657270,
            TEST_TOL0
        );

        check_result(
            dilog(1100.0),
            -21.232504073931749553,
            TEST_TOL0
        );
    }
}
