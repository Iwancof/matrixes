pub mod add;
pub mod lu;
pub mod mul;

use super::AsMatrix;

use num_traits::{One, Zero};

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneralMatrix<const H: usize, const W: usize, Inner> {
    inner: [[Inner; H]; W],
}

impl<const H: usize, const W: usize, Inner> AsMatrix<H, W, Inner> for GeneralMatrix<H, W, Inner> {
    fn at(&self, row: usize, col: usize) -> &Inner {
        &self.inner[col][row]
    }
}

impl<const H: usize, const W: usize, Inner> Zero for GeneralMatrix<H, W, Inner>
where
    Inner: Zero + Clone,
{
    fn zero() -> Self {
        use array_macro::array;

        Self {
            inner: array![_ => array![_ => Inner::zero(); H]; W],
        }
    }

    fn is_zero(&self) -> bool {
        self.inner
            .iter()
            .all(|row| row.iter().all(|elem| elem.is_zero()))
    }
}

impl<const H: usize, const W: usize, Inner> Display for GeneralMatrix<H, W, Inner>
where
    Inner: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(width) = f.width() {
            for h in 0..H {
                for w in 0..W {
                    // write!(f, "{1:.*}", precision, self.inner[w][h])?;
                    write!(f, "{:width$}", self.inner[w][h], width = width)?;
                }
                writeln!(f)?;
            }
        } else {
            for h in 0..H {
                for w in 0..W {
                    write!(f, "{}", self.inner[w][h])?;
                    // write!(f, self.inner[w][h])?;
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<const H: usize, const W: usize, Inner> GeneralMatrix<H, W, Inner> {
    #[inline]
    #[allow(unused)]
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Inner {
        &mut self.inner[col][row]
    }
    #[inline]
    #[allow(unused)]
    pub const fn inner(&self) -> &[[Inner; H]; W] {
        &self.inner
    }
    #[inline]
    #[allow(unused)]
    pub fn inner_mut(&mut self) -> &mut [[Inner; H]; W] {
        &mut self.inner
    }
    #[inline]
    #[allow(unused)]
    pub const fn as_ptr(&self) -> *const Inner {
        self.inner.as_ptr() as *const Inner
    }

    #[inline]
    #[allow(unused)]
    pub fn by(v: Inner) -> Self
    where
        Inner: Clone,
    {
        use array_macro::array;
        Self {
            inner: array![array![v.clone(); H]; W],
        }
    }

    #[inline]
    #[allow(unused)]
    pub fn by_f(f: impl Fn(usize, usize) -> Inner) -> Self {
        use array_macro::array;
        Self {
            inner: array![x => array![y => f(y, x); H]; W],
        }
    }

    #[inline]
    #[allow(unused)]
    pub const fn new_col_major(inner: [[Inner; H]; W]) -> Self {
        Self { inner }
    }

    #[inline]
    #[allow(unused)]
    pub fn new_row_major(v: [[Inner; W]; H]) -> Self {
        use core::mem::{forget, transmute_copy, MaybeUninit};

        let mut inner: MaybeUninit<[[Inner; H]; W]> = MaybeUninit::uninit();
        for (i, row) in v.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                unsafe {
                    let elem = transmute_copy(elem);
                    (inner.as_mut_ptr() as *mut Inner)
                        .add(j * H + i)
                        .write(elem);
                }
            }
        }

        forget(v); // prevent the array from being dropped

        Self {
            inner: unsafe { inner.assume_init() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn col_major() {
        let m = GeneralMatrix::new_col_major([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.at(0, 0), &1);
        assert_eq!(m.at(1, 0), &2);
        assert_eq!(m.at(2, 0), &3);
        assert_eq!(m.at(0, 1), &4);
        assert_eq!(m.at(1, 1), &5);
        assert_eq!(m.at(2, 1), &6);
        assert_eq!(m.at(0, 2), &7);
        assert_eq!(m.at(1, 2), &8);
        assert_eq!(m.at(2, 2), &9);
    }

    #[test]
    fn row_major() {
        let m = GeneralMatrix::new_row_major([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.at(0, 0), &1);
        assert_eq!(m.at(0, 1), &2);
        assert_eq!(m.at(0, 2), &3);
        assert_eq!(m.at(1, 0), &4);
        assert_eq!(m.at(1, 1), &5);
        assert_eq!(m.at(1, 2), &6);
        assert_eq!(m.at(2, 0), &7);
        assert_eq!(m.at(2, 1), &8);
        assert_eq!(m.at(2, 2), &9);
    }

    #[test]
    fn row_major_not_regular() {
        let m = GeneralMatrix::new_row_major([[1, 2, 3, 4], [5, 6, 7, 8]]);
        let ans = GeneralMatrix::new_col_major([[1, 5], [2, 6], [3, 7], [4, 8]]);
        assert_eq!(m, ans);
    }

    #[test]
    fn row_major_not_regular_boxed() {
        let m = GeneralMatrix::new_row_major([
            [Box::new(1), Box::new(2), Box::new(3), Box::new(4)],
            [Box::new(5), Box::new(6), Box::new(7), Box::new(8)],
        ]);

        let ans = GeneralMatrix::new_col_major([
            [Box::new(1), Box::new(5)],
            [Box::new(2), Box::new(6)],
            [Box::new(3), Box::new(7)],
            [Box::new(4), Box::new(8)],
        ]);

        assert_eq!(m, ans);

        // if implementation broken, this will print 'free(): double free detected in tcache 2'
    }

    #[test]
    fn row_major_drop() {
        use std::rc::Rc;

        let value_1 = Rc::new(1);
        let value_2 = Rc::new(2);

        let mut m = GeneralMatrix::new_row_major([[value_1, value_2]]);
        assert_eq!(Rc::get_mut(m.at_mut(0, 0)), Some(&mut 1));
        assert_eq!(Rc::get_mut(m.at_mut(0, 1)), Some(&mut 2));

        let weak = Rc::downgrade(m.at(0, 0));
        assert!(weak.upgrade().is_some());

        drop(m);

        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_construct_by() {
        let mat = GeneralMatrix::by(1);
        let ans = GeneralMatrix::new_col_major([[1; 2]; 3]);

        assert_eq!(mat, ans);
    }

    #[test]
    fn test_construct_by_f() {
        let mat = GeneralMatrix::by_f(|x, y| x + y);
        let ans =
            GeneralMatrix::new_row_major([[0, 1, 2, 3], [1, 2, 3, 4], [2, 3, 4, 5], [3, 4, 5, 6]]);

        assert_eq!(mat, ans);
    }
}
