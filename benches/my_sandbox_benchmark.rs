use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::{Array, Ix2};

use gsl_rust::bindings;
use gsl_rust::rng;

fn dummy() {
    let alpha: Vec<f64> = vec![13.0, 24.0, 56.0, 79.0, 90.0, 33.0, 44.0, 12.0, 2.0, 2.0];
    let k: usize = 10;
    let n: usize = 2000;
    let mut r: rng::Rng = rng::default_rng();
    let mut u: Array<f64, Ix2> = Array::zeros((n, k));

    for i in 0..n {
        unsafe {
            bindings::gsl_ran_dirichlet(
                &mut r,
                k as u64,
                alpha.as_ptr(),
                u.get_mut_ptr((i, 0)).unwrap()
            )
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dummy", |b| b.iter(|| dummy()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);