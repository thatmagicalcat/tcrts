use crate::value::ToVal;

/// A trait to represent a boolean value at the type level.
/// This is implemented by the `True` and `False` types
pub trait Boolean {}

/// A type to represent the boolean value `true` at the type level.
pub struct True;

/// A type to represent the boolean value `false` at the type level.
pub struct False;

impl Boolean for True {}
impl Boolean for False {}

impl ToVal for True {
    const VALUE: usize = 1;
}

impl ToVal for False {
    const VALUE: usize = 0;
}
