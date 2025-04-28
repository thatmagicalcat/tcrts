use super::*;

pub trait Map<F> {
    type Output;
}

impl<N: Num, F: TypeFn<N>> Map<F> for N {
    type Output = <F as TypeFn<N>>::Output;
}

impl<F> Map<F> for Nil {
    type Output = Nil;
}

impl<H, T, F> Map<F> for Cons<H, T>
where
    F: TypeFn<H>,
    T: Map<F>,
{
    type Output = Cons<<F as TypeFn<H>>::Output, <T as Map<F>>::Output>;
}
