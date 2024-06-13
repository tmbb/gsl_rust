/*
    special/fermi_dirac.rs
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

/// These routines compute the complete Fermi-Dirac integral with an index of $2$,
/// $F_2(x) = (1/2) \int_0^\infty dt (t^2 /(\exp(t-x)+1))$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_2_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_2_e).
pub fn fermi_dirac_2_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_2_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complete Fermi-Dirac integral with an index of $1$,
/// $F_1(x) = \int_0^\infty dt (t /(\exp(t-x)+1))$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_1_e).
pub fn fermi_dirac_1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complete Fermi-Dirac integral with an index of $0$.
/// 
/// This integral is given by $F_0(x) = \ln(1 + e^x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_0_e).
pub fn fermi_dirac_0_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_0_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complete Fermi-Dirac integral $F_{1/2}(x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_half_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_half_e).
pub fn fermi_dirac_half_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_half_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complete Fermi-Dirac integral with an index of $-1$.
/// 
/// This integral is given by $F_{-1}(x) = e^x / (1 + e^x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_m1_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_m1_e).
pub fn fermi_dirac_m1_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_m1_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complete Fermi-Dirac integral $F_{3/2}(x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_3half_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_3half_e).
pub fn fermi_dirac_3half_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_3half_e(x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the incomplete Fermi-Dirac integral with an index of
/// zero, $F_0(x,b) = \ln(1 + e^{b-x}) - (b-x)$
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_inc_0_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_inc_0_e).
pub fn fermi_dirac_inc_0_e(x: f64, b: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_inc_0_e(x, b, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complete Fermi-Dirac integral with an integer index
/// of $j$, $F_j(x) = (1/\Gamma(j+1)) \int_0^\infty dt (t^j /(\exp(t-x)+1))$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_int_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_int_e).
pub fn fermi_dirac_int_e(j: i32, x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_int_e(j, x, &mut result))?;
        Ok(result.into())
    }
}

/// These routines compute the complete Fermi-Dirac integral $F_{-1/2}(x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_mhalf_e`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_mhalf_e).
pub fn fermi_dirac_mhalf_e(x: f64) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::gsl_sf_fermi_dirac_mhalf_e(x, &mut result))?;
        Ok(result.into())
    }
}


/// These routines compute the complete Fermi-Dirac integral with an index of $-1$.
/// 
/// This integral is given by $F_{-1}(x) = e^x / (1 + e^x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_m1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_m1).
pub fn fermi_dirac_m1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_m1(x) }
}

/// These routines compute the complete Fermi-Dirac integral $F_{-1/2}(x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_mhalf`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_mhalf).
pub fn fermi_dirac_mhalf(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_mhalf(x) }
}

/// These routines compute the complete Fermi-Dirac integral with an index of $0$.
/// 
/// This integral is given by $F_0(x) = \ln(1 + e^x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_0).
pub fn fermi_dirac_0(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_0(x) }
}

/// These routines compute the complete Fermi-Dirac integral $F_{3/2}(x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_3half`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_3half).
pub fn fermi_dirac_3half(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_3half(x) }
}

/// These routines compute the complete Fermi-Dirac integral with an integer index
/// of $j$, $F_j(x) = (1/\Gamma(j+1)) \int_0^\infty dt (t^j /(\exp(t-x)+1))$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_int`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_int).
pub fn fermi_dirac_int(j: i32, x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_int(j, x) }
}

/// These routines compute the complete Fermi-Dirac integral $F_{1/2}(x)$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_half`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_half).
pub fn fermi_dirac_half(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_half(x) }
}

/// These routines compute the incomplete Fermi-Dirac integral with an index of
/// zero, $F_0(x,b) = \ln(1 + e^{b-x}) - (b-x)$
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_inc_0`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_inc_0).
pub fn fermi_dirac_inc_0(x: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_inc_0(x, b) }
}

/// These routines compute the complete Fermi-Dirac integral with an index of $1$,
/// $F_1(x) = \int_0^\infty dt (t /(\exp(t-x)+1))$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_1`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_1).
pub fn fermi_dirac_1(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_1(x) }
}

/// These routines compute the complete Fermi-Dirac integral with an index of $2$,
/// $F_2(x) = (1/2) \int_0^\infty dt (t^2 /(\exp(t-x)+1))$.
/// 
/// Binds the function
/// [`gsl_sf_fermi_dirac_2`](https://www.gnu.org/software/gsl/doc/html//specfunc.html#c.gsl_sf_fermi_dirac_2).
pub fn fermi_dirac_2(x: f64) -> f64 {
    unsafe { bindings::gsl_sf_fermi_dirac_2(x) }
}

#[cfg(test)]
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;

    #[test]
    fn test_fermi_dirac_0_e() {
        check_result!(fermi_dirac_0_e(-10.0), 0.00004539889921686464677, TEST_TOL0);
        check_result!(fermi_dirac_0_e(-1.0), 0.31326168751822283405, TEST_TOL0);
        check_result!(fermi_dirac_0_e(1.0), 1.3132616875182228340, TEST_TOL0);
        check_result!(fermi_dirac_0_e(10.0), 10.000045398899216865, TEST_TOL0);
    }

    #[test]
    fn test_fermi_dirac_1_e() {
        check_result!(fermi_dirac_1_e(-10.0), 0.00004539941448447633524, TEST_TOL0);
        check_result!(fermi_dirac_1_e(-2.0), 0.13101248471442377127, TEST_TOL0);
        check_result!(fermi_dirac_1_e(-1.0), 0.3386479964034521798, TEST_TOL0);
        check_result!(fermi_dirac_1_e(-0.4), 0.5825520806897909028, TEST_TOL0);
        check_result!(fermi_dirac_1_e(0.4), 1.1423819861584355337, TEST_TOL0);
        check_result!(fermi_dirac_1_e(1.0), 1.8062860704447742567, TEST_TOL0);
        check_result!(fermi_dirac_1_e(1.5), 2.5581520872227806402, TEST_TOL0);
        check_result!(fermi_dirac_1_e(2.5), 4.689474797599761667, TEST_TOL0);
        check_result!(fermi_dirac_1_e(10.0), 51.64488866743374196, TEST_TOL0);
        check_result!(fermi_dirac_1_e(12.0), 73.64492792264531092, TEST_TOL0);
        check_result!(fermi_dirac_1_e(20.0), 201.64493406478707282, TEST_TOL0);
        check_result!(fermi_dirac_1_e(50.0), 1251.6449340668482264, TEST_TOL0);
    }

    #[test]
    fn test_fermi_dirac_2_e() {
        check_result!(fermi_dirac_2_e(-10.0), 0.00004539967212174776662, TEST_TOL0);
        check_result!(fermi_dirac_2_e(-2.0), 0.13313272938565030508, TEST_TOL0);
        check_result!(fermi_dirac_2_e(-1.0), 0.3525648792978077590, TEST_TOL0);
        check_result!(fermi_dirac_2_e(-0.4), 0.6229402647001272120, TEST_TOL0);
        check_result!(fermi_dirac_2_e(0.4), 1.2915805581060844533, TEST_TOL0);
        check_result!(fermi_dirac_2_e(1.0), 2.1641656128127008622, TEST_TOL0);
        check_result!(fermi_dirac_2_e(1.5), 3.247184513920792475, TEST_TOL0);
        check_result!(fermi_dirac_2_e(2.5), 6.797764392735056317, TEST_TOL0);
        check_result!(fermi_dirac_2_e(10.0), 183.11605273482105278, TEST_TOL0);
        check_result!(fermi_dirac_2_e(12.0), 307.73921494638635166, TEST_TOL0);
        check_result!(fermi_dirac_2_e(20.0), 1366.2320146723590157, TEST_TOL0);
        check_result!(fermi_dirac_2_e(50.0), 20915.580036675744655, TEST_TOL0);
        check_result!(fermi_dirac_2_e(200.0), 1.3336623201467029786e+06, TEST_TOL0);
    }

    #[test]
    fn test_fermi_dirac_3half_e() {
        check_result!(fermi_dirac_3half_e(-10.0), 0.00004539956540456176333, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(-2.0), 0.13224678225177236685, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(-1.0), 0.3466747947990574170, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(-0.4), 0.6056120213305040910, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(0.4), 1.2258236403963668282, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(1.0), 2.0022581487784644573, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(1.5), 2.9277494127932173068, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(2.5), 5.768879312210516582, TEST_TOL0);
        check_result!(fermi_dirac_3half_e(10.0), 101.00510084332600020, TEST_TOL2);
        check_result!(fermi_dirac_3half_e(12.0), 156.51518642795728036, TEST_TOL1);
        check_result!(fermi_dirac_3half_e(20.0), 546.5630100657601959, TEST_TOL1);
        check_result!(fermi_dirac_3half_e(50.0), 5332.353566687145552, TEST_TOL1);
    }

    #[test]
    fn test_fermi_dirac_half_e() {
        check_result!(fermi_dirac_half_e(-10.0), 0.00004539920105264132755, TEST_TOL1);
        check_result!(fermi_dirac_half_e(-2.0), 0.12929851332007559106, TEST_TOL0);
        check_result!(fermi_dirac_half_e(-1.0), 0.3277951592607115477, TEST_TOL0);
        check_result!(fermi_dirac_half_e(-0.4), 0.5522452153690688947, TEST_TOL0);
        check_result!(fermi_dirac_half_e(0.4), 1.0386797503389389277, TEST_TOL0);
        check_result!(fermi_dirac_half_e(1.0), 1.5756407761513002308, TEST_TOL0);
        check_result!(fermi_dirac_half_e(1.5), 2.1448608775831140360, TEST_TOL0);
        check_result!(fermi_dirac_half_e(2.5), 3.606975377950373251, TEST_TOL0);
        check_result!(fermi_dirac_half_e(10.0), 24.084656964637653615, TEST_TOL0);
        check_result!(fermi_dirac_half_e(12.0), 31.540203287044242593, TEST_TOL0);
        check_result!(fermi_dirac_half_e(20.0), 67.49151222165892049, TEST_TOL0);
        check_result!(fermi_dirac_half_e(50.0), 266.09281252136259343, TEST_TOL1);
    }

    #[test]
    fn test_fermi_dirac_int_e() {
        check_result!(fermi_dirac_int_e(3, -2.0), 0.1342199155038680215, TEST_TOL0);
        check_result!(fermi_dirac_int_e(3, 0.0), 0.9470328294972459176, TEST_TOL0);
        check_result!(fermi_dirac_int_e(3, 0.1), 1.0414170610956165759, TEST_TOL0);
        check_result!(fermi_dirac_int_e(3, 1.0), 2.3982260822489407070, TEST_TOL0);
        check_result!(fermi_dirac_int_e(3, 3.0), 12.621635313399690724, TEST_TOL1);
        check_result!(fermi_dirac_int_e(3, 100.0), 4.174893231066566793e+06, TEST_TOL1);
        check_result!(fermi_dirac_int_e(3, 500.0), 2.604372285319088354e+09, TEST_TOL1);
        check_result!(fermi_dirac_int_e(5, -2.0), 0.13505242246823676478, TEST_TOL0);
        check_result!(fermi_dirac_int_e(5, 0.0), 0.9855510912974351041, TEST_TOL0);
        check_result!(fermi_dirac_int_e(5, 0.1), 1.0876519750101492782, TEST_TOL0);
        check_result!(fermi_dirac_int_e(5, 1.0), 2.6222337848692390539, TEST_TOL0);
        check_result!(fermi_dirac_int_e(5, 3.0), 17.008801618012113022, TEST_TOL1);
        check_result!(fermi_dirac_int_e(5, 100.0), 1.3957522531334869874e+09, TEST_TOL1);
        check_result!(fermi_dirac_int_e(5, 500.0), 2.1705672808114817955e+13, TEST_TOL2);
        check_result!(fermi_dirac_int_e(7, -2.0), 0.1352641105671255851, TEST_TOL0);
        check_result!(fermi_dirac_int_e(7, 0.0), 0.9962330018526478992, TEST_TOL0);
        check_result!(fermi_dirac_int_e(7, 0.1), 1.1005861815180315485, TEST_TOL0);
        check_result!(fermi_dirac_int_e(7, 1.0), 2.6918878172003129203, TEST_TOL0);
        check_result!(fermi_dirac_int_e(7, 3.0), 19.033338976999367642, TEST_TOL2);
        check_result!(fermi_dirac_int_e(7, 10.0), 5654.530932873610014, TEST_TOL1);
        check_result!(fermi_dirac_int_e(7, 50.0), 1.005005069985066278e+09, TEST_TOL2);
        check_result!(fermi_dirac_int_e(7, 500.0), 9.691690268341569514e+16, TEST_TOL3);
        check_result!(fermi_dirac_int_e(9, -2.0), 0.1353174385330242691, TEST_TOL0);
        check_result!(fermi_dirac_int_e(9, 0.0), 0.9990395075982715656, TEST_TOL0);
        check_result!(fermi_dirac_int_e(9, 0.1), 1.1039997234712941212, TEST_TOL0);
        check_result!(fermi_dirac_int_e(9, 1.0), 2.7113648898129249947, TEST_TOL0);
        check_result!(fermi_dirac_int_e(9, 3.0), 19.768544008138602223, TEST_TOL2);
        check_result!(fermi_dirac_int_e(9, 10.0), 10388.990167312912478, TEST_TOL2);
        check_result!(fermi_dirac_int_e(9, 50.0), 2.85466960802601649e+10, TEST_TOL1);
        check_result!(fermi_dirac_int_e(9, 500.0), 2.69273849842695876e+20, 2.0*TEST_TOL1);
        check_result!(fermi_dirac_int_e(10, -2.0), 0.13532635396712288092, TEST_TOL0);
        check_result!(fermi_dirac_int_e(10, 0.0), 0.9995171434980607541, TEST_TOL0);
        check_result!(fermi_dirac_int_e(10, 0.1), 1.1045818238852612296, TEST_TOL0);
        check_result!(fermi_dirac_int_e(10, 1.0), 2.7147765350346120647, TEST_TOL0);
        check_result!(fermi_dirac_int_e(10, 3.0), 19.917151938411675171, TEST_TOL1);
        check_result!(fermi_dirac_int_e(10, 10.0), 12790.918595516495955, TEST_TOL2);
        check_result!(fermi_dirac_int_e(10, 50.0), 1.3147703201869657654e+11, TEST_TOL2);
        check_result!(fermi_dirac_int_e(10, 500.0), 1.2241331244469204398e+22, TEST_TOL2);
        check_result!(fermi_dirac_int_e(11, -2.0), 0.1353308162894847149, TEST_TOL0);
        check_result!(fermi_dirac_int_e(11, 0.0), 0.9997576851438581909, TEST_TOL0);
        check_result!(fermi_dirac_int_e(11, 0.1), 1.1048751811565850418, TEST_TOL0);
        check_result!(fermi_dirac_int_e(11, 1.0), 2.7165128749007313436, TEST_TOL0);
        check_result!(fermi_dirac_int_e(11, 3.0), 19.997483022044603065, TEST_TOL2);
        check_result!(fermi_dirac_int_e(11, 10.0), 14987.996005901818036, TEST_TOL2);
        check_result!(fermi_dirac_int_e(11, 50.0), 5.558322924078990628e+11, TEST_TOL2);
        check_result!(fermi_dirac_int_e(11, 500.0), 5.101293089606198280e+23, TEST_TOL2);
        check_result!(fermi_dirac_int_e(20, -2.0), 0.13533527450327238373, TEST_TOL0);
        check_result!(fermi_dirac_int_e(20, 0.0), 0.9999995232582155428, TEST_TOL0);
        check_result!(fermi_dirac_int_e(20, 0.1), 1.1051703357941368203, TEST_TOL0);
        check_result!(fermi_dirac_int_e(20, 1.0), 2.7182783069905721654, TEST_TOL0);
        check_result!(fermi_dirac_int_e(20, 3.0), 20.085345296028242734, TEST_TOL2);
        check_result!(fermi_dirac_int_e(20, 10.0), 21898.072920149606475, TEST_TOL2);
        check_result!(fermi_dirac_int_e(20, 50.0), 1.236873256595717618e+16, TEST_TOL2);
        check_result!(fermi_dirac_int_e(20, 500.0), 9.358938204369557277e+36, TEST_TOL2);
    }

    #[test]
    fn test_fermi_dirac_m1_e() {
        check_result!(fermi_dirac_m1_e(-10.0), 0.00004539786870243439450, TEST_TOL0);
        check_result!(fermi_dirac_m1_e(-1.0), 0.26894142136999512075, TEST_TOL0);
        check_result!(fermi_dirac_m1_e(1.0), 0.7310585786300048793, TEST_TOL0);
        check_result!(fermi_dirac_m1_e(10.0), 0.9999546021312975656, TEST_TOL0);
    }

    #[test]
    fn test_fermi_dirac_mhalf_e() {
        check_result!(fermi_dirac_mhalf_e(-10.0), 0.00004539847236080549532, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(-2.0), 0.12366562180120994266, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(-1.0), 0.29402761761145122022, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(-0.4), 0.4631755336886027800, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(0.4), 0.7654084737661656915, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(1.0), 1.0270571254743506890, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(1.5), 1.2493233478527122008, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(2.5), 1.6663128834358313625, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(10.0), 3.552779239536617160, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(12.0), 3.897268231925439359, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(20.0), 5.041018507535328603, TEST_TOL0);
        check_result!(fermi_dirac_mhalf_e(50.0), 7.977530858581869960, TEST_TOL1);
    }
}