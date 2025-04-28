use super::*;

pub trait GetIndex<Index> {
    type Output;
}

impl<H, T> GetIndex<Zero> for Cons<H, T> {
    type Output = H;
}

impl<H, T, Index> GetIndex<Next<Index>> for Cons<H, T>
where
    Index: Num,
    T: GetIndex<Index>,
{
    type Output = <T as GetIndex<Index>>::Output;
}
