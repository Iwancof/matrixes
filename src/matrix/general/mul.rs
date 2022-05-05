use super::{AsMatrix, GeneralMatrix};

use num_traits::Zero;

use std::ops::{AddAssign, Mul};

impl<const LH: usize, const LWRH: usize, const RW: usize, InnerLeft, InnerRight, InnerOut>
    Mul<GeneralMatrix<LWRH, RW, InnerRight>> for GeneralMatrix<LH, LWRH, InnerLeft>
where
    InnerOut: Zero + Copy + AddAssign,
    InnerLeft: Clone + Mul<InnerRight, Output = InnerOut>,
    InnerRight: Clone,
{
    // Matrix(LH*LW) * Matrix(LW*RW)
    type Output = GeneralMatrix<LH, RW, InnerOut>;
    default fn mul(self, lhs: GeneralMatrix<LWRH, RW, InnerRight>) -> Self::Output {
        let mut ret = GeneralMatrix::zero();

        for h in 0..ret.size().0 {
            for w in 0..ret.size().1 {
                for index in 0..LWRH {
                    *ret.at_mut(h, w) += self.at(h, index).clone() * lhs.at(index, w).clone();
                }
            }
        }

        ret
    }
}

macro_rules! impl_macro {
    ($lapack: ident, $type: ty) => {
        paste::paste! {
            pub fn [<general_matrix_mul_ $lapack >]<const LH: usize, const LWRH: usize, const RW: usize>(
                a: &GeneralMatrix<LH, LWRH, $type>,
                b: &GeneralMatrix<LWRH, RW, $type>,
                c: &mut GeneralMatrix<LH, RW, $type>,
                alpha: $type,
                beta: $type,
            ) {
                extern "C" {
                    fn [<$lapack _>](
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
                let (ah, aw) = (LH, LWRH);
                let (bh, bw) = (LWRH, RW);
                let (ch, _cw) = (LH, RW);

                let trans_mode_a: *const i8 = &('N' as i8);
                let trans_mode_b: *const i8 = &('N' as i8);
                let height_of_a: *const i32 = &(ah as i32);
                let width_of_b: *const i32 = &(bw as i32);
                let widht_of_a_height_of_b: *const i32 = &(aw as i32); // aw == bh
                let alpha: *const $type = &alpha;
                let a: *const $type = a.inner() as *const _ as *const $type;
                let leading_a: *const i32 = &(ah as i32);
                let b: *const $type = b.as_ptr() as *const _ as *const $type;
                let leading_b: *const i32 = &(bh as i32);
                let beta: *const $type = &beta;
                let c: *mut $type = c.inner_mut() as *mut _ as *mut $type;
                let leading_c: *const i32 = &(ch as i32);

                unsafe {
                    [<$lapack _>](
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
            }
impl<const LH: usize, const LWRH: usize, const RW: usize> Mul<GeneralMatrix<LWRH, RW, $type>>
    for GeneralMatrix<LH, LWRH, $type>
{
    fn mul(self, lhs: GeneralMatrix<LWRH, RW, $type>) -> Self::Output {
        let mut ret = GeneralMatrix::zero();
        concat_idents!(general_matrix_mul_, $lapack)(&self, &lhs, &mut ret, 1.0, 0.0);
        ret
    }
}

        }
    }
}

impl_macro!(sgemm, f32);
impl_macro!(dgemm, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication_primitive_rust() {
        let m1 = GeneralMatrix::new_col_major([[1, 4], [2, 5], [3, 6]]);
        let m2 = GeneralMatrix::new_col_major([[1, 3, 5], [2, 4, 6]]);

        let expect = GeneralMatrix::new_col_major([[22, 49], [28, 64]]);

        assert_eq!(m1 * m2, expect);
    }

    #[test]
    fn internal_impl_macro() {
        let m1 = GeneralMatrix::new_col_major([[1., 4.], [2., 5.], [3., 6.]]);
        let m2 = GeneralMatrix::new_col_major([[1., 3., 5.], [2., 4., 6.]]);
        let mut m3 = GeneralMatrix::new_col_major([[1., 1.], [1., 1.]]);

        general_matrix_mul_sgemm(&m1, &m2, &mut m3, 1., 2.);

        let ans = GeneralMatrix::new_col_major([[24., 51.], [30., 66.]]);
        assert_eq!(m3, ans);
    }

    #[test]
    fn multiplication_lapack_f32() {
        let m1: GeneralMatrix<2, 3, f32> =
            GeneralMatrix::new_col_major([[1., 4.], [2., 5.], [3., 6.]]);
        let m2 = GeneralMatrix::new_col_major([[1., 3., 5.], [2., 4., 6.]]);

        let ans = GeneralMatrix::new_col_major([[22., 49.], [28., 64.]]);

        assert_eq!(m1 * m2, ans);
    }

    #[test]
    fn multiplication_lapack_f64() {
        let m1: GeneralMatrix<2, 3, f64> =
            GeneralMatrix::new_col_major([[1., 4.], [2., 5.], [3., 6.]]);
        let m2 = GeneralMatrix::new_col_major([[1., 3., 5.], [2., 4., 6.]]);

        let ans = GeneralMatrix::new_col_major([[22., 49.], [28., 64.]]);

        assert_eq!(m1 * m2, ans);
    }
}
