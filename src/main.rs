#![recursion_limit = "2048"]
#![allow(unused)]

mod arithmetic;
mod binaryop;
mod boolean;
mod cmp;
mod list;
mod number;

use arithmetic::*;
use binaryop::*;
use boolean::*;
use cmp::*;
use list::*;
use number::*;

fn main() {
    type _0 = Zero;
    type _1 = Next<_0>;
    type _2 = Next<_1>;
    type _3 = Next<_2>;
    type _4 = Next<_3>;
    type _5 = Next<_4>;

    // just in case
    type Hundred = macros::apply!(Next, Zero, 100);

    println!("4 + 5 = {}", <_4 as Add<_5>>::Output::VALUE);
    println!("4 < 5 = {}", <_4 as PeanoLt<_5>>::Output::VALUE);
    println!("1 == 1 = {}", <_1 as PeanoEq<_1>>::Output::VALUE);
    println!("1 >= 2 = {}", <_1 as PeanoGEq<_2>>::Output::VALUE);

    // [0, 1, 2, 5]
    type List = Cons<_0, Cons<_1, Cons<_2, Cons<_5, Nil>>>>;

    println!("List[3] = {}", <List as GetIndex<_3>>::Output::VALUE);
    println!("Length: {}", List::LENGTH);

    // append 4
    // [0, 1, 2, 5, 4]
    type Appended = <List as Append<_4>>::Output;
    println!(
        "Appended List[4] = {}",
        <Appended as GetIndex<_4>>::Output::VALUE
    );
    println!("Appended Length: {}", Appended::LENGTH);
}
