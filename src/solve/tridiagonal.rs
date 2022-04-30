use crate::matrix::tridiagonal;
use crate::{Matrix, TMatrix};

use super::{AXEqBError, TAXEqB};

impl<const AS: usize, const XW: usize, InnerA, InnerX, InnerB>
    TAXEqB<AS, AS, XW, InnerA, InnerX, InnerB>
    for (
        tridiagonal::TridiagonalMatrix<AS, InnerA>,
        Matrix<AS, XW, InnerB>,
    )
where
    [(); AS - 1]:,
{
    type Variable = Matrix<AS, XW, InnerX>;
    default unsafe fn solve_into_ptr(&self, _v: *mut Self::Variable) -> Option<AXEqBError> {
        todo!()
    }
}

impl<const AS: usize, const XW: usize> TAXEqB<AS, AS, XW, f32, f32, f32>
    for (tridiagonal::TridiagonalMatrix<AS, f32>, Matrix<AS, XW, f32>)
where
    [(); AS - 1]:,
{
    unsafe fn solve_into_ptr(&self, _v: *mut Self::Variable) -> Option<AXEqBError> {
        extern "C" {
            fn sgtsv_(n: *const i32, nrhs: *const i32);
        }

        todo!();
    }
}
