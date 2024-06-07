/*
    special/airy.rs
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
/// These routines compute the location of the `s`-th zero of the Airy
/// function $Ai(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Ai_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Ai_e) function.
#[allow(non_snake_case)]
pub fn airy_zero_Ai(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Ai_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th zero of the Airy
/// function $Bi(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Bi_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Bi_e) function.
#[allow(non_snake_case)]
pub fn airy_zero_Bi(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Bi_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th zero of the Airy
/// function derivative $Ai'(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Ai_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Ai_deriv_e) function.
#[allow(non_snake_case)]
pub fn airy_zero_Ai_deriv(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Ai_deriv_e(s, &mut result))?;
        Ok(result.into())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// <style>p { overflow-y: hidden; }</style>
/// These routines compute the location of the `s`-th zero of the Airy
/// function derivative $Bi'(x)$.
///
/// Binds the [`gsl_sf_airy_zero_Bi_deriv_e`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.gsl_sf_airy_zero_Bi_deriv_e) function.
#[allow(non_snake_case)]
pub fn airy_zero_Bi_deriv(s: u32) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(gsl_sf_airy_zero_Bi_deriv_e(s, &mut result))?;
        Ok(result.into())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::special::special_function_test::*;
    
    #[test]
    #[allow(non_snake_case)]
    fn test_airy_zero_Ai() {
        disable_error_handler();

        check_result(
            airy_zero_Ai(2),
            -4.087949444130970617,
            TEST_TOL0
        );

        check_result(
            airy_zero_Ai(50),
            -38.02100867725525443,
            TEST_TOL0
        );

        check_result(
            airy_zero_Ai(100),
            -60.45555727411669871,
            TEST_TOL0
        );

        check_result(
            airy_zero_Ai(110),
            -64.43135670991324811,
            TEST_TOL0
        );
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_airy_zero_Ai_deriv() {
        disable_error_handler();

        check_result(
            airy_zero_Ai_deriv(2),
            -3.248197582179836561,
            TEST_TOL0
        );

        check_result(
            airy_zero_Ai_deriv(50),
            -37.76565910053887108,
            TEST_TOL0
        );

        check_result(
            airy_zero_Ai_deriv(100),
            -60.25329596442479317,
            TEST_TOL0
        );

        check_result(
            airy_zero_Ai_deriv(110),
            -64.23545617243546956,
            TEST_TOL0
        );

        check_result(
            airy_zero_Ai_deriv(1000),
            -280.9378080358935071,
            TEST_TOL0
        );
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_airy_zero_Bi() {
        disable_error_handler();

        check_result(
            airy_zero_Bi(2),
            -3.271093302836352716,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi(50),
            -37.76583438165180116,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi(100),
            -60.25336482580837088,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi(110),
            -64.2355167606561537,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi(111),
            -64.6268994819519378,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi(200),
            -95.88699147356682665,
            TEST_TOL0
        );
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_airy_zero_Bi_deriv() {
        disable_error_handler();

        check_result(
            airy_zero_Bi_deriv(2),
            -4.073155089071828216,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi_deriv(50),
            -38.02083574095788210,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi_deriv(100),
            -60.45548887257140819,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi_deriv(110),
            -64.43129648944845060,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi_deriv(111),
            -64.82208737584206093,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi_deriv(200),
            -96.04731050310324450,
            TEST_TOL0
        );

        check_result(
            airy_zero_Bi_deriv(1000),
            -281.0315164471118527,
            TEST_TOL0
        );
    }
}
