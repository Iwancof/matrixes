#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

#[macro_use]
mod solve;

mod matrix;
mod opes;

extern crate openblas_src;

use matrix::lu::TFactorizeLU;
use matrix::{Matrix, TMatrix};

fn main() {
    let a = Matrix::new([[3., 2., 1.], [4., 5., 3.], [1., 6., 3.]])
        .lu()
        .unwrap();
    let b_1 = Matrix::new([[48., 0., 32.], [0., 59., 0.]]);
    let b_2 = Matrix::new([[0., 56., 0.], [57., 0., 34.]]);

    let x = solve!(a times ? = b_1 + b_2).unwrap();
    println!("{}", x);
}
