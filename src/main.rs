#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

#[macro_use]
mod solve;

mod matrix;
mod opes;
mod regular;

extern crate openblas_src;

use matrix::{Matrix, TMatrix};

fn main() {
    let a = Matrix::new([[3., 2., 1.], [4., 5., 3.], [1., 6., 3.]]);
    let b = Matrix::new([[48., 56., 32.], [57., 59., 34.]]);
    let ans = Matrix::new([[5., 8., 1.], [7., 9., 0.]]);

    let x = solve!(a times x = b).unwrap();
    println!("{}", x);
}
