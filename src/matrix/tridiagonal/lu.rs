use crate::matrix::TMatrix;

#[derive(Debug, Clone, Copy)]

pub enum LuError {
    InvalidArgument(u8),
    Other(i32),
}
