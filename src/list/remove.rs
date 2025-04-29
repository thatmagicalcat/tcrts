use super::*;

pub trait Remove<Index> {
    type Output;
}

impl<H, T> Remove<Zero> for Cons<H, T> {
    type Output = T;
}

impl<H, T, Index> Remove<Next<Index>> for Cons<H, T>
where
    Index: Num,
    T: Remove<Index>,
{
    type Output = Cons<H, <T as Remove<Index>>::Output>;
}
