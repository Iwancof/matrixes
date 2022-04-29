use crate::Matrix;

use std::ops::Add;
impl<const H: usize, const W: usize, InnerLeft, InnerRight, InnerOut> Add<Matrix<H, W, InnerRight>>
    for Matrix<H, W, InnerLeft>
where
    InnerOut: Default + Copy,
    InnerLeft: Clone + Add<InnerRight, Output = InnerOut>,
    InnerRight: Clone,
{
    type Output = Matrix<H, W, InnerOut>;
    default fn add(self, lhs: Matrix<H, W, InnerRight>) -> Self::Output {
        {
            let mut output = Matrix::default();
            for h in 0..H {
                for w in 0..W {
                    *output.at_mut(h, w) = self.at(h, w).clone() + lhs.at(h, w).clone();
                }
            }

            output
        }
    }
}

macro_rules! create_add_trait_implementation {
    ($f: ident, $type: ty) => {
        impl<const H: usize, const W: usize> Add<Matrix<H, W, $type>> for Matrix<H, W, $type> {
            default fn add(self, lhs: Matrix<H, W, $type>) -> Self::Output {
                {
                    extern "C" {
                        fn $f(
                            n: *const i32,
                            alpha: *const $type,
                            x: *const $type,
                            incx: *const i32,
                            y: *mut $type,
                            incy: *const i32,
                        );
                    }
                    let mut output = self;

                    let n = &((H * W) as i32);
                    let alpha = &1.0;
                    let x = lhs.as_ptr();
                    let incx = &1;
                    let y = output.as_mut_ptr();
                    let incy = &1;

                    unsafe {
                        $f(n, alpha, x, incx, y, incy);
                    }

                    output
                }
            }
        }
    };
}

create_add_trait_implementation!(saxpy_, f32);
create_add_trait_implementation!(daxpy_, f64);

mod test {
    use super::*;
    #[test]
    fn test_matrix_addition_f32() {
        let left = Matrix::<3, 2, f32>::new([[1., 2., 3.], [4., 5., 6.]]);
        let right = Matrix::new([[6., 5., 4.], [3., 2., 1.]]);
        let expect = Matrix::new([[7., 7., 7.], [7., 7., 7.]]);

        assert_eq!(left + right, expect);
    }

    #[test]
    fn test_matrix_addition_f64() {
        let left = Matrix::<3, 2, f64>::new([[1., 2., 3.], [4., 5., 6.]]);
        let right = Matrix::new([[6., 5., 4.], [3., 2., 1.]]);
        let expect = Matrix::new([[7., 7., 7.], [7., 7., 7.]]);

        assert_eq!(left + right, expect);
    }

    #[test]
    fn test_matrix_addition_rust_imple() {
        let left = Matrix::<3, 2, i32>::new([[1, 2, 3], [4, 5, 6]]);
        let right = Matrix::new([[6, 5, 4], [3, 2, 1]]);
        let expect = Matrix::new([[7, 7, 7], [7, 7, 7]]);

        assert_eq!(left + right, expect);
    }
}
