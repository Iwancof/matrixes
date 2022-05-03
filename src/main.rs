#![allow(incomplete_features)]
#![feature(new_uninit)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]

pub mod factorizations;
pub mod matrix;

use matrix::general::lu::{self, GeneralLuError, GeneralLuFormat, GeneralLuPivot};
use matrix::general::GeneralMatrix;

const fn m(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

pub struct MyPivot<const A: usize> {
    i: [i32; A],
}

impl<const A: usize> Default for MyPivot<A> {
    fn default() -> Self {
        Self { i: [0; A] }
    }
}

pub struct TestLu<const H: usize, const W: usize, Inner, Piv> {
    inner: GeneralMatrix<H, W, Inner>,
    p: Piv,
}

impl<const H: usize, const W: usize>
    crate::factorizations::lu::LuFormat<GeneralMatrix<H, W, f64>, MyPivot<{ m(H, W) }>>
    for TestLu<H, W, f64, MyPivot<{ m(H, W) }>>
where
    MyPivot<{ m(H, W) }>: Default,
{
    fn new_with(mt: GeneralMatrix<H, W, f64>, pivot: MyPivot<{ m(H, W) }>) -> Self {
        todo!()
    }
    fn new_with_box(mt: Box<GeneralMatrix<H, W, f64>>, pivot: MyPivot<{ m(H, W) }>) -> Box<Self> {
        todo!()
    }
    fn data_ref(&self) -> (&GeneralMatrix<H, W, f64>, &MyPivot<{ m(H, W) }>) {
        todo!()
    }
    fn data_mut(&mut self) -> (&mut GeneralMatrix<H, W, f64>, &mut MyPivot<{ m(H, W) }>) {
        todo!()
    }
}

impl<const H: usize, const W: usize>
    crate::factorizations::lu::AsLu<H, W, f64, MyPivot<{ m(H, W) }>, GeneralLuError>
    for GeneralMatrix<H, W, f64>
{
    type Lu = TestLu<H, W, f64, MyPivot<{ m(H, W) }>>;

    default fn fact_internal(dest: &mut Self::Lu) -> GeneralLuError {
        #[link(name = "lapack")]
        extern "C" {
            fn aa();
        }

        unsafe {
            aa();
        }

        todo!()
    }
}

fn main() {
    use factorizations::lu::{AsLu, LuFormat};
    use num_traits::Zero;

    let m2: GeneralMatrix<3, 3, f32> =
        GeneralMatrix::new_row_major([[10., 10., 10.], [10., 10., 10.], [10., 10., 10.]]);
    m2.lu();

    // let m2: GeneralMatrix<3, 3, f64> =
    //     GeneralMatrix::new_row_major([[10., 10., 10.], [10., 10., 10.], [10., 10., 10.]]);
    // m2.lu();

    /*
    let mut x =
        <GeneralMatrix<3, 3, f32> as AsLu<3, 3, f32, GeneralLuPivot<3>, GeneralLuError>>::Lu::new(
            m2,
        );

    <GeneralMatrix<3, 3, f32> as AsLu<3, 3, f32, GeneralLuPivot<3>, GeneralLuError>>::fact_internal(
        &mut x,
    );
    println!("{:?}", x);
    */

    /*
    let d =
        <GeneralMatrix<3, 3, f32> as AsLu<3, 3, f32, GeneralLuPivot<3>, GeneralLuError>>::lu(m2);

    println!("{:?}", d.unwrap());
    */

    // let x = <<GeneralMatrix<10, 10, f32> as AsLu<10, 10, f32, GeneralLuPivot<10>, GeneralLuError>>::Lu as LuFormat<GeneralMatrix<10, 10, f32>, GeneralLuPivot<10>>>::new(GeneralMatrix::<10, 10, f32>::zero());
    /*
    let x = <GeneralLuFormat<10, 10, f32, GeneralLuPivot<10>> as LuFormat<
        GeneralMatrix<10, 10, f32>,
        GeneralLuPivot<10>,
    >>::new(GeneralMatrix::<10, 10, f32>::zero());
    */
    let x = <GeneralLuFormat<10, 10, f32, GeneralLuPivot<10>> as LuFormat<
        GeneralMatrix<10, 10, f32>,
        GeneralLuPivot<10>,
    >>::test;

    print_typename(x);

    let f = crate::factorizations::lu::work;
    print_typename(f);

    // println!("{:?}", x);
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
