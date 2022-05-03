#![allow(incomplete_features)]
#![feature(new_uninit)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]

pub mod factorizations;
pub mod matrix;

use matrix::general::GeneralMatrix;

fn main() {
    use factorizations::lu::AsLu;

    let m2 = GeneralMatrix::new_row_major([[10., 10., 10.], [10., 10., 10.]]);
    m2.lu();
}
