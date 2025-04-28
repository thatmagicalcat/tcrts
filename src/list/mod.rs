pub mod append;
pub mod filter;
pub mod index;
pub mod length;
pub mod map;
pub mod pop;

pub mod all {
    pub use super::append::*;
    pub use super::filter::*;
    pub use super::index::*;
    pub use super::length::*;
    pub use super::map::*;
    pub use super::pop::*;
    pub use super::*;
}

use crate::all::*;

pub trait List {}

pub struct Cons<H, T>(std::marker::PhantomData<(H, T)>);
pub struct Nil;

impl<H, T> List for Cons<H, T> {}
impl List for Nil {}
