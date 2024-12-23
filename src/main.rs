extern crate blas_src;
extern crate ndarray;
extern crate ndarray_linalg;

use ndarray::prelude::*;
use ndarray_linalg::generate::random;
use ndarray_linalg::solve::Determinant;

fn main() {
    let arr: Array2<f64> = random((3, 3));
    let arr2: Array2<f64> = random((3, 3));
    let dd = arr.dot(&arr2);
    println!("{:?}", dd.det());
}
