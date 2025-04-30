/// Type level function trait
///
/// # Example
/// A type level function to square a number
/// ```rust
/// use tcrts::all::*;
/// 
/// struct SquareFn;
/// impl <N: Num + PeanoMul<N>> TypeFn<N> for SquareFn {
///     type Output = <N as PeanoMul<N>>::Output;
/// }
///
/// // usage
/// type _2 = Next<Next<Zero>>;
/// type _4 = <SquareFn as TypeFn<_2>>::Output;
/// assert_eq!(_4::VALUE, 4);
/// ```
pub trait TypeFn<Input> {
    type Output;
}
