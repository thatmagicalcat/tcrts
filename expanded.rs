#![feature(prelude_import)]
#![recursion_limit = "2048"]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
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
type _50 = Next<
    Next<
        Next<
            Next<
                Next<
                    Next<
                        Next<
                            Next<
                                Next<
                                    Next<
                                        Next<
                                            Next<
                                                Next<
                                                    Next<
                                                        Next<
                                                            Next<
                                                                Next<
                                                                    Next<
                                                                        Next<
                                                                            Next<
                                                                                Next<
                                                                                    Next<
                                                                                        Next<
                                                                                            Next<
                                                                                                Next<
                                                                                                    Next<
                                                                                                        Next<
                                                                                                            Next<
                                                                                                                Next<
                                                                                                                    Next<
                                                                                                                        Next<
                                                                                                                            Next<
                                                                                                                                Next<
                                                                                                                                    Next<
                                                                                                                                        Next<
                                                                                                                                            Next<
                                                                                                                                                Next<
                                                                                                                                                    Next<
                                                                                                                                                        Next<
                                                                                                                                                            Next<
                                                                                                                                                                Next<
                                                                                                                                                                    Next<Next<Next<Next<Next<Next<Next<Next<Next<Zero>>>>>>>>>,
                                                                                                                                                                >,
                                                                                                                                                            >,
                                                                                                                                                        >,
                                                                                                                                                    >,
                                                                                                                                                >,
                                                                                                                                            >,
                                                                                                                                        >,
                                                                                                                                    >,
                                                                                                                                >,
                                                                                                                            >,
                                                                                                                        >,
                                                                                                                    >,
                                                                                                                >,
                                                                                                            >,
                                                                                                        >,
                                                                                                    >,
                                                                                                >,
                                                                                            >,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
struct Shl;
struct Shr;
struct Inc;
struct Dec;
struct LoopStart;
struct LoopEnd;
trait Instr {}
impl Instr for Shl {}
impl Instr for Shr {}
impl Instr for Inc {}
impl Instr for Dec {}
type Cells = Cons<
    _0,
    Cons<
        _0,
        Cons<
            _0,
            Cons<
                _0,
                Cons<_0, Cons<_0, Cons<_0, Cons<_0, Cons<_0, Cons<_0, Cons<_0, Nil>>>>>>>,
            >,
        >,
    >,
>;
trait Interpret<Ptr: Num = Zero, DataArray = Cells> {
    type DataArray;
}
impl<Ptr: Num, DataArray> Interpret<Ptr, DataArray> for Nil {
    type DataArray = DataArray;
}
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Inc, NextInstrs>
where
    Ptr: Num,
    DataArray: GetIndex<Ptr> + GetIndex<_0>
        + Replace<Ptr, Next<<DataArray as tcrts::list::GetIndex<Ptr>>::Output>>,
    NextInstrs: Interpret<
        Ptr,
        <DataArray as tcrts::list::Replace<
            Ptr,
            Next<<DataArray as tcrts::list::GetIndex<Ptr>>::Output>,
        >>::Output,
    >,
{
    type DataArray = <NextInstrs as Interpret<
        Ptr,
        <DataArray as tcrts::list::Replace<
            Ptr,
            Next<<DataArray as tcrts::list::GetIndex<Ptr>>::Output>,
        >>::Output,
    >>::DataArray;
}
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Dec, NextInstrs>
where
    Ptr: Num,
    DataArray: GetIndex<Ptr>
        + Replace<
            Ptr,
            <<DataArray as tcrts::list::GetIndex<
                Ptr,
            >>::Output as tcrts::arithmetic::PeanoSub<_1>>::Output,
        >,
    <DataArray as GetIndex<Ptr>>::Output: PeanoSub<_1>,
    NextInstrs: Interpret<
        Ptr,
        <DataArray as tcrts::list::Replace<
            Ptr,
            <<DataArray as tcrts::list::GetIndex<
                Ptr,
            >>::Output as tcrts::arithmetic::PeanoSub<_1>>::Output,
        >>::Output,
    >,
{
    type DataArray = <NextInstrs as Interpret<
        Ptr,
        <DataArray as tcrts::list::Replace<
            Ptr,
            <<DataArray as tcrts::list::GetIndex<
                Ptr,
            >>::Output as tcrts::arithmetic::PeanoSub<_1>>::Output,
        >>::Output,
    >>::DataArray;
}
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Shr, NextInstrs>
where
    Ptr: Num,
    NextInstrs: Interpret<Next<Ptr>, DataArray>,
{
    type DataArray = <NextInstrs as Interpret<Next<Ptr>, DataArray>>::DataArray;
}
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray> for Cons<Shl, NextInstrs>
where
    Ptr: Num + PeanoSub<_1>,
    NextInstrs: Interpret<<Ptr as tcrts::arithmetic::PeanoSub<_1>>::Output, DataArray>,
{
    type DataArray = <NextInstrs as Interpret<
        <Ptr as tcrts::arithmetic::PeanoSub<_1>>::Output,
        DataArray,
    >>::DataArray;
}
trait Loop<Ptr: Num, ConditionPtr, LoopStartList, DataArray> {
    type DataArray;
}
impl<
    NextInstrs,
    Ptr,
    ConditionPtr,
    LoopStartList,
    DataArray,
