#![allow(incomplete_features)]
#![feature(specialization)]

use std::default::Default;

use std::fmt::{Debug, Display};

extern crate openblas_src;

#[derive(Clone)]
struct Matrix<const H: usize, const W: usize, Inner> {
    e: [[Inner; W]; H],
}

macro_rules! create_util_const_matrixes {
    ($type: ty) => {
        impl<const S: usize> Matrix<S, S, $type> {
            fn one() -> Self {
                let mut ret = [[0 as $type; S]; S];
                for i in 0..S {
                    ret[i][i] = 1 as $type;
                }

                Matrix { e: ret }
            }
        }
    };
}

create_util_const_matrixes!(f32);
create_util_const_matrixes!(f64);

#[test]
fn one_matrix_test() {
    let a = Matrix {
        e: [[1., 0.], [0., 1.]],
    };
    let m = Matrix::<2, 2, f32>::one();

    assert_eq!(a, m);
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

impl<const H: usize, const W: usize, Inner> TMatrix for Matrix<H, W, Inner> {
    #[allow(unused)]
    fn get_size(&self) -> (usize, usize) {
        Self::get_size_type()
    }
    fn get_size_type() -> (usize, usize) {
        (H, W)
    }
}

trait TMatrix {
    fn get_size(&self) -> (usize, usize);
    fn is_regular(&self) -> bool {
        self.get_size().0 == self.get_size().1
    }
    fn get_size_type() -> (usize, usize);
}

#[derive(Debug, Clone, Copy)]
enum InverseError {
    Singular,
}

trait TRegularMatrix: TMatrix + Sized {
    unsafe fn inv_into_ptr(&self, target: *mut Self) -> Option<InverseError>;
    fn inv_into_ref(&self, target: &mut Self) -> Option<InverseError> {
        unsafe { self.inv_into_ptr(target as *mut Self) }
    }
    fn inv(&self) -> Result<Self, InverseError> {
        use std::mem::MaybeUninit;

        let mut target = MaybeUninit::<Self>::uninit();
        unsafe {
            let ret = self.inv_into_ptr(target.as_mut_ptr());

            match ret {
                None => Ok(target.assume_init()),
                Some(err) => Err(err),
            }
        }
    }
}

impl<const S: usize, Inner> TRegularMatrix for Matrix<S, S, Inner>
where
    Inner: Copy,
{
    default unsafe fn inv_into_ptr(&self, target: *mut Self) -> Option<InverseError> {
        todo!()
    }
}

macro_rules! create_inverse_trait_implementation {
    ($f: ident, $i: ident, $type: ty) => {
        impl<const S: usize> TRegularMatrix for Matrix<S, S, $type> {
            unsafe fn inv_into_ptr(&self, target: *mut Self) -> Option<InverseError> {
                #[link(name = "lapack")]
                extern "C" {
                    fn $f(
                        // fn sgetrf_(
                        m: *const i32,
                        n: *const i32,
                        A: *mut f32,
                        lda: *mut i32,
                        ipiv: *mut i32,
                        info: *mut i32,
                    );

                    fn $i(
                        n: *const i32,
                        A: *mut f32,
                        lda: *mut i32,
                        ipiv: *mut i32,
                        work: *mut f32,
                        lwork: *mut i32,
                        info: *mut i32,
                    );
                }

                *target = self.clone();

                let m: *const i32 = &(S as i32);
                let n: *const i32 = &(S as i32);
                let a: *mut f32 = (*target).e.as_mut_ptr() as *mut f32;
                let lda: *mut i32 = &mut (S as i32);
                let ipiv: *mut i32 = [0; S].as_mut_ptr() as *mut i32;
                let info: *mut i32 = &mut 0;

                $f(m, n, a, lda, ipiv, info);
                if *info != 0 {
                    return match *info {
                        _ => Some(InverseError::Singular),
                    };
                }

                let work: *mut f32 = [0.0; S].as_mut_ptr() as *mut f32;
                let lwork: *mut i32 = &mut (S as i32);

                $i(n, a, lda, ipiv, work, lwork, info);
                if *info != 0 {
                    return match *info {
                        _ => Some(InverseError::Singular),
                    };
                }

                None
            }
        }
    };
}

create_inverse_trait_implementation!(sgetrf_, sgetri_, f32);
create_inverse_trait_implementation!(dgetrf_, dgetri_, f64);

#[test]
fn test_inverse() {
    let m = Matrix::<2, 2, f32> {
        e: [[4., 2.], [1., 3.]],
    };
    let ans = Matrix::<2, 2, f32> {
        e: [[0.3, -0.2], [-0.1, 0.4]],
    };

    assert_eq!(m.inv().unwrap(), ans);
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
    default fn mul(self, lhs: Matrix<LWRH, RW, InnerRight>) -> Self::Output {
        let mut ret = Matrix::default();

        for h in 0..ret.get_size().0 {
            for w in 0..ret.get_size().1 {
                for index in 0..LWRH {
                    ret.e[h][w] += self.e[h][index].clone() * lhs.e[index][w].clone();
                }
            }
        }

        // println!("Call default");

        ret
    }
}

macro_rules! create_mul_trait_implementation {
    ($f: ident, $type: ty) => {
        impl<const LH: usize, const LWRH: usize, const RW: usize> Mul<Matrix<LWRH, RW, $type>>
            for Matrix<LH, LWRH, $type>
        {
            // Matrix(LH*LW) * Matrix(LW*RW)
            #[inline(never)]
            fn mul(self, lhs: Matrix<LWRH, RW, $type>) -> Self::Output {
                extern "C" {
                    fn $f(
                        // C = alpha * A * B + beta * C
                        trans_mode_a: *const i8,
                        trans_mode_b: *const i8,
                        height_of_a: *const i32,
                        width_of_b: *const i32,
                        widht_of_a_height_of_b: *const i32,
                        alpha: *const $type,
                        a: *const $type,
                        leading_a: *const i32,
                        b: *const $type,
                        leading_b: *const i32,
                        beta: *const $type,
                        c: *mut $type,
                        leading_c: *const i32,
                    );
                }
                use core::mem::MaybeUninit;

                let mut ret = MaybeUninit::<[[$type; RW]; LH]>::uninit();

                let (ah, aw) = self.get_size();
                let (bh, bw) = lhs.get_size();
                let (ch, cw) = Self::Output::get_size_type();

                let trans_mode_a: *const i8 = &('N' as i8);
                let trans_mode_b: *const i8 = &('N' as i8);
                let height_of_a: *const i32 = &(ah as i32);
                let width_of_b: *const i32 = &(bw as i32);
                let widht_of_a_height_of_b: *const i32 = &(aw as i32); // aw == bh
                let alpha: *const $type = &1.0;
                let a: *const $type = self.e.as_ptr() as *const _;
                let leading_a: *const i32 = &(ah as i32);
                let b: *const $type = self.e.as_ptr() as *const _;
                let leading_b: *const i32 = &(bh as i32);
                let beta: *const $type = &0.0;
                let c: *mut $type = ret.as_mut_ptr() as *mut _;
                let leading_c: *const i32 = &(ch as i32);

                unsafe {
                    $f(
                        trans_mode_a,
                        trans_mode_b,
                        height_of_a,
                        width_of_b,
                        widht_of_a_height_of_b,
                        alpha,
                        a,
                        leading_a,
                        b,
                        leading_b,
                        beta,
                        c,
                        leading_c,
                    );
                }

                unsafe {
                    Matrix {
                        e: ret.assume_init(),
                    }
                }
            }
        }
    };
}

create_mul_trait_implementation!(sgemm_, f32);
create_mul_trait_implementation!(dgemm_, f64);

#[test]
fn test_matrix_multiple() {
    let l = Matrix::<2, 3, f32> {
        e: [[1., 2., 3.], [4., 5., 6.]],
    };
    let r = Matrix::<3, 2, f32> {
        e: [[1., 2.], [3., 4.], [5., 6.]],
    };

    let expect = Matrix::<2, 2, f32> {
        e: [[22., 28.], [49., 64.]],
    };

    assert_eq!(l * r, expect);
}

fn main() {
    let l = Matrix {
        e: [[1., 2.], [4., 5.]],
    };
    let r = Matrix {
        e: [[3., 7.], [2., 4.]],
    };

    println!("{:?}", l * r);
    /*
    // let mut target = Matrix::default();

    // l.inv_into_ref(&mut target);

    // let target = l.inv().unwrap();

    println!("{:?}", target);
    println!("{:?}", l * target);
    */
}
