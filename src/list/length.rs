use super::*;

type _1 = Next<Zero>;

pub trait Length {
    type Output;
}

impl<H, T> Length for Cons<H, T>
where
    T: Length,
    <T as Length>::Output: Num,
{
    type Output = <<T as Length>::Output as PeanoAdd<_1>>::Output;
}

impl Length for Nil {
    type Output = Zero;
}
