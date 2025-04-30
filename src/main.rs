#![recursion_limit = "2048"]
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

type _50 = macros::apply!(Next, Zero, 50);

struct Shl;
struct Shr;
struct Inc;
struct Dec;

trait Instr {}

impl Instr for Shl {}
impl Instr for Shr {}
impl Instr for Inc {}
impl Instr for Dec {}

type Cells = macros::list![_0, _0, _0, _0, _0, _0, _0, _0, _0, _0, _0]; // 11
trait Interpret<Ptr: Num = Zero, DataArray = Cells> {
    type DataArray;
}

impl<Ptr: Num, DataArray> Interpret<Ptr, DataArray> for Nil {
    type DataArray = DataArray;
}

// + instruction
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Inc, NextInstrs>
where
    Ptr: Num,
    DataArray: GetIndex<Ptr> + GetIndex<_0> + Replace<Ptr, Next<tcrts::index!(DataArray, Ptr)>>,
    NextInstrs:
        Interpret<Ptr, tcrts::replace!(DataArray, Ptr, Next<tcrts::index!(DataArray, Ptr)>)>,
{
    type DataArray = <NextInstrs as Interpret<
        Ptr,
        tcrts::replace!(DataArray, Ptr, Next<tcrts::index!(DataArray, Ptr)>),
    >>::DataArray;
}

// - instruction
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Dec, NextInstrs>
where
    Ptr: Num,
    DataArray: GetIndex<Ptr> + Replace<Ptr, tcrts::subtract!(tcrts::index!(DataArray, Ptr), _1)>,
    <DataArray as GetIndex<Ptr>>::Output: PeanoSub<_1>,
    NextInstrs: Interpret<
            Ptr,
            tcrts::replace!(
                DataArray,
                Ptr,
                tcrts::subtract!(tcrts::index!(DataArray, Ptr), _1)
            ),
        >,
{
    type DataArray = <NextInstrs as Interpret<
        Ptr,
        tcrts::replace!(
            DataArray,
            Ptr,
            tcrts::subtract!(tcrts::index!(DataArray, Ptr), _1)
        ),
    >>::DataArray;
}

// > instruction
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Shr, NextInstrs>
where
    Ptr: Num,
    NextInstrs: Interpret<Next<Ptr>, DataArray>,
{
    type DataArray = <NextInstrs as Interpret<Next<Ptr>, DataArray>>::DataArray;
}

// < instruction
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Shl, NextInstrs>
where
    Ptr: Num + PeanoSub<_1>,
    NextInstrs: Interpret<tcrts::subtract!(Ptr, _1), DataArray>,
{
    type DataArray = <NextInstrs as Interpret<tcrts::subtract!(Ptr, _1), DataArray>>::DataArray;
}

fn main() {
    #[rustfmt::skip]
    type InstructionSet = macros::list![
    //  +++->
        Inc, Inc, Inc, Dec, Shr,
    //  +>
        Inc, Shr,
    //  +++-
        Inc, Inc, Inc, Dec
    ];

    // output should be [2, 1, 2, ..]

    println!(
        "Output: {:?}",
        macros::list_to_array!(<InstructionSet as Interpret>::DataArray, 0..11)
    );
}

// struct Square;
// impl<N: Num + tcrts::arithmetic::PeanoMul<N>> TypeFn<N> for Square {
//     type Output = <N as PeanoMul<N>>::Output;
// }

// struct LessThan50;
// impl<N> TypeFn<N> for LessThan50
// where
//     N: Num + PeanoLt<_50>,
// {
//     type Output = <N as PeanoLt<_50>>::Output;
// }

// struct GetLeft;
// impl<L, R> TypeFn<Pair<L, R>> for GetLeft {
//     type Output = L;
// }

// struct GetRight;
// impl<L, R> TypeFn<Pair<L, R>> for GetRight {
//     type Output = R;
// }

// fn main() {
//     type List = macros::list![_0, _1, _2, _3, _4, _5, _6, _7, _8, _9];

//     // square all elements
//     type ListSq = <List as Map<Square>>::Output;

//     println!("         list: {:?}", macros::list_to_array!(List, 0..=9));
//     println!("squared  list: {:?}", macros::list_to_array!(ListSq, 0..=9));

//     type FilteredList = <ListSq as Filter<LessThan50>>::Output;
//     println!(
//         "filtered list: {:?}",
//         macros::list_to_array!(FilteredList, 0..=7) // two items removed
//     );

//     type List2 = macros::list![_1, _1, _1, _9];
//     type List2Length = <List2 as Length>::Output;

//     #[rustfmt::skip]
//     type Last = <
//         List2 as GetIndex<
//             <
//                 <List2 as Length>::Output
//                 as PeanoSub<_1> // length - 1
//             >::Output
//         >
//     >::Output;

//     println!("Last element: {}", Last::VALUE);

//     type Enumerated = <List2 as Enumerate>::Output;

//     type LeftEnumerated = <Enumerated as Map<GetLeft>>::Output;
//     println!("left: {:?}", macros::list_to_array!(LeftEnumerated, 0..=3));

//     type RightEnumerated = <Enumerated as Map<GetRight>>::Output;
//     println!(
//         "right: {:?}",
//         macros::list_to_array!(RightEnumerated, 0..=3)
//     );
// }
