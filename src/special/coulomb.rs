/*
    special/coulomb.rs
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
/// These routines compute the lowest-order normalized hydrogenic bound
/// state radial wavefunction
/// $R\_1 := 2Z \sqrt{Z} \exp(-Z r)$.
///
/// Binds the [`gsl_sf_hydrogenicR_1_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hydrogenicR_1_e) function.
#[allow(non_snake_case)]
pub fn hydrogenicR_1(z: f64, r: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hydrogenicR_1_e(z, r, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the `n`-th normalized hydrogenic bound state
/// radial wavefunction,
/// $R\_n := {2 Z^{3/2} \over n^2} \left({2Z r \over n}\right)^l \sqrt{(n-l-1)! \over (n+l)!} \exp(-Z r/n) L^{2l+1}\_{n-l-1}(2Z r / n).$
/// where $L^a\_b(x)$ is the generalized Laguerre polynomial.
/// The normalization is chosen such that the wavefunction $\psi$ is
/// given by $\psi(n,l,r) = R\_n Y\_{lm}$.
///
/// Binds the [`gsl_sf_hydrogenicR_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_hydrogenicR_e) function.
#[allow(non_snake_case)]
pub fn hydrogenicR(n: i32, l: i32, z: f64, r: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_hydrogenicR_e(n, l, z, r, &mut result))?;
        Ok(result.into())
    }
}

#[allow(non_snake_case)]
pub fn coulomb_CL(l: f64, eta: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_coulomb_CL_e(l, eta, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::special::special_function_test::*;
    
    #[test]
    #[allow(non_snake_case)]
    fn test_hydrogenicR() {
        disable_error_handler();

        check_result(
            hydrogenicR(4, 1, 3.0, 0.0),
            0.0,
            TEST_TOL0
        );

        check_result(
            hydrogenicR(4, 0, 3.0, 2.0),
            -0.03623182256981820062,
            TEST_TOL2
        );

        check_result(
            hydrogenicR(4, 1, 3.0, 2.0),
            -0.028065049083129581005,
            TEST_TOL2
        );

        check_result(
            hydrogenicR(4, 2, 3.0, 2.0),
            0.14583027278668431009,
            TEST_TOL0
        );

        check_result(
            hydrogenicR(100, 0, 3.0, 2.0),
            -0.00007938950980052281367,
            TEST_TOL3
        );

        check_result(
            hydrogenicR(100, 10, 3.0, 2.0),
            7.112823375353605977e-12,
            TEST_TOL2
        );

        check_result(
            hydrogenicR(100, 90, 3.0, 2.0),
            5.845231751418131548e-245,
            TEST_TOL2
        );
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_hydrogenicR_1() {
        disable_error_handler();

        check_result(
            hydrogenicR_1(3.0, 2.0),
            0.025759948256148471036,
            TEST_TOL0
        );

        check_result(
            hydrogenicR_1(3.0, 10.0),
            9.724727052062819704e-13,
            TEST_TOL1
        );
    }
}
