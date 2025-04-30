#![recursion_limit = "2048"]

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

type TestList = macros::list![_1, _9, _6, _3, _5, _7];

#[test]
fn length() {
    assert_eq!(<TestList as Length>::Output::VALUE, 6);
}

#[test]
fn index() {
    // this macro internally uses GetIndex trait
    assert_eq!(macros::list_to_array!(TestList, 0..=5), [1, 9, 6, 3, 5, 7]);
}

#[test]
fn append() {
    // [1, 9, 6, 3, 5, 7, 0]
    type NewList = <TestList as Append<_0>>::Output;

    assert_eq!(<NewList as Length>::Output::VALUE, 7);
    assert_eq!(<NewList as GetIndex<_6>>::Output::VALUE, 0);
}

#[test]
fn pop() {
    // [1, 9, 6, 3, 5]
    type NewList = <TestList as Pop>::Output;
    type PopItem = <TestList as Pop>::Item;

    assert_eq!(<NewList as Length>::Output::VALUE, 5);
    assert_eq!(PopItem::VALUE, 7);
}
