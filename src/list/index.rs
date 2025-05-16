use super::*;

pub trait FirstItem {
    type Output;
}

pub trait LastItem {
    type Output;
}

impl<H, T> FirstItem for Cons<H, T> {
    type Output = H;
}

impl <H> LastItem for Cons<H, Nil> {
    type Output = H;
}

impl<H, T> LastItem for Cons<H, T>
where
    T: LastItem,
{
    type Output = <T as LastItem>::Output;
}

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
