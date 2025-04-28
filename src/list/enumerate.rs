use super::*;

pub trait Enumerate<Start = Zero> {
    type Output;
}

impl<H, T, I> Enumerate<I> for Cons<H, T>
where
    T: Enumerate<Next<I>>,
{
    type Output = Cons<Pair<I, H>, <T as Enumerate<Next<I>>>::Output>;
}

impl<I> Enumerate<I> for Nil {
    type Output = Nil;
}
