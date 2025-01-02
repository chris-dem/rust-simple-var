extern crate blas_src;
extern crate ndarray;
extern crate ndarray_linalg;

use ndarray::prelude::*;
use ndarray_linalg::DeterminantC;
use rand::prelude::*;
use rand::rngs::SmallRng;
use ndarray_linalg::generate::random_using;
use std::time::Instant;

fn foo<T: RngCore>(rng : &mut T) -> Option<f64> {
    let n = 1000;
    let arr: Array2<f64> = random_using((n, n), rng);
    let arr2: Array2<f64> = random_using((n, n), rng);
    let d = arr.dot(&arr2);
    d.detc().ok()
}

fn main() {
    let sims = 100;
    let mut rng = SmallRng::seed_from_u64(24);
    let start = Instant::now();
    for _ in 0..sims {
        foo(&mut rng);
    }
    let duration = start.elapsed();
    println!("{}", duration.as_secs_f64());
}
