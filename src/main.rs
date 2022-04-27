use std::default::Default;
use std::fmt::Display;

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

use std::ops::Add;
impl<const H: usize, const W: usize, Inner> Add for Matrix<H, W, Inner>
where
    Inner: Copy + Add,
    <Inner as Add>::Output: Copy + Default,
{
    type Output = Matrix<H, W, <Inner as Add>::Output>;
    fn add(self, lhs: Self) -> Self::Output {
        // TODO: replace with lapack.
        let mut output = Matrix::default();
        for h in 0..H {
            for w in 0..W {
                output.e[w][h] = self.e[w][h] + lhs.e[w][h];
            }
        }

        output
    }
}

/*
use std::ops::Mul;
impl<const LH: usize, const LH: usize, const RH: usize, Inner> Mul for Matrix<LW, LH, Inner> {
    // Matrix(LW*LH) * Matrix(LH*RH)
}
*/

fn main() {
    let n = Matrix {
        e: [[10, 20, 30], [40, 50, 60]],
    };

    println!("{}", n);
}