> Loop<Ptr, ConditionPtr, LoopStartList, DataArray> for Cons<LoopStart, NextInstrs>
where
    Ptr: Num,
    NextInstrs: Interpret<Ptr, DataArray>,
{
    type DataArray = <NextInstrs as Interpret<Ptr, DataArray>>::DataArray;
}
impl<NextInstrs, Ptr, DataArray> Interpret<Ptr, DataArray>
for Cons<LoopStart, NextInstrs>
where
    Ptr: Num + PeanoSub<_1>,
    NextInstrs: Interpret<Ptr, DataArray>,
{
    type DataArray = <Self as Loop<Ptr, Ptr, Self, DataArray>>::DataArray;
}
fn main() {
    type T = ();
    match (&T::VALUE, &7) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    #[rustfmt::skip]
    type InstructionSet = Cons<
        Inc,
        Cons<
            Inc,
            Cons<
                Inc,
                Cons<
                    Dec,
                    Cons<
                        Shr,
                        Cons<
                            Inc,
                            Cons<Shr, Cons<Inc, Cons<Inc, Cons<Inc, Cons<Dec, Nil>>>>>,
                        >,
                    >,
                >,
            >,
        >,
    >;
    {
        ::std::io::_print(
            format_args!(
                "Output: {0:?}\n",
                [
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Zero,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<tcrts::number::Zero>,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<tcrts::number::Next<tcrts::number::Zero>>,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<tcrts::number::Next<tcrts::number::Zero>>,
                        >,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<
                                tcrts::number::Next<
                                    tcrts::number::Next<tcrts::number::Zero>,
                                >,
                            >,
                        >,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<
                                tcrts::number::Next<
                                    tcrts::number::Next<
                                        tcrts::number::Next<tcrts::number::Zero>,
                                    >,
                                >,
                            >,
                        >,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<
                                tcrts::number::Next<
                                    tcrts::number::Next<
                                        tcrts::number::Next<
                                            tcrts::number::Next<tcrts::number::Zero>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<
                                tcrts::number::Next<
                                    tcrts::number::Next<
                                        tcrts::number::Next<
                                            tcrts::number::Next<
                                                tcrts::number::Next<tcrts::number::Zero>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<
                                tcrts::number::Next<
                                    tcrts::number::Next<
                                        tcrts::number::Next<
                                            tcrts::number::Next<
                                                tcrts::number::Next<
                                                    tcrts::number::Next<tcrts::number::Zero>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<
                                tcrts::number::Next<
                                    tcrts::number::Next<
                                        tcrts::number::Next<
                                            tcrts::number::Next<
                                                tcrts::number::Next<
                                                    tcrts::number::Next<
                                                        tcrts::number::Next<tcrts::number::Zero>,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >>::Output::VALUE,
                    <<InstructionSet as Interpret>::DataArray as tcrts::list::GetIndex<
                        tcrts::number::Next<
                            tcrts::number::Next<
                                tcrts::number::Next<
                                    tcrts::number::Next<
                                        tcrts::number::Next<
                                            tcrts::number::Next<
                                                tcrts::number::Next<
                                                    tcrts::number::Next<
                                                        tcrts::number::Next<
                                                            tcrts::number::Next<tcrts::number::Zero>,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >>::Output::VALUE,
                ],
            ),
        );
    };
}
