pub mod ax_eq_b;

macro_rules! solve {
    ($a: ident times x = $c: expr) => {{
        use crate::solve::ax_eq_b::TAXEqB;
        ($a, $c).solve()
    }}; // ($a: ident (tria) times x = $c: expr) => {};
}
