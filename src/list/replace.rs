use super::*;

pub trait Replace<Index, Value> {
    type Output;
}

impl<H, T, Value> Replace<Zero, Value> for Cons<H, T> {
    type Output = Cons<Value, T>;
}

impl<H, T, Index, Value> Replace<Next<Index>, Value> for Cons<H, T>
where
    Index: Num,
    T: Replace<Index, Value>,
{
    type Output = Cons<H, <T as Replace<Index, Value>>::Output>;
}
