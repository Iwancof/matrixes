use super::{AsMatrix, GeneralMatrix};

use std::ops::Add;

impl<const H: usize, const W: usize, InnerLeft, InnerRight, InnerOut>
    Add<GeneralMatrix<H, W, InnerRight>> for GeneralMatrix<H, W, InnerLeft>
where
    InnerOut: Clone,
    InnerRight: Clone,
    InnerLeft: Add<InnerRight, Output = InnerOut> + Clone,
{
    type Output = GeneralMatrix<H, W, InnerOut>;

    default fn add(self, rhs: GeneralMatrix<H, W, InnerRight>) -> Self::Output {
        use array_macro::array;

        Self::Output::new_col_major(
            array![col => array![row => self.at(row, col).clone() + rhs.at(row, col).clone(); H]; W],
        )
    }
}

macro_rules! impl_macro {
    ($lapack: ident, $type: ty) => {
        paste::paste! {
            #[inline(always)]
            #[allow(unused)]
            pub fn [<general_matrix_add_ $lapack >]<const H: usize, const W: usize>(
                dest: &mut GeneralMatrix<H, W, $type>,
                alpha: $type,
                value: &GeneralMatrix<H, W, $type>,
            ) {
                // dest += alpha * value

                #[link(name = "lapack")]
                extern "C" {
                    fn [<$lapack _>](
                        n: *const i32,
                        alpha: *const $type,
                        x: *const $type,
                        incx: *const i32,
                        y: *mut $type,
                        incy: *const i32,
                    );
                }
                let n = &((H * W) as i32);
                let alpha = &alpha;
                let x = value.inner() as *const _ as *const $type;
                let incx = &1;
                let y = dest.inner_mut() as *mut _ as *mut $type;
                let incy = &1;

                unsafe {
                    [<$lapack _>](n, alpha, x, incx, y, incy);
                }
            }
        }

        impl<const H: usize, const W: usize> Add<GeneralMatrix<H, W, $type>>
            for GeneralMatrix<H, W, $type>
        {
            #[inline(always)]
            default fn add(self, rhs: GeneralMatrix<H, W, $type>) -> Self::Output {
                let mut dest = self;
                concat_idents!(general_matrix_add_, $lapack)(&mut dest, 1.0, &rhs);
                dest
            }
        }
    };
}

impl_macro!(saxpy, f32);
impl_macro!(daxpy, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_primitive_rust() {
        let m1 = GeneralMatrix::new_row_major([[1, 2, 3], [4, 5, 6]]);
        let m2 = GeneralMatrix::new_row_major([[6, 5, 4], [3, 2, 1]]);

        let ans = GeneralMatrix::new_row_major([[7, 7, 7], [7, 7, 7]]);

        assert_eq!(m1 + m2, ans);
    }

    #[test]
    fn internal_impl_macro() {
        let mut m1 = GeneralMatrix::new_row_major([[1., 2., 3.], [4., 5., 6.]]);
        let m2 = GeneralMatrix::new_row_major([[6., 5., 4.], [3., 2., 1.]]);

        let ans = GeneralMatrix::new_row_major([[7., 7., 7.], [7., 7., 7.]]);

        general_matrix_add_saxpy(&mut m1, 1.0, &m2);

        assert_eq!(m1, ans);
    }

    #[test]
    fn addition_lapack_f32() {
        let m1: GeneralMatrix<2, 3, f32> =
            GeneralMatrix::new_row_major([[1., 2., 3.], [4., 5., 6.]]);
        let m2: GeneralMatrix<2, 3, f32> =
            GeneralMatrix::new_row_major([[6., 5., 4.], [3., 2., 1.]]);

        let ans: GeneralMatrix<2, 3, f32> =
            GeneralMatrix::new_row_major([[7., 7., 7.], [7., 7., 7.]]);

        assert_eq!(m1 + m2, ans);
    }

    #[test]
    fn addition_lapack_f64() {
        let m1: GeneralMatrix<2, 3, f64> =
            GeneralMatrix::new_row_major([[1., 2., 3.], [4., 5., 6.]]);
        let m2: GeneralMatrix<2, 3, f64> =
            GeneralMatrix::new_row_major([[6., 5., 4.], [3., 2., 1.]]);

        let ans: GeneralMatrix<2, 3, f64> =
            GeneralMatrix::new_row_major([[7., 7., 7.], [7., 7., 7.]]);

        assert_eq!(m1 + m2, ans);
    }
}
