use crate::{Matrix, TMatrix};

use super::{AXEqBError, TAXEqB};

use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::ops::Mul;

impl<const AH: usize, const AWXH: usize, const XW: usize, InnerA, InnerX, InnerB>
    TAXEqB<AH, AWXH, XW, InnerA, InnerX, InnerB>
    for (Matrix<AH, AWXH, InnerA>, Matrix<AH, XW, InnerB>)
where
    InnerA: Mul<InnerX, Output = InnerB> + Clone,
    InnerX: Clone,
    InnerB: Clone + Debug + Display + PartialEq,
    Matrix<AH, AWXH, InnerA>: Mul<Matrix<AWXH, XW, InnerX>, Output = Matrix<AH, XW, InnerB>>,
{
    type Variable = Matrix<AWXH, XW, InnerX>;
    default unsafe fn solve_into_ptr(&self, v: *mut Self::Variable) -> Option<AXEqBError> {
        let (a, b) = self;

        let a = (*a).clone();
        let b = (*b).clone();
        let v = (*v).clone();

        assert_eq!(a * v, b);

        todo!();
    }
}

macro_rules! create_solve_ax_b_imple {
    ($f: ident, $type: ty) => {
        impl<const AS: usize, const XW: usize> TAXEqB<AS, AS, XW, $type, $type, $type>
            for (Matrix<AS, AS, $type>, Matrix<AS, XW, $type>)
        where
            Matrix<AS, AS, $type>: Mul<Matrix<AS, XW, $type>, Output = Matrix<AS, XW, $type>>,
        {
            default unsafe fn solve_into_ptr(&self, v: *mut Self::Variable) -> Option<AXEqBError> {
                extern "C" {
                    fn $f(
                        n: *const i32,    // integer. order of A.
                        nrhs: *const i32, // integer. width of x
                        a: *mut $type,    // array of $type. matrix A
                        lda: *const i32,  // integer. leading dimension of matrix A
                        ipiv: *mut i32,   // array of i32. N <= length.
                        b: *mut $type,    // array of $type. matrix B
                        ldb: *const i32,  // integer. leading dimension of matrix B
                        info: *mut i32,   // integer. result
                    );
                }

                let (a, b) = self;
                let mut a = (*a).clone();
                *v = (*b).clone();

                let n: *const i32 = &(AS as i32);
                let nrhs: *const i32 = &(XW as i32);
                let a: *mut $type = a.as_mut_ptr();
                let lda: *const i32 = &(AS as i32);
                let mut ipiv: [i32; AS] = [0; AS];
                let b: *mut $type = (*v).as_mut_ptr();
                let ldb: *const i32 = &(AS as i32);
                let info: *mut i32 = &mut 0;

                $f(n, nrhs, a, lda, ipiv.as_mut_ptr(), b, ldb, info);
                if *info != 0 {
                    match *info {
                        -8..=-1 => return Some(AXEqBError::ArgumentError((-*info) as u8)),
                        i => return Some(AXEqBError::Unknown(i)),
                    }
                }

                None
            }
        }
    };
}

create_solve_ax_b_imple!(sgesv_, f32);
create_solve_ax_b_imple!(dgesv_, f64); // TODO: better? dgetsvx

use crate::matrix::lu;
impl<const AH: usize, const AWXH: usize, const XW: usize, InnerA, InnerX, InnerB>
    TAXEqB<AH, AWXH, XW, InnerA, InnerX, InnerB>
    for (lu::FactorizeLU<AH, AWXH, InnerA>, Matrix<AH, XW, InnerB>)
where
    [(); lu::min(AH, AWXH)]:,
{
    type Variable = Matrix<AWXH, XW, InnerX>;
    default unsafe fn solve_into_ptr(&self, _v: *mut Self::Variable) -> Option<AXEqBError> {
        todo!()
    }
}

