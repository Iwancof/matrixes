use crate::matrix::AsMatrix;

pub trait LuFormat<Base, Pivot>
where
    Self: Sized,
    Pivot: Default,
{
    // require methods
    fn new_with(mt: Base, pivot: Pivot) -> Self;
    fn data_ref(&self) -> (&Base, &Pivot);
    fn data_mut(&mut self) -> (&mut Base, &mut Pivot);

    fn new(mt: Base) -> Self
    where
        Pivot: Default,
    {
        Self::new_with(mt, Default::default())
    }
}

pub trait AsLuError {
    fn as_lapack_into_mut(&mut self) -> &mut i32;
    fn as_lapack_into(&self) -> &i32;

    fn is_invalid(&self) -> bool {
        self.as_lapack_into() < &0
    }
    fn is_error(&self) -> bool {
        self.as_lapack_into() != &0
    }
}

// following trait will be implemented for Matrix
pub trait AsLu<const H: usize, const W: usize, Inner, Pivot, LuError>
where
    // Pivot: Default,
    Self: AsMatrix<H, W, Inner> + Sized,
    LuError: AsLuError,
    Pivot: Default,
{
    type Lu: LuFormat<Self, Pivot>;

    // require methods
    fn fact_internal(dest: &mut Self::Lu) -> LuError;

    // provide methods
    fn lu(self) -> Result<(Self::Lu, LuError), LuError> {
        let mut dest = Self::Lu::new(self);
        let err = Self::fact_internal(&mut dest);
        if err.is_invalid() {
            Err(err)
        } else {
            Ok((dest, err))
        }
    }
}
