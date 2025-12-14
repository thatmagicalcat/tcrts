mod append;
mod enumerate;
mod filter;
mod index;
mod length;
mod map;
mod pop;
mod remove;
mod replace;
mod reverse;

pub use append::*;
pub use enumerate::*;
pub use filter::*;
pub use index::*;
pub use length::*;
pub use map::*;
pub use pop::*;
pub use remove::*;
pub use replace::*;
pub use reverse::*;

use crate::all::*;

pub trait List {}

pub struct Cons<H, T>(std::marker::PhantomData<(H, T)>);
pub struct Nil;

impl<H, T> List for Cons<H, T> {}
impl List for Nil {}
