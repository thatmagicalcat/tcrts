use crate::value::ToVal;

pub trait Boolean {}
pub struct True;
pub struct False;

impl Boolean for True {}
impl Boolean for False {}

impl ToVal for True {
    const VALUE: usize = 1;
}

impl ToVal for False {
    const VALUE: usize = 0;
}
