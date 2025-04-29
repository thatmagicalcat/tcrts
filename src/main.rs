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
impl<N: Num + ts_abuse::arithmetic::PeanoMul<N>> TypeFn<N> for Square {
    type Output = <N as PeanoMul<N>>::Output;
}

struct LessThan50;
impl<N> TypeFn<N> for LessThan50
where
    N: Num + PeanoLt<_50>,
{
    type Output = <N as PeanoLt<_50>>::Output;
}

struct GetLeft;
impl<L, R> TypeFn<Pair<L, R>> for GetLeft {
    type Output = L;
}

struct GetRight;
impl<L, R> TypeFn<Pair<L, R>> for GetRight {
    type Output = R;
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

    type List2 = macros::list![_1, _1, _1, _9];
    type List2Length = <List2 as Length>::Output;

    #[rustfmt::skip]
    type Last = <
        List2 as GetIndex<
            <
                <List2 as Length>::Output
                as PeanoSub<_1> // length - 1
            >::Output
        >
    >::Output;

    println!("Last element: {}", Last::VALUE);

    type Enumerated = <List2 as Enumerate>::Output;

    type LeftEnumerated = <Enumerated as Map<GetLeft>>::Output;
    println!("left: {:?}", macros::list_to_array!(LeftEnumerated, 0..=3));

    type RightEnumerated = <Enumerated as Map<GetRight>>::Output;
    println!(
        "right: {:?}",
        macros::list_to_array!(RightEnumerated, 0..=3)
    );
}
