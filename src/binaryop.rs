use crate::boolean::*;

pub trait Not {
    type Output: Boolean;
}

pub trait And<B: Boolean> {
    type Output: Boolean;
}

pub trait Or<B: Boolean> {
    type Output: Boolean;
}

pub trait Xor<B: Boolean> {
    type Output: Boolean;
}

impl And<False> for False {
    type Output = False;
}

impl And<True> for False {
    type Output = False;
}

impl And<False> for True {
    type Output = False;
}

impl And<True> for True {
    type Output = True;
}

impl Or<False> for False {
    type Output = False;
}

impl Or<True> for False {
    type Output = True;
}

impl Or<False> for True {
    type Output = True;
}

impl Or<True> for True {
    type Output = True;
}

impl Not for True {
    type Output = False;
}

impl Not for False {
    type Output = True;
}

impl Xor<False> for False {
    type Output = False;
}

impl Xor<True> for True {
    type Output = False;
}

impl Xor<False> for True {
    type Output = True;
}

impl Xor<True> for False {
    type Output = True;
}
