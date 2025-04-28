use super::*;

pub trait Filter<F> {
    type Output;
}

pub trait FilterInternal<C: Boolean, F> {
    type Output;
}

impl<H, T, F> FilterInternal<True, F> for Cons<H, T>
where
    T: List + Filter<F>,
{
    type Output = Cons<H, <T as Filter<F>>::Output>;
}

impl<H, T, F> FilterInternal<False, F> for Cons<H, T>
where
    T: List + Filter<F>,
    F: TypeFn<H>,
{
    type Output = <T as Filter<F>>::Output;
}

impl<F> Filter<F> for Nil {
    type Output = Nil;
}

impl<C: Boolean, F> FilterInternal<C, F> for Nil {
    type Output = Nil;
}

impl<H, T, F> Filter<F> for Cons<H, T>
where
    F: TypeFn<H>,
    Cons<H, T>: FilterInternal<<F as TypeFn<H>>::Output, F>,
    <F as TypeFn<H>>::Output: Boolean,
{
    type Output = <Self as FilterInternal<<F as TypeFn<H>>::Output, F>>::Output;
}
