use crate::number::*;

pub trait Length {
    const LENGTH: usize;
}

pub trait GetIndex<Index> {
    type Output;
}

pub trait Append<Item> {
    type Output;
}

pub struct Cons<H, T>(std::marker::PhantomData<(H, T)>);
pub struct Nil;

impl Length for Nil {
    const LENGTH: usize = 0;
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

impl<H, T, Item> Append<Item> for Cons<H, T>
where
    T: Append<Item>,
{
    type Output = Cons<H, <T as Append<Item>>::Output>;
}

// Appending to Nil will return a list with the item
impl<Item> Append<Item> for Nil {
    type Output = Cons<Item, Nil>;
}

impl<H, T: Length> Length for Cons<H, T> {
    const LENGTH: usize = 1 + T::LENGTH;
}
