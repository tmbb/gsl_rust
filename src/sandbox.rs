// #[cfg(test)]
// mod test {
//     use crate::rng;
//     use crate::distribution;
//     // use crate::test_helpers;
//     use crate::bindings;
//     use ndarray::{Array, Ix1, Ix2};

//     #[test]
//     fn test_dummy2() {
//         let mut r: rng::Rng = rng::default_rng();
//         let n: usize = 2000;
//         let result: Array<f64, Ix1> = Array::from_iter(
//             (0..n).map(|_i| distribution::gamma_knuth(&mut r, 1.0, 2.0))
//         );

//         panic!("{:?}", result)
//     }

//     #[test]
//     fn test_dummy() {
//         let alpha: Vec<f64> = vec![13.0, 24.0, 56.0, 79.0, 90.0, 33.0, 44.0, 12.0, 2.0, 2.0];
//         let k: usize = 10;
//         let n: usize = 2000;
//         let mut r: rng::Rng = rng::default_rng();
//         let mut u: Array<f64, Ix2> = Array::zeros((n, k));

//         for i in 0..n {
//             unsafe {
//                 bindings::gsl_ran_dirichlet(
//                     &mut r,
//                     k as u64,
//                     alpha.as_ptr(),
//                     u.get_mut_ptr((i, 0)).unwrap()
//                 )
//             }
//         }

//         panic!("{:?}", u);
//     }
// }