#![allow(incomplete_features)]
#![feature(specialization)]

use std::default::Default;

use std::fmt::{Debug, Display};

extern crate openblas_src;

#[derive(Clone)]
struct Matrix<const H: usize, const W: usize, Inner> {
    e: [[Inner; W]; H],
    // e: [Inner; H * W],
}

/*
impl<const H: usize, const W: usize, Inner> Matrix<H, W, Inner> {
    #[inline]
    fn like_matrix(&self) -> &[[Inner; W]; H] {
        let (slice, remain) = self.e.as_chunks::<W>();

        assert_eq!(slice.len(), H);
        slice
    }
}
*/

impl<const H: usize, const W: usize, Inner> Display for Matrix<H, W, Inner>
where
    Inner: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let (h, w) = self.get_size();
        for y in 0..h {
            for x in 0..w {
                write!(f, "{:5}, ", self.e[y][x])?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
impl<const H: usize, const W: usize, Inner> Debug for Matrix<H, W, Inner>
where
    Inner: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let (h, w) = self.get_size();
        for y in 0..h {
            write!(f, "[")?;
            for x in 0..w {
                write!(f, "{:5?}, ", self.e[y][x])?;
            }
            write!(f, "], ")?;
        }

        Ok(())
    }
}

impl<const H: usize, const W: usize, Inner> Default for Matrix<H, W, Inner>
where
    Inner: Default + Copy,
{
    fn default() -> Self {
        Self {
            e: [[Inner::default(); W]; H],
        }
    }
}

impl<const H: usize, const W: usize, Inner> TMatrix for Matrix<H, W, Inner> {
    #[allow(unused)]
    fn get_size(&self) -> (usize, usize) {
        (H, W)
    }
}

trait TMatrix {
    fn get_size(&self) -> (usize, usize);
    fn is_regular(&self) -> bool {
        self.get_size().0 == self.get_size().1
    }
}

trait TRegularMatrix: TMatrix {
    fn inv_into_ref(&self, target: &mut Self) {}
}

impl<const S: usize, Inner> TRegularMatrix for Matrix<S, S, Inner>
where
    Inner: Copy,
{
    default fn inv_into_ref(&self, target: &mut Self) {
        todo!()
    }
}

// TODO: replace Matrix::e to one array. and use it with chunked function.

impl<const S: usize> TRegularMatrix for Matrix<S, S, f32> {
    fn inv_into_ref(&self, target: &mut Self) {
        use lapack::sgetri;
        // sgetri(S as i32,1
    }
}

use std::cmp::PartialEq;
impl<const H: usize, const W: usize, InnerLeft, InnerRight> PartialEq<Matrix<H, W, InnerRight>>
    for Matrix<H, W, InnerLeft>
where
    InnerLeft: PartialEq<InnerRight>,
{
    fn eq(&self, lhs: &Matrix<H, W, InnerRight>) -> bool {
        for h in 0..H {
            for w in 0..W {
                if self.e[h][w] != lhs.e[h][w] {
                    return false;
                }
            }
        }
        true
    }
}
/*
use std::cmp::Eq;
impl<const H: usize, const W: usize, Inner> Eq for Matrix<W, H, Inner> where Inner: Eq {}
*/

use std::ops::Add;
impl<const H: usize, const W: usize, InnerLeft, InnerRight, InnerOut> Add<Matrix<H, W, InnerRight>>
    for Matrix<H, W, InnerLeft>
where
    InnerOut: Default + Copy,
    InnerLeft: Clone + Add<InnerRight, Output = InnerOut>,
    InnerRight: Clone,
{
    type Output = Matrix<H, W, InnerOut>;
    fn add(self, lhs: Matrix<H, W, InnerRight>) -> Self::Output {
        {
            let mut output = Matrix::default();
            for h in 0..H {
                for w in 0..W {
                    output.e[h][w] = self.e[h][w].clone() + lhs.e[h][w].clone();
                }
            }

            output
        }
    }
}

#[test]
fn test_matrix_addition() {
    let left = Matrix {
        e: [[1., 2., 3.], [4., 5., 6.]],
    };
    let right = Matrix {
        e: [[6., 5., 4.], [3., 2., 1.]],
    };
    let expect = Matrix {
        e: [[7., 7., 7.], [7., 7., 7.]],
    };

    assert_eq!(left + right, expect);
}

use std::ops::Sub;
impl<const H: usize, const W: usize, InnerLeft, InnerRight, InnerOut> Sub<Matrix<H, W, InnerRight>>
    for Matrix<H, W, InnerLeft>
where
    InnerOut: Default + Copy,
    InnerLeft: Clone + Sub<InnerRight, Output = InnerOut>,
    InnerRight: Clone,
{
    type Output = Matrix<H, W, InnerOut>;
    fn sub(self, lhs: Matrix<H, W, InnerRight>) -> Self::Output {
        let mut output = Matrix::default();
        for h in 0..H {
            for w in 0..W {
                output.e[h][w] = self.e[h][w].clone() - lhs.e[h][w].clone();
            }
        }

        output
    }
}

use std::ops::{AddAssign, Mul};
impl<const LH: usize, const LWRH: usize, const RW: usize, InnerLeft, InnerRight, InnerOut>
    Mul<Matrix<LWRH, RW, InnerRight>> for Matrix<LH, LWRH, InnerLeft>
where
    InnerOut: Default + Copy + AddAssign,
    InnerLeft: Clone + Mul<InnerRight, Output = InnerOut>,
    InnerRight: Clone,
{
    // Matrix(LH*LW) * Matrix(LW*RW)
    type Output = Matrix<LH, RW, InnerOut>;
    fn mul(self, lhs: Matrix<LWRH, RW, InnerRight>) -> Self::Output {
        let mut ret = Matrix::default();

        for h in 0..ret.get_size().0 {
            for w in 0..ret.get_size().1 {
                for index in 0..LWRH {
                    ret.e[h][w] += self.e[h][index].clone() * lhs.e[index][w].clone();
                }
            }
        }

        ret
    }
}

#[test]
fn test_matrix_multiple() {
    let l = Matrix {
        e: [[1., 2., 3.], [4., 5., 6.]],
    };
    let r = Matrix {
        e: [[1., 2.], [3., 4.], [5., 6.]],
    };

    let expect = Matrix {
        e: [[22., 28.], [49., 64.]],
    };

    assert_eq!(l * r, expect);
}

fn main() {
    let l = Matrix {
        e: [[1., 2., 3.], [4., 5., 6.]],
    };
    let r = Matrix {
        e: [[1., 2.], [3., 4.], [5., 6.]],
    };

    println!("{}", l * r);
}
