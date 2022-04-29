#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

#[macro_use]
mod solve;

mod matrix;
mod opes;
mod regular;

extern crate openblas_src;

use matrix::{
    lu::{FactorizeLU, TFactorizeLU},
    Matrix, TMatrix,
};
use regular::TRegularMatrix;

fn main() {}
