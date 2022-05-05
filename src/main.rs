#![allow(incomplete_features)]
#![feature(new_uninit)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]
#![feature(box_syntax)]

pub mod factorizations;
pub mod matrix;

use matrix::general::lu::{self, GeneralLuError, GeneralLuFormat, GeneralLuPivot};
use matrix::general::GeneralMatrix;
use matrix::AsMatrix;

fn main() {
    use factorizations::lu::{AsLu, LuFormat};
    use rand::Rng;
    use std::time::Instant;

    let mut rng = rand::thread_rng();

    const SIZE: usize = 100;
    let mut inner: Box<[[f64; SIZE]; SIZE]> = box [[0.0; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            inner[i][j] = rng.gen();
        }
    }
    let x = GeneralMatrix::new_col_major_box(inner);
    println!("Generated");

    let start = Instant::now();
    let result = x.lu();
    let duration = start.elapsed();

    println!("{:?}", duration);

    match result {
        Ok((l, u)) => {
            // println!("Lu: {:?}", l);
            println!("{:?}", l.pivot);
            println!("Error: {}", u);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
