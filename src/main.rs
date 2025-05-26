#![recursion_limit = "1024"]
#![allow(unused)]

use tcrts::all::*;

type _0 = Zero;
type _1 = Next<_0>;
type _2 = Next<_1>;
type _3 = Next<_2>;
type _4 = Next<_3>;
type _5 = Next<_4>;
type _6 = Next<_5>;
type _7 = Next<_6>;
type _8 = Next<_7>;
type _9 = Next<_8>;

struct GreaterThanFive;
impl<N> TypeFn<N> for GreaterThanFive
where
    N: Num + PeanoLt<_5> + PeanoEq<_5>,
    <N as PeanoLt<_5>>::Output: Or<<N as PeanoEq<_5>>::Output>,
    <<N as PeanoLt<_5>>::Output as Or<<N as PeanoEq<_5>>::Output>>::Output: Not,
{
    type Output = tcrts::gt!(N, _5);
}

struct Square;
impl<N: Num + PeanoMul<N>> TypeFn<N> for Square {
    type Output = <N as PeanoMul<N>>::Output;
}

fn main() {
    type Product = <_3 as PeanoMul<_3>>::Output;
    assert_eq!(Product::VALUE, _9::VALUE);

    type List = macros::list![_2, _4, _3, _6, _8, _9];
    type Filtered = <List as Filter<GreaterThanFive>>::Output;
    type FilterMap = <Filtered as Map<Square>>::Output;

    println!("{:?}", macros::list_to_array!(FilterMap, 0..=2));
}
