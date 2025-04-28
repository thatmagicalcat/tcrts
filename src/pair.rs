pub struct Pair<Left, Right>(std::marker::PhantomData<(Left, Right)>);

pub trait Left {
    type Output;
}

pub trait Right {
    type Output;
}

impl<A, B> Left for Pair<A, B> {
    type Output = A;
}

impl<A, B> Right for Pair<A, B> {
    type Output = B;
}
