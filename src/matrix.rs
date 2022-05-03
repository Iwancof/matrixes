pub mod general;

pub trait AsMatrix<const H: usize, const W: usize, Inner> {
    fn at(&self, row: usize, col: usize) -> &Inner;

    fn size(&self) -> (usize, usize) {
        Self::type_size()
    }
    fn type_size() -> (usize, usize) {
        (H, W)
    }
}
