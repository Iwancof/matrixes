pub mod add;
pub mod lu;
pub mod mul;

use super::{print_matrix_display, AsMatrix};

use num_traits::{One, Zero};
use once_cell::sync::Lazy;

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct GeneralMatrix<const H: usize, const W: usize, Inner>
where
    Inner: Clone,
{
    #[cfg(feature = "on_heap")]
    inner: Box<[[Inner; H]; W]>,

    #[cfg(not(feature = "on_heap"))]
    inner: [[Inner; H]; W],
}

impl<const H: usize, const W: usize, Inner> AsMatrix<H, W, Inner> for GeneralMatrix<H, W, Inner>
where
    Inner: Clone,
{
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
            #[cfg(feature = "on_heap")]
            inner: box array![_ => array![_ => Inner::zero(); H]; W],

            #[cfg(not(feature = "on_heap"))]
            inner: array![_ => array![_ => Inner::zero(); H]; W],
        }
    }

    fn is_zero(&self) -> bool {
        self.inner
            .iter()
            .all(|row| row.iter().all(|elem| elem.is_zero()))
    }
}

impl<const S: usize, Inner> One for GeneralMatrix<S, S, Inner>
where
    Inner: Zero + One + Clone + AddAssign + Copy,
{
    fn one() -> Self {
        use array_macro::array;

        Self {
            #[cfg(feature = "on_heap")]
            inner: box array![x => array![y => if x == y { Inner::one() } else { Inner::zero() }; S]; S],

            #[cfg(not(feature = "on_heap"))]
            inner: array![x => array![y => if x == y { Inner::one() } else { Inner::zero() }; S], S],
        }
    }
}

impl<const H: usize, const W: usize, Inner> Display for GeneralMatrix<H, W, Inner>
where
    Inner: Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        print_matrix_display(self, f)
    }
}

impl<const H: usize, const W: usize, Inner> GeneralMatrix<H, W, Inner>
where
    Inner: Clone,
{
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
    pub fn by(v: Inner) -> Self {
        use array_macro::array;
        Self {
            #[cfg(feature = "on_heap")]
            inner: box array![_ => array![_ => v.clone(); H]; W],

            #[cfg(not(feature = "on_heap"))]
            inner: array![array![v.clone(); H]; W],
        }
    }

    #[inline]
    #[allow(unused)]
    pub fn by_f(f: impl Fn(usize, usize) -> Inner) -> Self {
        #[cfg(feature = "on_heap")]
        {
            let mut inner = Box::<[[Inner; H]; W]>::new_uninit();
            let ptr = inner.as_mut_ptr() as *mut Inner;
            for h in 0..H {
                for w in 0..W {
                    unsafe {
                        // (*inner.as_mut_ptr())[w][h] = f(w, h);
                        ptr.add(w * H + h).write(f(w, h));
                    }
                }
            }
            Self {
                inner: unsafe { inner.assume_init() },
            }
        }
        #[cfg(not(feature = "on_heap"))]
        {
            use array_macro::array;
            Self {
                inner: array![x => array![y => f(x, y); H]; W],
            }
        }
    }
}

#[cfg(feature = "on_heap")]
impl<const H: usize, const W: usize, Inner> GeneralMatrix<H, W, Inner>
where
    Inner: Clone,
{
    #[inline]
    #[allow(unused)]
    pub fn new_col_major(inner: [[Inner; H]; W]) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    #[inline]
    #[allow(unused)]
    pub const fn new_col_major_box(inner: Box<[[Inner; H]; W]>) -> Self {
        Self { inner }
    }

    #[inline]
    #[allow(unused)]
    pub fn new_row_major(v: [[Inner; W]; H]) -> Self {
        use core::mem::MaybeUninit;

        let mut heap: Box<MaybeUninit<[[Inner; H]; W]>> = Box::new_uninit();
        let ptr = heap.as_mut_ptr() as *mut Inner;

        for h in 0..H {
            for w in 0..W {
                unsafe {
                    ptr.add(w * H + h).write(v[h][w].clone());
                }
            }
        }

        Self {
            inner: unsafe { heap.assume_init() },
        }
    }

    #[inline]
    #[allow(unused)]
    pub fn new_row_major_box(v: Box<[[Inner; W]; H]>) -> Self {
        use core::mem::{forget, transmute_copy, MaybeUninit};

        let mut heap: Box<MaybeUninit<[[Inner; H]; W]>> = Box::new_uninit();
        let ptr = heap.as_mut_ptr() as *mut Inner;

        for h in 0..H {
            for w in 0..W {
                unsafe {
                    ptr.add(w * H + h).write(v[h][w].clone());
                }
            }
        }

        Self {
            inner: unsafe { heap.assume_init() },
        }
    }
}

#[cfg(not(feature = "on_heap"))]
impl<const H: usize, const W: usize, Inner> GeneralMatrix<H, W, Inner>
where
    Inner: Clone,
{
    #[inline]
    #[allow(unused)]
    pub const fn new_col_major(inner: [[Inner; H]; W]) -> Self {
        Self { inner }
    }

    #[inline]
    #[allow(unused)]
    pub fn new_row_major(v: [[Inner; W]; H]) -> Self {
        println!("todo! reimplement");
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
    fn row_major_only() {
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
    fn row_major_not_regular_stack() {
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
        let mat = GeneralMatrix::by_f(|x, y| x + 2 * y);
        let ans =
            GeneralMatrix::new_row_major([[0, 1, 2, 3], [2, 3, 4, 5], [4, 5, 6, 7], [6, 7, 8, 9]]);

        assert_eq!(mat, ans);
    }

    #[test]
    fn test_zero() {
        let x = GeneralMatrix::zero();
        let ans = GeneralMatrix::new_row_major([[0; 4]; 4]);

        assert_eq!(x, ans);
    }

    #[test]
    fn test_one() {
        let x = GeneralMatrix::one();
        let ans = GeneralMatrix::<4, 4, i32>::by_f(|x, y| if x == y { 1 } else { 0 });

        assert_eq!(x, ans);
    }
}
