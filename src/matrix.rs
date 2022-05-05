pub mod general;
pub mod tridiagonal;

use num_traits::Zero;

use core::fmt::{self, Display, Formatter};

pub trait AsMatrix<const H: usize, const W: usize, Inner> {
    fn at(&self, row: usize, col: usize) -> &Inner;

    fn size(&self) -> (usize, usize) {
        Self::type_size()
    }
    fn type_size() -> (usize, usize) {
        (H, W)
    }
}

pub fn print_matrix_display<const H: usize, const W: usize, Inner, T>(
    mt: &T,
    f: &mut Formatter<'_>,
) -> fmt::Result
where
    T: AsMatrix<H, W, Inner>,
    Inner: Display,
{
    if let Some(width) = f.width() {
        for h in 0..H {
            for w in 0..W {
                write!(f, "{:width$}", mt.at(w, h), width = width)?;
            }
            writeln!(f)?;
        }
    } else {
        for h in 0..H {
            for w in 0..W {
                write!(f, "{}", mt.at(w, h))?;
            }
            writeln!(f)?;
        }
    }

    Ok(())
}
