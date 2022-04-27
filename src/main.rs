use std::default::Default;

use std::fmt::{Debug, Display};

#[derive(Clone)]
struct Matrix<const H: usize, const W: usize, Inner> {
    e: [[Inner; W]; H],
}

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

impl<const H: usize, const W: usize, Inner> Matrix<H, W, Inner> {
    #[allow(unused)]
    fn get_size(&self) -> (usize, usize) {
        (H, W)
    }

    #[allow(unused)]
    fn is_regular(&self) -> bool {
        H == W
    }
}

impl<const S: usize, Inner> Matrix<S, S, Inner>
where
    Inner: Copy,
{
    fn inv_ref(&self, target: &mut Self) {
        for h in 0..S {
            for w in 0..S {
                target.e[h][w] = self.e[h][w];
            }
        }
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
        // TODO: replace with lapack.
        let mut output = Matrix::default();
        for h in 0..H {
            for w in 0..W {
                output.e[h][w] = self.e[h][w].clone() + lhs.e[h][w].clone();
            }
        }

        output
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
        // TODO: replace with lapack.
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
