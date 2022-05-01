pub mod lu;

use super::regular::TRegularMatrix;
use super::{equal_using_at, Matrix, TMatrix};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display};

pub trait TTridiagonalMatrix<const S: usize, Inner>: TRegularMatrix<S, Inner> {}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct TridiagonalMatrix<const S: usize, Inner>
where
    [Inner; S - 1]:,
{
    // inner: Matrix<S, S, Inner>,
    upper: [Inner; S - 1],
    diagonal: [Inner; S],
    lower: [Inner; S - 1],
}

impl<const S: usize, Inner> Debug for TridiagonalMatrix<S, Inner>
where
    [Inner; S - 1]:,
    Inner: Debug + Clone + num_traits::identities::Zero,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (h, w) = self.get_size();
        for y in 0..h {
            write!(f, "[")?;
            for x in 0..w {
                write!(f, "{:5?}, ", self.at(y, x).clone())?;
            }
            write!(f, "], ")?;
        }

        Ok(())
    }
}

impl<const S: usize, Inner> Display for TridiagonalMatrix<S, Inner>
where
    [Inner; S - 1]:,
    Inner: Display + Clone + num_traits::identities::Zero,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (h, w) = self.get_size();
        for y in 0..h {
            for x in 0..w {
                write!(f, "{:5}, ", self.at(y, x).clone())?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl<const S: usize, Inner> TMatrix<S, S, Inner> for TridiagonalMatrix<S, Inner>
where
    [Inner; S - 1]:,
    Inner: Clone + num_traits::identities::Zero,
{
    fn at(&self, row: usize, col: usize) -> Inner {
        if row == col {
            self.diagonal[row].clone()
        } else if row == col + 1 {
            self.lower[col].clone()
        } else if row + 1 == col {
            self.upper[row].clone()
        } else {
            Inner::zero()
        }
    }
}

fn is_tridiagonal<const S: usize, Inner>(m: &Matrix<S, S, Inner>) -> bool
where
    [Inner; S - 1]:,
    Inner: num_traits::identities::Zero + PartialEq + Clone + Copy,
{
    for h in 0..S {
        for w in 0..S {
            if ((h as isize) - (w as isize)).abs() <= 1 {
                continue;
            }
            if m.at(h, w) != Inner::zero() {
                return false;
            }
        }
    }

    true
}

impl<const S: usize, Inner> TryFrom<Matrix<S, S, Inner>> for TridiagonalMatrix<S, Inner>
where
    [Inner; S - 1]:,
    Inner: num_traits::identities::Zero + PartialEq + Clone + Copy,
{
    // TODO: replace `matrix<S, S, Inner>` with `T: TRegularMatrix<S, Inner>`
    // but this implementation make soemthing wrong....

    type Error = Matrix<S, S, Inner>;

    fn try_from(m: Matrix<S, S, Inner>) -> Result<Self, Self::Error> {
        if !is_tridiagonal(&m) {
            return Err(m);
        }

        let mut upper = [Inner::zero(); S - 1];
        let mut diagonal = [Inner::zero(); S];
        let mut lower = [Inner::zero(); S - 1];

        // for cache
        for w in 0..(S) {
            // unlikely
            if w == 0 {
                diagonal[w] = m.at(w, w).clone();
                lower[w] = m.at(w, w + 1).clone();
            } else if w == S - 1 {
                upper[w - 1] = m.at(w, w - 1).clone();
                diagonal[w] = m.at(w, w).clone();
            } else {
                upper[w - 1] = m.at(w, w - 1).clone();
                diagonal[w] = m.at(w, w).clone();
                lower[w] = m.at(w, w + 1).clone();
            }
        }

        Ok(Self {
            upper,
            diagonal,
            lower,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_test_ok_1() {
        let matrix = Matrix::new([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]);

        let tri = TridiagonalMatrix::try_from(matrix.clone()).unwrap();

        assert!(equal_using_at(&tri, &matrix));
    }

    #[test]
    fn from_test_ok_2() {
        let matrix = Matrix::new([[1, 1, 0, 0], [1, 2, 1, 0], [0, 1, 3, 1], [0, 0, 1, 4]]);
        let tri = TridiagonalMatrix::try_from(matrix.clone()).unwrap();

        println!("{}", tri);
        println!("{}", matrix);

        assert!(equal_using_at(&tri, &matrix));
    }

    #[test]
    fn from_test_ng() {
        let matrix = Matrix::new([[1, 1, 1, 0], [1, 2, 1, 1], [1, 1, 3, 1], [0, 1, 1, 4]]);

        let tri = TridiagonalMatrix::try_from(matrix.clone());

        assert_eq!(tri, Err(matrix));
    }
}
