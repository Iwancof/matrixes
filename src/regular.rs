use crate::matrix::lu;
use crate::{Matrix, TMatrix};

#[derive(Debug, Clone, Copy)]
pub enum InverseError {
    LuError(lu::LuError),
    ArgumentError(u8),
    Singular,
}

pub trait TRegularMatrix: TMatrix + Sized {
    unsafe fn inv_into_ptr(&self, target: *mut Self) -> Option<InverseError>;
    fn inv_into_ref(&self, target: &mut Self) -> Option<InverseError> {
        unsafe { self.inv_into_ptr(target) }
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
    default unsafe fn inv_into_ptr(&self, _target: *mut Self) -> Option<InverseError> {
        todo!()
    }
}

macro_rules! create_inverse_trait_implementation {
    ($i: ident, $type: ty) => {
        impl<const S: usize> TRegularMatrix for Matrix<S, S, $type>
        where
            [(); lu::min(S, S)]:,
        {
            // #[inline(never)]
            unsafe fn inv_into_ptr(&self, target: *mut Self) -> Option<InverseError> {
                #[link(name = "lapack")]
                extern "C" {

                    fn $i(
                        n: *const i32,     // integer
                        A: *mut $type,     // array of $type. length = S * S
                        lda: *const i32,   // integer
                        ipiv: *const i32,  // array of integer.. length = S
                        work: *mut $type,  // array of $type. length = S
                        lwork: *const i32, // integer
                        info: *mut i32,    // integer
                    );
                }

                use lu::TFactorizeLU;
                let mut lu = self
                    .clone()
                    .lu()
                    .map_err(|err| InverseError::LuError(err))
                    .ok()?;

                let (a, ipiv): (*mut $type, *const i32) = lu.as_ptr_mut_e_const_piv();
                let n: *const i32 = &(S as i32);
                let lda: *const i32 = &(S as i32);
                let mut work: [$type; S] = [0.0; S];
                let lwork: *const i32 = &(S as i32);
                let info: *mut i32 = &mut 0;

                $i(n, a, lda, ipiv, work.as_mut_ptr(), lwork, info);

                if *info != 0 {
                    return match *info {
                        -7..=-1 => Some(InverseError::ArgumentError((-*info) as u8)),
                        _ => Some(InverseError::Singular),
                    };
                }

                *target = lu.into();

                None
            }
        }
    };
}

create_inverse_trait_implementation!(sgetri_, f32);
create_inverse_trait_implementation!(dgetri_, f64);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_inverse_f32() {
        let m = Matrix::<2, 2, f32>::new([[4., 2.], [1., 3.]]);
        let ans = Matrix::<2, 2, f32>::new([[0.3, -0.2], [-0.1, 0.4]]);

        assert_eq!(m.inv().unwrap(), ans);
    }
    #[test]
    fn test_inverse_f64() {
        let m = Matrix::<2, 2, f64>::new([[4., 2.], [1., 3.]]);
        let ans = Matrix::<2, 2, f64>::new([[0.3, -0.2], [-0.1, 0.4]]);

        assert_eq!(m.inv().unwrap(), ans);
    }
}