macro_rules! create_solve_ax_b_imple_by_lu {
    ($f: ident, $type: ty) => {
        impl<const AS: usize, const XW: usize> TAXEqB<AS, AS, XW, $type, $type, $type>
            for (lu::FactorizeLU<AS, AS, $type>, Matrix<AS, XW, $type>)
        where
            [(); lu::min(AS, AS)]:,
        {
            default unsafe fn solve_into_ptr(&self, v: *mut Self::Variable) -> Option<AXEqBError> {
                extern "C" {
                    fn $f(
                        trans: *const u8, // char 'N' or 'T'
                        n: *const i32,    // integer. order of matrix A
                        nrhs: *const i32, // integer. width of matrix B
                        a: *const $type,  // array of $type.
                        lda: *const i32,  // integer. leading dimension of matrix A
                        ipiv: *const i32, // array of i32
                        b: *mut $type,    // array of $type.
                        ldb: *const i32,  //integer. leading dimension of matrix B
                        info: *mut i32,   // integer.
                    );
                }

                let (self_a, self_b) = self;
                *v = self_b.clone();

                let trans: *const u8 = &('N' as u8);
                let n: *const i32 = &(AS as i32);
                let nrhs: *const i32 = &(AS as i32);
                let (a, ipiv): (*const $type, *const i32) = self_a.as_ptr();
                let lda: *const i32 = &(AS as i32);
                let b: *mut $type = (*v).as_mut_ptr();
                let ldb: *const i32 = &(AS as i32);
                let info: *mut i32 = &mut 0;

                $f(trans, n, nrhs, a, lda, ipiv, b, ldb, info);

                if *info != 0 {
                    return match *info {
                        -9..=-1 => Some(AXEqBError::ArgumentError((-*info) as u8)),
                        i => Some(AXEqBError::Unknown(i)),
                    };
                }

                None
            }
        }
    };
}

create_solve_ax_b_imple_by_lu!(sgetrs_, f32);
create_solve_ax_b_imple_by_lu!(dgetrs_, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_ax_b_by_lu_f32() {
        use crate::matrix::lu::FactorizeLU;
        use float_cmp::{ApproxEq, F32Margin};

        use lu::TFactorizeLU;

        let a = Matrix::new([[3., 2., 1.], [4., 5., 3.], [1., 6., 3.]]);
        let b = Matrix::new([[48., 56., 32.], [57., 59., 34.]]);
        let ans = Matrix::new([[5., 8., 1.], [7., 9., 0.]]);

        let lu: FactorizeLU<3, 3, f32> = a.lu().unwrap();

        let s = (lu, b).solve().unwrap();

        println!("{}", ans);
        println!("{}", s);

        assert!(ans.approx_eq(
            s,
            F32Margin {
                ulps: 2,
                epsilon: 1e-4
            }
        ));
        // approx_eq!(Matrix::<3, 2, f32>, ans, s, epsilon = 1e-10);
    }

    #[test]
    fn solve_ax_b_by_lu_f64() {
        use crate::matrix::lu::FactorizeLU;
        use float_cmp::{ApproxEq, F64Margin};

        use lu::TFactorizeLU;

        let a = Matrix::new([[3., 2., 1.], [4., 5., 3.], [1., 6., 3.]]);
        let b = Matrix::new([[48., 56., 32.], [57., 59., 34.]]);
        let ans = Matrix::new([[5., 8., 1.], [7., 9., 0.]]);

        let lu: FactorizeLU<3, 3, f64> = a.lu().unwrap();

        let s = (lu, b).solve().unwrap();

        println!("{}", ans);
        println!("{}", s);

        assert!(ans.approx_eq(
            s,
            F64Margin {
                ulps: 2,
                epsilon: 1e-8
            }
        ));
    }

    #[test]
    fn solve_ax_b_f32() {
        use float_cmp::{ApproxEq, F32Margin};

        let a = Matrix::<3, 3, f32>::new([[3., 2., 1.], [4., 5., 3.], [1., 6., 3.]]);
        let b = Matrix::new([[48., 56., 32.], [57., 59., 34.]]);
        let ans = Matrix::new([[5., 8., 1.], [7., 9., 0.]]);

        let s = (a, b).solve().unwrap();

        println!("{}", ans);
        println!("{}", s);

        assert!(ans.approx_eq(
            s,
            F32Margin {
                ulps: 2,
                epsilon: 1e-4
            }
        ));
        // approx_eq!(Matrix::<3, 2, f32>, ans, s, epsilon = 1e-10);
    }

    #[test]
    fn solve_ax_b_f64() {
        use float_cmp::{ApproxEq, F64Margin};

        let a = Matrix::<3, 3, f64>::new([[3., 2., 1.], [4., 5., 3.], [1., 6., 3.]]);
        let b = Matrix::new([[48., 56., 32.], [57., 59., 34.]]);
        let ans = Matrix::new([[5., 8., 1.], [7., 9., 0.]]);

        let s = (a, b).solve().unwrap();

        println!("{}", ans);
        println!("{}", s);

        assert!(ans.approx_eq(
            s,
            F64Margin {
                ulps: 2,
                epsilon: 1e-8
            }
        ));
        // approx_eq!(Matrix::<3, 2, f32>, ans, s, epsilon = 1e-10);
    }
}
