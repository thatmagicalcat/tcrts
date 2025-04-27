use crate::value::ToVal;

pub trait Num {}

pub struct Zero;
pub struct Next<N>(std::marker::PhantomData<N>);

impl Num for Zero {}
impl<N: Num> Num for Next<N> {}

impl ToVal for Zero {
    const VALUE: usize = 0;
}

impl<N: Num + ToVal> ToVal for Next<N> {
    const VALUE: usize = 1 + N::VALUE;
}
