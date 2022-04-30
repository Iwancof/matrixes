pub mod general;
pub mod tridiagonal;

#[derive(Debug, Clone, Copy)]
pub enum AXEqBError {
    ArgumentError(u8),
    Unknown(i32),
}

pub trait TAXEqB<const AH: usize, const AWXH: usize, const XW: usize, InnerA, InnerX, InnerB> {
    // solve AX = B
    type Variable;
    unsafe fn solve_into_ptr(&self, x: *mut Self::Variable) -> Option<AXEqBError>;
    fn solve_into(&self, x: &mut Self::Variable) -> Option<AXEqBError> {
        unsafe { self.solve_into_ptr(x) }
    }
    fn solve(&self) -> Result<Self::Variable, AXEqBError> {
        use core::mem::MaybeUninit;
        let mut ret = MaybeUninit::uninit();

        return match unsafe { self.solve_into_ptr(ret.as_mut_ptr()) } {
            Some(err) => Err(err),
            None => unsafe { Ok(ret.assume_init()) },
        };
    }
}

macro_rules! solve {
    ($a: ident times ? = $b: expr) => {{
        use crate::solve::TAXEqB;
        ($a, $b).solve()
    }}; // ($a: ident (tria) times x = $c: expr) => {};
}
