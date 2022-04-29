use crate::{Matrix, TMatrix};
use std::fmt::{Debug, Display};

pub const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub struct FactorizeLU<const H: usize, const W: usize, Inner>
where
    [(); min(H, W)]:,
{
    e: [[Inner; H]; W],
    ipiv: [i32; min(H, W)],
}

impl<const H: usize, const W: usize, Inner> FactorizeLU<H, W, Inner>
where
    [(); min(H, W)]:,
{
    #[inline(always)]
    #[allow(unused)]
    pub const fn at(&self, row: usize, col: usize) -> &Inner {
        &self.e[col][row]
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Inner {
        &mut self.e[col][row]
    }
}

macro_rules! create_unit_lu_struct {
    ($type: ty) => {
        impl<const S: usize> FactorizeLU<S, S, $type>
        where
            [(); min(S, S)]:,
        {
            #[inline(always)]
            #[allow(unused)]
            pub fn from_l_u_ipiv(
                l: Matrix<S, S, $type>,
                u: Matrix<S, S, $type>,
                ipiv: [i32; min(S, S)],
            ) -> Self
            where
                [(); S]:,
                $type: std::ops::Add<Output = $type> + Copy + Clone + Default,
            {
                Self {
                    e: (l + u - Matrix::<S, S, $type>::one()).get(),
                    ipiv,
                }
            }
        }
    };
}

create_unit_lu_struct!(f32);
create_unit_lu_struct!(f64);

impl<const H: usize, const W: usize, Inner> Display for FactorizeLU<H, W, Inner>
where
    Inner: Display,
    [(); min(H, W)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "e:")?;
        for y in 0..H {
            write!(f, "  ")?;
            for x in 0..W {
                write!(f, "{:5}, ", self.at(y, x))?;
            }
            writeln!(f, "")?;
        }

        writeln!(f, "ipiv:")?;
        write!(f, "  ")?;
        for y in 0..min(H, W) {
            write!(f, "{:5}, ", self.ipiv[y])?;
        }
        writeln!(f, "")?;

        Ok(())
    }
}
impl<const H: usize, const W: usize, Inner> Debug for FactorizeLU<H, W, Inner>
where
    Inner: Debug,
    [(); min(H, W)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..H {
            write!(f, "[")?;
            for x in 0..W {
                write!(f, "{:5?}, ", self.at(y, x))?;
            }
            write!(f, "], ")?;
        }

        Ok(())
    }
}

pub trait TFactorizeLU<const H: usize, const W: usize, Inner>: TMatrix
where
    [(); min(H, W)]:,
{
    fn lu(self) -> FactorizeLU<H, W, Inner>;
}

impl<const H: usize, const W: usize, Inner> TFactorizeLU<H, W, Inner> for Matrix<H, W, Inner>
where
    [(); min(H, W)]:,
{
    default fn lu(self) -> FactorizeLU<H, W, Inner> {
        unimplemented!()
    }
}

macro_rules! create_lu_decom_imple {
    ($f: ident, $type: ty) => {
        impl<const H: usize, const W: usize> TFactorizeLU<H, W, $type> for Matrix<H, W, $type>
        where
            [(); min(H, W)]:,
        {
            default fn lu(mut self) -> FactorizeLU<H, W, $type> {
                extern "C" {
                    fn $f(
                        m: *const i32,   // integer
                        n: *const i32,   // integer
                        a: *mut $type,   // array of $type. length = S * S
                        lda: *const i32, // integer
                        ipiv: *mut i32,  // array of integer. length = S
                        info: *mut i32,  // integer
                    );
                }

                let m: *const i32 = &(H as i32);
                let n: *const i32 = &(W as i32);
                let a: *mut $type = self.as_mut_ptr();
                let lda: *const i32 = &(H as i32);
                let mut ipiv: [i32; min(H, W)] = [0; min(H, W)];
                /*
                let lda: *const i32 = &(H as i32);
                let mut ipiv: [i32; 3] = [100; 3];
                */
                let info: *mut i32 = &mut 0;

                unsafe { $f(m, n, a, lda, ipiv.as_mut_ptr(), info) };
                println!("{:?}", ipiv);

                // unsafe { println!("{}", *info) };

                FactorizeLU {
                    e: self.e,
                    // ipiv: ipiv[0..2].try_into().unwrap(),
                    ipiv,
                }
            }
        }
    };
}

create_lu_decom_imple!(sgetrf_, f32);
create_lu_decom_imple!(dgetrf_, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn factorize_lu_f32() {
        let m = Matrix::new([[2., 4.], [9., 4.]]);
        let lu = m.lu();

        let l = Matrix::new([[1., 0.5], [0., 1.]]);
        let u = Matrix::new([[4., 0.], [4., 7.]]);
        let piv = [2, 2];
        let ans = FactorizeLU::<2, 2, f32>::from_l_u_ipiv(l, u, piv);

        assert_eq!(lu, ans);
    }
    #[test]
    fn factorize_lu_f64() {
        let m = Matrix::new([[2., 4.], [9., 4.]]);
        let lu = m.lu();

        let l = Matrix::new([[1., 0.5], [0., 1.]]);
        let u = Matrix::new([[4., 0.], [4., 7.]]);
        let piv = [2, 2];
        let ans = FactorizeLU::<2, 2, f64>::from_l_u_ipiv(l, u, piv);

        assert_eq!(lu, ans);
    }
}
