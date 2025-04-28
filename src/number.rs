use crate::value::ToVal;

/// A trait to represent a number in Peano arithmetic.
/// Zero is the base case for the recursive definition of Peano numbers.
/// All other numbers can be represented by wrapping `Zero` in `Next` type
/// that many times.
/// 
/// # Example
/// ```rust
/// use ts_abuse::all::*;
/// 
/// type _0 = Zero;
/// type _1 = Next<_0>;
/// type _2 = Next<_1>;
/// 
/// assert_eq!(_0::VALUE, 0);
/// assert_eq!(_1::VALUE, 1);
/// assert_eq!(_2::VALUE, 2);
/// ```
pub trait Num {}

/// Smallest number in Peano arithmetic, zero.
/// Read `Num` trait's documentation for more details.
pub struct Zero;

/// A wrapper type to represent the successor of a number in Peano arithmetic.
/// Read `Num` trait's documentation for more details.
pub struct Next<N>(std::marker::PhantomData<N>);

impl Num for Zero {}
impl<N: Num> Num for Next<N> {}

impl ToVal for Zero {
    const VALUE: usize = 0;
}

impl<N: Num + ToVal> ToVal for Next<N> {
    const VALUE: usize = 1 + N::VALUE;
}
