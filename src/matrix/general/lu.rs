use super::GeneralMatrix;
use crate::factorizations::lu;

pub const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Debug, PartialEq)]
pub struct GeneralLuPivot<const S: usize> {
    pub pivot: [i32; S],
    // pub pivot: i32,
}

impl<const S: usize> Default for GeneralLuPivot<S> {
    fn default() -> Self {
        Self { pivot: [0; S] }
        // Self { pivot: 0 }
    }
}

#[derive(Debug, PartialEq)]
pub struct GeneralLuFormat<const H: usize, const W: usize, Inner, Pivot>
where
    Inner: Clone, // where
                  // [(); min(H, W)]:,
{
    pub internal_matrix: GeneralMatrix<H, W, Inner>,
    // pivot: [i32; min(H, W)],
    pub pivot: Pivot,
}

impl<const H: usize, const W: usize, Inner, Pivot> GeneralLuFormat<H, W, Inner, Pivot>
where
    Inner: Clone,
{
    pub fn new(matrix: GeneralMatrix<H, W, Inner>, pivot: Pivot) -> Self {
        Self {
            internal_matrix: matrix,
            pivot,
        }
    }
}
impl<const S: usize, Inner, Pivot> GeneralLuFormat<S, S, Inner, Pivot>
where
    Inner: Clone,
{
    pub fn from_l_u(
        l: GeneralMatrix<S, S, Inner>,
        u: GeneralMatrix<S, S, Inner>,
        pivot: Pivot,
    ) -> Self
    where
        Inner: Copy
            + Clone
            + num_traits::Zero
            + num_traits::One
            + std::ops::SubAssign
            + std::ops::AddAssign
            + std::ops::Add,
    {
        use num_traits::One;
        println!("Reimplement with sub!");
        let mut internal = l + u; // - GeneralMatrix::one();

        for i in 0..S {
            *internal.at_mut(i, i) -= Inner::one();
        }

        Self {
            internal_matrix: internal,
            pivot,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct GeneralLuError(i32);

impl GeneralLuError {
    pub const SUCCESS: Self = Self(0);
    pub const INVALID_ARG_ROW_SIZE: Self = Self(-1);
    pub const INVALID_ARG_COLUMN_SIZE: Self = Self(-2);
    pub const INVALID_ARG_MATRIX: Self = Self(-3);
    pub const INVALID_ARG_MATRIX_LD: Self = Self(-4);
    pub const INVALID_ARG_PIVOT: Self = Self(-5);
    pub const INVALID_ARG_INFO: Self = Self(-6);
}

impl core::fmt::Debug for GeneralLuError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::SUCCESS => write!(f, "GeneralLuError::SUCCESS"),
            Self::INVALID_ARG_ROW_SIZE => write!(f, "GeneralLuError::INVALID_ARG_ROW_SIZE"),
            Self::INVALID_ARG_COLUMN_SIZE => write!(f, "GeneralLuError::INVALID_ARG_COLUMN_SIZE"),
            Self::INVALID_ARG_MATRIX => write!(f, "GeneralLuError::INVALID_ARG_MATRIX"),
            Self::INVALID_ARG_MATRIX_LD => write!(f, "GeneralLuError::INVALID_ARG_MATRIX_LD"),
            Self::INVALID_ARG_PIVOT => write!(f, "GeneralLuError::INVALID_ARG_PIVOT"),
            Self::INVALID_ARG_INFO => write!(f, "GeneralLuError::INVALID_ARG_INFO"),
            _ => write!(f, "The factorization has been completed, but because of element {} of the diagonal is exactly zero, division by zero will occur if it is used to solve equation by the result.", self.0),
        }
    }
}

impl core::fmt::Display for GeneralLuError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}

impl lu::AsLuError for GeneralLuError {
    fn as_lapack_into_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
    fn as_lapack_into(&self) -> &i32 {
        &self.0
    }
}

