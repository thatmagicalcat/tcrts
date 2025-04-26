#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::marker::PhantomData;
fn main() {
    type Ten = Next<Next<Next<Next<Next<Next<Next<Next<Next<Next<Zero>>>>>>>>>>;
    type Twenty = Next<
        Next<
            Next<
                Next<
                    Next<
                        Next<
                            Next<
                                Next<
                                    Next<
                                        Next<
                                            Next<
                                                Next<Next<Next<Next<Next<Next<Next<Next<Next<Zero>>>>>>>>>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >;
    {
        ::std::io::_print(format_args!("{0}\n", <Ten as Add<Twenty>>::Output::VALUE));
    };
}
struct Zero;
struct Next<N>(std::marker::PhantomData<N>);
trait NN {}
impl NN for Zero {}
impl<N: NN> NN for Next<N> {}
trait Add<N> {
    type Output;
}
impl<N: NN> Add<Zero> for N {
    type Output = N;
}
impl<N, M> Add<Next<M>> for N
where
    M: NN,
    N: NN + Add<M>,
{
    type Output = Next<<N as Add<M>>::Output>;
}
trait ToVal {
    const VALUE: usize;
}
impl ToVal for Zero {
    const VALUE: usize = 0;
}
impl<N: NN + ToVal> ToVal for Next<N> {
    const VALUE: usize = 1 + N::VALUE;
}
