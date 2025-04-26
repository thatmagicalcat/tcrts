#![recursion_limit = "2048"]
#![allow(unused)]

mod number;
mod arithmetic;
mod boolean;
mod binaryop;
mod cmp;

use number::*;
use cmp::*;
use arithmetic::*;
use boolean::*;
use binaryop::*;

fn main() {
    type One = Next<Zero>;
    type Two = Next<One>;
    type Three = Next<Two>;
    type Four = Next<Three>;
    type Five = Next<Four>;

    // just in case
    type Hundred = macros::apply!(Next, Zero, 100);

    println!("4 + 5 = {}", <Four as Add<Five>>::Output::VALUE);
    println!("4 < 5 = {}", <Four as PeanoLt<Five>>::Output::VALUE);
    println!("1 == 1 = {}", <One as PeanoEq<One>>::Output::VALUE);
    println!("1 >= 2 = {}", <One as PeanoGEq<Two>>::Output::VALUE);
}

trait ToVal {
    const VALUE: usize;
}
