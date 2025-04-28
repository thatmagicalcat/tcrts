use super::*;

pub trait Append<Item> {
    type Output;
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
