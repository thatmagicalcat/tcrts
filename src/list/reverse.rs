use super::*;

pub trait Reverse {
    type Output;
}

pub trait ReverseHelper<Acc> {
    type Output;
}

impl<L: ReverseHelper<Nil>> Reverse for L {
    type Output = <L as ReverseHelper<Nil>>::Output;
}

impl<Acc> ReverseHelper<Acc> for Nil {
    type Output = Acc;
}

impl<H, T, Acc> ReverseHelper<Acc> for Cons<H, T>
where
    T: ReverseHelper<Cons<H, Acc>>,
{
    type Output = <T as ReverseHelper<Cons<H, Acc>>>::Output;
}
