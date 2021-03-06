#![allow(incomplete_features)]
#![feature(new_uninit)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]
#![feature(box_syntax)]

pub mod factorizations;
pub mod matrix;

use matrix::general::GeneralMatrix;
use matrix::tridiagonal::TridiagonalMatrix;
use matrix::AsMatrix;
use rand::Rng;

fn main() {
    // let m = TridiagonalMatrix::new([1, 2, 3], [4, 5, 6, 7], [8, 9, 10]);
    use factorizations::lu::AsLu;

    let mut rng = rand::thread_rng();

    const SIZE: usize = 10000;
    let mut inner = Box::new([[0.0; SIZE]; SIZE]);

    for i in 0..SIZE {
        for j in 0..SIZE {
            inner[i][j] = rng.gen();
        }
    }

    let a = GeneralMatrix::new_col_major_box(inner);
    println!("Generated");
    use std::time::Instant;

    let start = Instant::now();
    a.lu();
    let elap = start.elapsed();

    println!("elap: {:?}", elap);
}
