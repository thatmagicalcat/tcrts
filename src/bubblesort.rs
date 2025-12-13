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

trait Max<Rhs> {
    type Output;
}

trait Min<Rhs> {
    type Output;
}

impl<Lhs, Rhs> Max<Rhs> for Lhs
where
    Lhs: Num + PeanoGt<Rhs>,
    Rhs: Num,
    <Lhs as PeanoGt<Rhs>>::Output: Conditional<Lhs, Rhs>,
{
    type Output = tcrts::condition! {
        tcrts::gt!(Lhs, Rhs) => Lhs | Rhs
    };
}

impl<Lhs, Rhs> Min<Rhs> for Lhs
where
    Lhs: Num + PeanoLt<Rhs>,
    Rhs: Num,
    <Lhs as PeanoLt<Rhs>>::Output: Conditional<Lhs, Rhs>,
{
    type Output = tcrts::condition! {
        tcrts::lt!(Lhs, Rhs) => Lhs | Rhs
    };
}

trait BubblePass {
    type Output;
}

// base case #1 (empty list)
impl BubblePass for Nil {
    type Output = Nil;
}

// base case #2 (list with one item)
// no sorting is required
impl<A> BubblePass for Cons<A, Nil> {
    type Output = Cons<A, Nil>;
}

// recursive step
// formula:
// NewList = Cons(
//     Min(A, B),
//     BubbleSort(
//         Cons(
//             Max(A, B),
//             Tail
//         )
//     )
// )
impl<A, B, Tail> BubblePass for Cons<A, Cons<B, Tail>>
where
    A: Num + PeanoLt<B> + PeanoEq<B>, B: Num,
    <A as PeanoLt<B>>::Output: Conditional<A, B>,
    <A as PeanoLt<B>>::Output:
        Or<<A as PeanoEq<B>>::Output>,
    <<A as PeanoLt<B>>::Output as Or<
        <A as PeanoEq<B>>::Output,
    >>::Output: Not,
    <<<A as PeanoLt<B>>::Output as Or<
        <A as PeanoEq<B>>::Output,
    >>::Output as Not>::Output: Conditional<A, B>, Cons<<<<<A as PeanoLt<B>>::Output as Or<<A as PeanoEq<B>>::Output>>::Output as Not>::Output as Conditional<A, B>>::Output, Tail>: BubblePass
{
    type Output = Cons<
        <A as Min<B>>::Output,
        <Cons<<A as Max<B>>::Output, Tail> as BubblePass>::Output
    >;
}

// if Count = 0, stop
trait BubbleSort<Count> {
    type Output;
}

impl<Count, L> BubbleSort<Next<Count>> for L
where
    Count: Num,
    L: List + BubblePass,
    // run the pass again with count - 1
    <L as BubblePass>::Output: BubbleSort<Count>,
{
    type Output = <<L as BubblePass>::Output as BubbleSort<Count>>::Output;
}

impl <L: List> BubbleSort<Zero> for L {
    type Output = L;
}

pub fn main() {
    type List = macros::list![_9, _5, _2, _3, _8, _1, _0];
    type N = <List as Length>::Output;
    type SortedList = <List as BubbleSort<N>>::Output;
    println!("{:?}", macros::list_to_array!(SortedList, 0..=6));
}

fn print_ty<T>() {
    println!("{}", std::any::type_name::<T>());
}
