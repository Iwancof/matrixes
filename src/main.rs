#![allow(incomplete_features)]
#![feature(new_uninit)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]
#![feature(box_syntax)]

pub mod factorizations;
pub mod matrix;

use matrix::tridiagonal::TridiagonalMatrix;
use matrix::AsMatrix;

fn main() {
    let m = TridiagonalMatrix::new([1, 2, 3], [4, 5, 6, 7], [8, 9, 10]);

    println!("{:10}", m);
}
