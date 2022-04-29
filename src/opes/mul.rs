use crate::{Matrix, TMatrix};

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
                    *ret.at_mut(h, w) += self.at(h, index).clone() * lhs.at(index, w).clone();
                }
            }
        }

        ret
    }
}

macro_rules! create_mul_trait_implementation {
    ($f: ident, $type: ty) => {
        impl<const LH: usize, const LWRH: usize, const RW: usize> Mul<Matrix<LWRH, RW, $type>>
            for Matrix<LH, LWRH, $type>
        {
            // Matrix(LH*LW) * Matrix(LW*RW)
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

                let mut ret = MaybeUninit::<[[$type; LH]; RW]>::uninit();

                let (ah, aw) = self.get_size();
                let (bh, bw) = lhs.get_size();
                let (ch, _cw) = Self::Output::get_size_type();

                let trans_mode_a: *const i8 = &('N' as i8);
                let trans_mode_b: *const i8 = &('N' as i8);
                let height_of_a: *const i32 = &(ah as i32);
                let width_of_b: *const i32 = &(bw as i32);
                let widht_of_a_height_of_b: *const i32 = &(aw as i32); // aw == bh
                let alpha: *const $type = &1.0;
                let a: *const $type = self.as_ptr() as *const _;
                let leading_a: *const i32 = &(ah as i32);
                let b: *const $type = lhs.as_ptr() as *const _;
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

                unsafe { Matrix::new(ret.assume_init()) }
            }
        }
    };
}

create_mul_trait_implementation!(sgemm_, f32);
create_mul_trait_implementation!(dgemm_, f64);

#[test]
fn test_matrix_multiple_f32() {
    let l = Matrix::<2, 3, f32>::new([[1., 4.], [2., 5.], [3., 6.]]);
    let r = Matrix::<3, 2, f32>::new([[1., 3., 5.], [2., 4., 6.]]);

    // 1 2 3       1 2       22  28
    //         *   3 4   =
    // 4 5 6       5 6       49  64

    let expect = Matrix::<2, 2, f32>::new([[22., 49.], [28., 64.]]);

    assert_eq!(l * r, expect);
}

#[test]
fn test_matrix_multiple_f64() {
    let l = Matrix::<2, 3, f64>::new([[1., 4.], [2., 5.], [3., 6.]]);
    let r = Matrix::<3, 2, f64>::new([[1., 3., 5.], [2., 4., 6.]]);

    let expect = Matrix::<2, 2, f64>::new([[22., 49.], [28., 64.]]);

    assert_eq!(l * r, expect);
}

#[test]
fn test_matrix_multiple_rust_imple() {
    let l = Matrix::<2, 3, _>::new([[1, 4], [2, 5], [3, 6]]);
    let r = Matrix::<3, 2, _>::new([[1, 3, 5], [2, 4, 6]]);

    let expect = Matrix::<2, 2, i32>::new([[22, 49], [28, 64]]);

    assert_eq!(l * r, expect);
}
