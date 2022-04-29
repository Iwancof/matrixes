pub mod lu;

use std::default::Default;

#[repr(C)]
#[derive(Clone)]
pub struct Matrix<const H: usize, const W: usize, Inner> {
    // e: [[Inner; W]; H],
    e: [[Inner; H]; W],
}

pub trait TMatrix {
    fn get_size(&self) -> (usize, usize);
    fn is_regular(&self) -> bool {
        self.get_size().0 == self.get_size().1
    }
    fn get_size_type() -> (usize, usize);
}

impl<const H: usize, const W: usize, Inner> Matrix<H, W, Inner> {
    #[inline(always)]
    #[allow(unused)]
    pub const fn at(&self, row: usize, col: usize) -> &Inner {
        &self.e[col][row]
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Inner {
        &mut self.e[col][row]
    }
    #[inline(always)]
    #[allow(unused)]
    pub const fn as_ptr(&self) -> *const Inner {
        &self.e[0][0] as *const _
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn as_mut_ptr(&mut self) -> *mut Inner {
        &mut self.e[0][0] as *mut _
    }
    #[inline(always)]
    #[allow(unused)]
    pub const fn by(v: Inner) -> Self
    where
        Inner: Copy,
    {
        Self { e: [[v; H]; W] }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn by_f(f: impl Fn(usize, usize) -> Inner) -> Self {
        use core::mem::MaybeUninit;
        let mut e = MaybeUninit::<[[Inner; H]; W]>::uninit();
        for i in 0..W {
            for j in 0..H {
                unsafe {
                    (*e.as_mut_ptr())[i][j] = f(j, i);
                }
            }
        }
        unsafe { Self { e: e.assume_init() } }
    }
    #[inline(always)]
    #[allow(unused)]
    pub const fn new(e: [[Inner; H]; W]) -> Self {
        Self { e }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn get(self) -> [[Inner; H]; W] {
        self.e
    }
}

impl<const H: usize, const W: usize, Inner> Default for Matrix<H, W, Inner>
where
    Inner: Default + Copy,
{
    fn default() -> Self {
        Self::by(Inner::default())
    }
}

impl<const H: usize, const W: usize, Inner> TMatrix for Matrix<H, W, Inner> {
    #[allow(unused)]
    fn get_size(&self) -> (usize, usize) {
        Self::get_size_type()
    }
    fn get_size_type() -> (usize, usize) {
        (H, W)
    }
}

macro_rules! create_util_const_matrixes {
    ($type: ty) => {
        impl<const S: usize> Matrix<S, S, $type> {
            #[inline(always)]
            #[allow(unused)]
            pub fn one() -> Self {
                Self::by_f(|col, row| if col == row { 1 as $type } else { 0 as $type })
            }
        }
    };
}

create_util_const_matrixes!(i32);
create_util_const_matrixes!(i64);
create_util_const_matrixes!(f32);
create_util_const_matrixes!(f64);

use std::fmt::{Debug, Display};

impl<const H: usize, const W: usize, Inner> Display for Matrix<H, W, Inner>
where
    Inner: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let (h, w) = self.get_size();
        for y in 0..h {
            for x in 0..w {
                write!(f, "{:5}, ", self.at(y, x))?;
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
                write!(f, "{:5?}, ", self.at(y, x))?;
            }
            write!(f, "], ")?;
        }

        Ok(())
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
                if self.at(h, w) != lhs.at(h, w) {
                    return false;
                }
            }
        }
        true
    }
}

mod test {
    use super::*;

    #[test]
    fn at_test() {
        let mut m = Matrix::new([[1, 2], [3, 4]]);
        // 1 3
        // 2 4
        assert_eq!(m.at(0, 0), &1);
        assert_eq!(m.at(1, 0), &2);
        assert_eq!(m.at(0, 1), &3);
        assert_eq!(m.at(1, 1), &4);

        *m.at_mut(1, 1) += 100;

        assert_eq!(m, Matrix::new([[1, 2], [3, 104]]));
    }

    #[test]
    fn as_any_ptr() {
        let mut m = Matrix::new([[1, 2], [3, 4]]);
        let mptr = m.as_ptr();

        let mut array = [1, 2, 3, 4];
        let aptr = array.as_ptr();

        for i in 0..4 {
            unsafe {
                assert_eq!(*mptr.add(i), *aptr.add(i));
            }
        }

        // remove mptr and aptr.

        let mmut_ptr = m.as_mut_ptr();
        unsafe { *mmut_ptr.add(3) += 100 };
        array[3] += 100;

        let mptr = m.as_ptr();
        let aptr = array.as_ptr();

        for i in 0..4 {
            unsafe {
                assert_eq!(*mptr.add(i), *aptr.add(i));
            }
        }
    }

    #[test]
    fn test_by() {
        let m = Matrix::by(3);

        assert_eq!(m, Matrix::new([[3, 3], [3, 3]]));
    }

    #[test]
    fn test_by_f() {
        let m = Matrix::by_f(|col, row| col * 100 + row);
        let ans = Matrix::new([[0, 100], [1, 101]]);

        assert_eq!(m, ans);
    }

    #[test]
    fn one_matrix_test() {
        let a = Matrix::new([[1., 0.], [0., 1.]]);
        let m = Matrix::<2, 2, f32>::one();

        assert_eq!(a, m);
    }
}
