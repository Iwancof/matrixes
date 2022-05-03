use super::GeneralMatrix;
use crate::factorizations::lu;

pub const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

// #[derive(Debug)]
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

#[derive(Debug)]
pub struct GeneralLuFormat<const H: usize, const W: usize, Inner, Pivot>
// where
// [(); min(H, W)]:,
{
    internal_matrix: GeneralMatrix<H, W, Inner>,
    // pivot: [i32; min(H, W)],
    pivot: Pivot,
}

impl<const H: usize, const W: usize>
    lu::LuFormat<GeneralMatrix<H, W, f32>, GeneralLuPivot<{ min(H, W) }>>
    for GeneralLuFormat<H, W, f32, GeneralLuPivot<{ min(H, W) }>>
where
    Self: Sized,
{
    fn new_with(
        internal_matrix: GeneralMatrix<H, W, f32>,
        pivot: GeneralLuPivot<{ min(H, W) }>,
    ) -> Self {
        Self {
            internal_matrix,
            pivot,
        }
    }
    fn new_with_box(
        mt: Box<GeneralMatrix<H, W, f32>>,
        pivot: GeneralLuPivot<{ min(H, W) }>,
    ) -> Box<Self> {
        let mut make = Box::new_uninit();

        unsafe {
            core::ptr::write(
                make.as_mut_ptr(),
                Self {
                    internal_matrix: *mt,
                    pivot,
                },
            );
        }

        unsafe { make.assume_init() }
    }
    fn data_ref(&self) -> (&GeneralMatrix<H, W, f32>, &GeneralLuPivot<{ min(H, W) }>) {
        (&self.internal_matrix, &self.pivot)
    }
    fn data_mut(
        &mut self,
    ) -> (
        &mut GeneralMatrix<H, W, f32>,
        &mut GeneralLuPivot<{ min(H, W) }>,
    ) {
        (&mut self.internal_matrix, &mut self.pivot)
    }
}

#[derive(Debug)]
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

impl lu::AsLuError for GeneralLuError {
    fn as_lapack_into_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
    fn as_lapack_into(&self) -> &i32 {
        &self.0
    }
}

impl<const H: usize, const W: usize>
    lu::AsLu<H, W, f32, GeneralLuPivot<{ min(H, W) }>, GeneralLuError>
    for GeneralMatrix<H, W, f32>
{
    type Lu = GeneralLuFormat<H, W, f32, GeneralLuPivot<{ min(H, W) }>>;

    default fn fact_internal(dest: &mut Self::Lu) -> GeneralLuError {
        #[link(name = "lapack")]
        extern "C" {
            fn sgetrf_(
                m: *const i32,   // integer
                n: *const i32,   // integer
                a: *mut f32,     // array of $type. length = S * S
                lda: *const i32, // integer
                ipiv: *mut i32,  // array of integer. length = S
                info: *mut i32,  // integer
            );
        }
        Self::Lu::test();

        let m: *const i32 = &(H as i32);
        let n: *const i32 = &(W as i32);
        let lda: *const i32 = &(H as i32);

        use lu::{AsLuError, LuFormat};
        let (mat, piv) = dest.data_mut();
        let mat = mat.inner_mut() as *mut _ as *mut f32;
        let piv = &mut piv.pivot as *mut _ as *mut i32;

        let mut error = GeneralLuError::SUCCESS;

        unsafe { sgetrf_(m, n, mat, lda, piv, error.as_lapack_into_mut()) };

        error
    }
}
