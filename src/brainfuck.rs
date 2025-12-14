use std::{any::type_name, marker::PhantomData, ops::Rem, process::Output};

use tcrts::all::*;
use tcrts_macros as macros;

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

struct TkInc;
struct TkDec;
struct TkShl;
struct TkShr;
struct TkLoopStart;
struct TkLoopEnd;

struct OpInc;
struct OpDec;
struct OpShl;
struct OpShr;
struct OpLoop<Body>(PhantomData<Body>);

trait Token {}
trait OpCode {}

impl Token for TkInc {}
impl Token for TkDec {}
impl Token for TkShl {}
impl Token for TkShr {}
impl Token for TkLoopStart {}
impl Token for TkLoopEnd {}

impl OpCode for OpInc {}
impl OpCode for OpDec {}
impl OpCode for OpShl {}
impl OpCode for OpShr {}
impl<Body> OpCode for OpLoop<Body> {}

trait Parser {
    type Ast;
    type Remainder;
}

impl Parser for Nil {
    type Ast = Nil;
    type Remainder = Nil;
}

impl<T: Parser> Parser for Cons<TkInc, T> {
    type Ast = Cons<OpInc, <T as Parser>::Ast>;
    type Remainder = <T as Parser>::Remainder;
}

impl<T: Parser> Parser for Cons<TkDec, T> {
    type Ast = Cons<OpDec, <T as Parser>::Ast>;
    type Remainder = <T as Parser>::Remainder;
}

impl<T: Parser> Parser for Cons<TkShl, T> {
    type Ast = Cons<OpShl, <T as Parser>::Ast>;
    type Remainder = <T as Parser>::Remainder;
}

impl<T: Parser> Parser for Cons<TkShr, T> {
    type Ast = Cons<OpShr, <T as Parser>::Ast>;
    type Remainder = <T as Parser>::Remainder;
}

impl<T> Parser for Cons<TkLoopEnd, T> {
    type Ast = Nil;
    type Remainder = T;
}

impl<T> Parser for Cons<TkLoopStart, T>
where
    T: Parser,
    <T as Parser>::Remainder: Parser,
{
    type Ast = Cons<OpLoop<<T as Parser>::Ast>, <<T as Parser>::Remainder as Parser>::Ast>;
    type Remainder = <<T as Parser>::Remainder as Parser>::Remainder;
}

struct State<L, C, R>(PhantomData<(L, C, R)>);
type InitialState = State<Nil, Zero, Nil>;

trait SafePop {
    type Head;
    type Tail;
}

impl SafePop for Nil {
    type Head = Zero;
    type Tail = Nil;
}

impl<H, T> SafePop for Cons<H, T> {
    type Head = H;
    type Tail = T;
}

trait Execute<InState> {
    type OutState;
}

// base case
impl<In> Execute<In> for Nil {
    type OutState = In;
}

// recursive step
impl<Op, In, Tail> Execute<In> for Cons<Op, Tail>
where
    Op: Execute<In>,
    Tail: Execute<Op::OutState>,
{
    type OutState = <Tail as Execute<Op::OutState>>::OutState;
}

// Input: State<L, C, R>

// +
// Output: State(L, C + 1, R)
impl<L, C: Num, R> Execute<State<L, C, R>> for OpInc {
    type OutState = State<L, Next<C>, R>;
}

// -
// Output: State(L, C - 1, R)
impl<L, C: Num, R> Execute<State<L, Next<C>, R>> for OpDec {
    type OutState = State<L, C, R>;
}

// >
// Output: State(C ++ L, R.head, R.tail)
impl<L, C, R> Execute<State<L, C, R>> for OpShr
where
    C: Num,
    R: SafePop,
{
    type OutState = State<Cons<C, L>, R::Head, R::Tail>;
}

// <
// Output: State(L.tail, L.head, C ++ R)
impl<L, C, R> Execute<State<L, C, R>> for OpShl
where
    C: Num,
    L: SafePop,
{
    type OutState = State<L::Tail, L::Head, Cons<C, R>>;
}

trait MergeState {
    type Output;
}

pub fn main() {
    type Input = macros::list![
        _1, _2, _3, _4,
        _5 // TkInc, TkShr, TkInc, TkInc, TkDec,
           // TkShl,
           // TkLoopStart,
           // TkDec,
           // TkShl,
           // TkInc,
           // TkShr,
           // TkLoopEnd
    ];

    type Rev = <Input as Reverse>::Output;
    println!("{:?}", <Rev as Array>::OUTPUT);

    // type Instructions = <Input as Parser>::Ast;
    // type OutputState = <Instructions as Execute<InitialState>>::OutState;
    //
    // println!("{}", type_name::<OutputState>());
}
