use num_traits::Zero;

use super::{print_matrix_display, AsMatrix};

use core::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "on_heap")]
pub struct TridiagonalMatrix<const S: usize, Inner>
where
    Inner: Clone + Zero,
    [(); S - 1]:,
{
    upper: Box<[Inner; S - 1]>,
    diagonal: Box<[Inner; S]>,
    lower: Box<[Inner; S - 1]>,

    zero: Inner,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg(not(feature = "on_heap"))]
pub struct TridiagonalMatrix<const S: usize, Inner>
where
    Inner: Clone + Zero,
    [Inner; S - 1]:,
{
    upper: [Inner; S - 1],
    diagonal: [Inner; S],
    lower: [Inner; S - 1],

    zero: Inner,
}

impl<const S: usize, Inner> TridiagonalMatrix<S, Inner>
where
    Inner: Clone + Zero,
    [(); S - 1]:,
{
    // static ZERO: Inner = Inner::zero();

    #[inline]
    #[allow(unused)]
    pub const fn at_upper(&self, i: usize) -> &Inner {
        &self.upper[i]
    }

    #[inline]
    #[allow(unused)]
    pub const fn at_diagonal(&self, i: usize) -> &Inner {
        &self.diagonal[i]
    }

    #[inline]
    #[allow(unused)]
    pub const fn at_lower(&self, i: usize) -> &Inner {
        &self.lower[i]
    }

    #[inline]
    #[allow(unused)]
    pub fn at_upper_mut(&mut self, i: usize) -> &mut Inner {
        &mut self.upper[i]
    }

    #[inline]
    #[allow(unused)]
    pub fn at_diagonal_mut(&mut self, i: usize) -> &mut Inner {
        &mut self.diagonal[i]
    }

    #[inline]
    #[allow(unused)]
    pub fn at_lower_mut(&mut self, i: usize) -> &mut Inner {
        &mut self.lower[i]
    }

    #[inline]
    #[allow(unused)]
    pub const fn inner(&self) -> (&[Inner; S - 1], &[Inner; S], &[Inner; S - 1]) {
        (&self.upper, &self.diagonal, &self.lower)
    }

    #[inline]
    #[allow(unused)]
    pub fn inner_mut(&mut self) -> (&mut [Inner; S - 1], &mut [Inner; S], &mut [Inner; S - 1]) {
        (&mut self.upper, &mut self.diagonal, &mut self.lower)
    }

    #[inline]
    #[allow(unused)]
    pub fn by(v: Inner) -> Self {
        use array_macro::array;

        Self {
            #[cfg(feature = "on_heap")]
            upper: box array![v.clone(); S - 1],
            #[cfg(feature = "on_heap")]
            diagonal: box array![v.clone(); S],
            #[cfg(feature = "on_heap")]
            lower: box array![v.clone(); S - 1],

            #[cfg(not(feature = "on_heap"))]
            upper: array![v.clone(); S - 1],
            #[cfg(not(feature = "on_heap"))]
            diagonal: array![v.clone(); S],
            #[cfg(not(feature = "on_heap"))]
            lower: array![v.clone(); S - 1],

            zero: Inner::zero(),
        }
    }

    #[inline]
    #[allow(unused)]
    pub fn by_f(
        fu: impl Fn(usize) -> Inner,
        fd: impl Fn(usize) -> Inner,
        fl: impl Fn(usize) -> Inner,
    ) -> Self {
        use core::mem::MaybeUninit;

        #[cfg(feature = "on_heap")]
        let mut upper = Box::<[Inner; S - 1]>::new_uninit();
        #[cfg(feature = "on_heap")]
        let mut diagonal = Box::<[Inner; S]>::new_uninit();
        #[cfg(feature = "on_heap")]
        let mut lower = Box::<[Inner; S - 1]>::new_uninit();

        #[cfg(not(feature = "on_heap"))]
        let mut upper = MaybeUninit::<[Inner; S - 1]>::uninit();
        #[cfg(not(feature = "on_heap"))]
        let mut diagonal = MaybeUninit::<[Inner; S]>::uninit();
        #[cfg(not(feature = "on_heap"))]
        let mut lower = MaybeUninit::<[Inner; S - 1]>::uninit();

        let (ip, dp, lp) = (
            upper.as_mut_ptr() as *mut Inner,
            diagonal.as_mut_ptr() as *mut Inner,
            lower.as_mut_ptr() as *mut Inner,
        );

        unsafe {
            for i in 0..S - 1 {
                ip.add(i).write(fu(i));
                dp.add(i).write(fd(i));
                lp.add(i).write(fl(i));
            }
            dp.add(S - 1).write(fd(S - 1));
        }

        unsafe {
            #[cfg(feature = "on_heap")]
            {
                Self {
                    upper: upper.assume_init(),
                    diagonal: diagonal.assume_init(),
                    lower: lower.assume_init(),
                    zero: Inner::zero(),
                }
            }

            #[cfg(not(feature = "on_heap"))]
            {
                Self {
                    upper: upper.assume_init(),
                    diagonal: diagonal.assume_init(),
                    lower: lower.assume_init(),
                    zero: Inner::zero(),
                }
            }
        }
    }
}

