use crate::boolean::*;

pub trait Conditional<TrueTy, FalseTy> {
    type Output;
}

impl<TrueTy, FalseTy> Conditional<TrueTy, FalseTy> for True {
    type Output = TrueTy;
}

impl<TrueTy, FalseTy> Conditional<TrueTy, FalseTy> for False {
    type Output = FalseTy;
}
