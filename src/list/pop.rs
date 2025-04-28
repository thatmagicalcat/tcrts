use super::*;

pub trait Pop {
    /// New list
    type Output;

    /// The popped item
    type Item;
}

// pop [I] = []
impl<I> Pop for Cons<I, Nil> {
    type Output = Nil;
    type Item = I;
}

impl<H, T: Pop> Pop for Cons<H, T> {
    type Output = Cons<H, <T as Pop>::Output>;
    type Item = <T as Pop>::Item;
}