macro_rules! impl_macro {
    ($lapack: ident, $type: ty) => {
        paste::paste! {
            /*
            impl<const H: usize, const W: usize, Inner, Pivot>
                lu::LuFormat<GeneralMatrix<H, W, Inner>, Pivot>
                for GeneralLuFormat<H, W, f32, Pivot>


                こういうふうに実装したいけどコンパイラが unstable feature だからか link error が発生するためマクロで対処
                PoC: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=ba0622c9087eb18a11611dabe4bc5563
                該当commit: https://github.com/Iwancof/matrixes/commit/30926fd8d2a684a07c138ee599e952449e7cf5ec

            */

            impl<const H: usize, const W: usize>
                lu::LuFormat<GeneralMatrix<H, W, $type>, GeneralLuPivot<{ min(H, W) }>>
                for GeneralLuFormat<H, W, $type, GeneralLuPivot<{ min(H, W) }>>
            where
                Self: Sized,
            {
                fn new_with(
                    internal_matrix: GeneralMatrix<H, W, $type>,
                    pivot: GeneralLuPivot<{ min(H, W) }>,
                ) -> Self {
                    Self {
                        internal_matrix,
                        pivot,
                    }
                }
                fn data_ref(&self) -> (&GeneralMatrix<H, W, $type>, &GeneralLuPivot<{ min(H, W) }>) {
                    (&self.internal_matrix, &self.pivot)
                }
                fn data_mut(
                    &mut self,
                ) -> (
                    &mut GeneralMatrix<H, W, $type>,
                    &mut GeneralLuPivot<{ min(H, W) }>,
                ) {
                    (&mut self.internal_matrix, &mut self.pivot)
                }
            }
            impl<const H: usize, const W: usize>
                lu::AsLu<H, W, $type, GeneralLuPivot<{ min(H, W) }>, GeneralLuError>
                for GeneralMatrix<H, W, $type>
            {
                type Lu = GeneralLuFormat<H, W, $type, GeneralLuPivot<{ min(H, W) }>>;

                default fn fact_internal(dest: &mut Self::Lu) -> GeneralLuError {
                    #[link(name = "lapack")]
                    extern "C" {
                        fn [<$lapack _>] (
                            m: *const i32,   // integer
                            n: *const i32,   // integer
                            a: *mut $type,     // array of $type. length = S * S
                            lda: *const i32, // integer
                            ipiv: *mut i32,  // array of integer. length = S
                            info: *mut i32,  // integer);
                        );
                    }

                    let m: *const i32 = &(H as i32);
                    let n: *const i32 = &(W as i32);
                    let lda: *const i32 = &(H as i32);

                    use lu::{AsLuError, LuFormat};
                    let (mat, piv) = dest.data_mut();
                    let mat = mat.inner_mut() as *mut _ as *mut $type;
                    let piv = &mut piv.pivot as *mut _ as *mut i32;

                    let mut error = GeneralLuError::SUCCESS;

                    unsafe { concat_idents!($lapack, _)(m, n, mat, lda, piv, error.as_lapack_into_mut()) };

                    error
                }
            }
        }
    };
}

impl_macro!(sgetrf, f32);
impl_macro!(dgetrf, f64);

#[cfg(test)]
mod test {
    use super::*;
    use crate::factorizations::lu::{AsLu, AsLuError};

    #[test]
    fn factorize_lu_f32() {
        let m = GeneralMatrix::new_col_major([[2. as f32, 4.], [9., 4.]]);

        let l = GeneralMatrix::new_col_major([[1., 0.5], [0., 1.]]);
        let u = GeneralMatrix::new_col_major([[4., 0.], [4., 7.]]);
        let piv = GeneralLuPivot { pivot: [2, 2] };

        let ans = GeneralLuFormat::from_l_u(l, u, piv);

        let result = m.lu();
        let result = result.expect("factorization failed");

        let (matrix, error): (
            GeneralLuFormat<2, 2, f32, GeneralLuPivot<2>>,
            GeneralLuError,
        ) = result;

        assert!(!error.is_error());

        assert_eq!(matrix, ans);
    }

    #[test]
    fn factorize_lu_f64() {
        let m = GeneralMatrix::new_col_major([[2. as f64, 4.], [9., 4.]]);

        let l = GeneralMatrix::new_col_major([[1., 0.5], [0., 1.]]);
        let u = GeneralMatrix::new_col_major([[4., 0.], [4., 7.]]);
        let piv = GeneralLuPivot { pivot: [2, 2] };

        let ans = GeneralLuFormat::from_l_u(l, u, piv);

        let result = m.lu();
        let result = result.expect("factorization failed");

        let (matrix, error): (
            GeneralLuFormat<2, 2, f64, GeneralLuPivot<2>>,
            GeneralLuError,
        ) = result;

        assert!(!error.is_error());

        assert_eq!(matrix, ans);
    }
}