#[cfg(feature = "on_heap")]
impl<const S: usize, Inner> TridiagonalMatrix<S, Inner>
where
    Inner: Clone + Zero,
    [(); S - 1]:,
{
    #[inline]
    #[allow(unused)]
    pub fn new(upper: [Inner; S - 1], diagonal: [Inner; S], lower: [Inner; S - 1]) -> Self {
        Self {
            upper: Box::new(upper),
            diagonal: Box::new(diagonal),
            lower: Box::new(lower),
            zero: Inner::zero(),
        }
    }

    #[inline]
    #[allow(unused)]
    pub fn new_box(
        upper: Box<[Inner; S - 1]>,
        diagonal: Box<[Inner; S]>,
        lower: Box<[Inner; S - 1]>,
    ) -> Self {
        Self {
            upper,
            diagonal,
            lower,
            zero: Inner::zero(),
        }
    }
}

#[cfg(not(feature = "on_heap"))]
impl<const S: usize, Inner> TridiagonalMatrix<S, Inner>
where
    Inner: Clone + Zero,
    [(); S - 1]:,
{
    #[inline]
    #[allow(unused)]
    pub fn new(upper: [Inner; S - 1], diagonal: [Inner; S], lower: [Inner; S - 1]) -> Self {
        Self {
            upper,
            diagonal,
            lower,
            zero: Inner::zero(),
        }
    }
}

impl<const S: usize, Inner> Display for TridiagonalMatrix<S, Inner>
where
    Inner: Clone + Display + Zero,
    [(); S - 1]:,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        print_matrix_display(self, f)
    }
}

impl<const S: usize, Inner> AsMatrix<S, S, Inner> for TridiagonalMatrix<S, Inner>
where
    Inner: Clone + Zero,
    [(); S - 1]:,
{
    fn at(&self, row: usize, col: usize) -> &Inner {
        if row == col {
            &self.diagonal[row]
        } else if row == col + 1 {
            &self.upper[col]
        } else if row + 1 == col {
            &self.lower[row]
        } else {
            &self.zero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_trait_access() {
        let original = TridiagonalMatrix::new([1., 1., 1.], [1., 1., 1., 1.], [1., 1., 1.]);
        assert_eq!(original.at(0, 0), &1.);
        assert_eq!(original.at(0, 1), &1.);
        assert_eq!(original.at(0, 2), &0.);
        assert_eq!(original.at(0, 3), &0.);

        assert_eq!(original.at(1, 0), &1.);
        assert_eq!(original.at(1, 1), &1.);
        assert_eq!(original.at(1, 2), &1.);
        assert_eq!(original.at(1, 3), &0.);

        assert_eq!(original.at(2, 0), &0.);
        assert_eq!(original.at(2, 1), &1.);
        assert_eq!(original.at(2, 2), &1.);
        assert_eq!(original.at(2, 3), &1.);
    }

    #[test]
    fn test_by() {
        let ans = TridiagonalMatrix::new([1., 1., 1.], [1., 1., 1., 1.], [1., 1., 1.]);
        let m = TridiagonalMatrix::by(1.);

        assert_eq!(ans, m);
    }

    #[test]
    fn test_by_f() {
        let ans = TridiagonalMatrix::new([0., 1., 2.], [0., 2., 4., 6.], [0., 3., 6.]);
        let m = TridiagonalMatrix::by_f(|i| i as f64, |i| (2 * i) as f64, |i| (3 * i) as f64);

        assert_eq!(ans, m);
    }
}
