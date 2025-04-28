use super::*;

pub trait Length {
    const LENGTH: usize;
}

impl<H, T: Length> Length for Cons<H, T> {
    const LENGTH: usize = 1 + T::LENGTH;
}

impl Length for Nil {
    const LENGTH: usize = 0;
}

