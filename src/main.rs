#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(box_into_inner)]

#[macro_use]
mod solve;

mod matrix;
mod opes;

extern crate openblas_src;

const SIZE: usize = 500;

// use matrix::lu::TFactorizeLU;
use matrix::regular::lu::TFactorizeLU;
use matrix::{regular::TRegularMatrix, Matrix, TMatrix};

fn main() {
    use rand::Rng;
    use std::time::Instant;

    let mut rng = rand::thread_rng();

    let mut inner = Box::new([[0.0 as f32; SIZE]; SIZE]);
    for i in 0..SIZE {
        for j in 0..SIZE {
            inner[i][j] = rng.gen();
        }
    }
    let a = Box::new(Matrix::new(Box::into_inner(inner)));

    let start = Instant::now();
    let inv = a.inv().unwrap();
    let ans = Box::into_inner(a) * inv;
    let end = start.elapsed();
    println!("{}", ans);

    println!("{}", end.as_secs_f64());
    // println!("{}", d);

    // let x = solve!(a times ? = b).unwrap();
    // println!("{}", x);
}
