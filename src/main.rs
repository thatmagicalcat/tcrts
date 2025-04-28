#![recursion_limit = "2048"]
#![allow(unused)]

use ts_abuse::all::*;

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

type _50 = macros::apply!(Next, Zero, 50);

struct Square;
impl<N: Num + PeanoMul<N>> TypeFn<N> for Square {
    type Output = <N as PeanoMul<N>>::Output;
}

struct LessThan50;
impl<N> TypeFn<N> for LessThan50
where
    N: Num + PeanoLt<_50>,
{
    type Output = <N as PeanoLt<_50>>::Output;
}

fn main() {
    type List = macros::list![_0, _1, _2, _3, _4, _5, _6, _7, _8, _9];

    // square all elements
    type ListSq = <List as Map<Square>>::Output;

    println!("         list: {:?}", macros::list_to_array!(List, 0..=9));
    println!("squared  list: {:?}", macros::list_to_array!(ListSq, 0..=9));

    type FilteredList = <ListSq as Filter<LessThan50>>::Output;
    println!(
        "filtered list: {:?}",
        macros::list_to_array!(FilteredList, 0..=7) // two items removed
    );
}
