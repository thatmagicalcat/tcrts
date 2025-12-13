use crate::binaryop::*;
use crate::boolean::*;
use crate::number::*;

pub trait PeanoEq<N> {
    type Output: Boolean;
}

pub trait PeanoNEq<N> {
    type Output: Boolean;
}

pub trait PeanoLt<N> {
    type Output: Boolean;
}

pub trait PeanoLEq<N> {
    type Output: Boolean;
}

pub trait PeanoGt<N> {
    type Output: Boolean;
}

pub trait PeanoGEq<N> {
    type Output: Boolean;
}

impl<M, N> PeanoGt<M> for N
where
    N: Num + PeanoLt<M> + PeanoEq<M>,
    M: Num,
    <N as PeanoLt<M>>::Output: Or<<N as PeanoEq<M>>::Output>,
    <<N as PeanoLt<M>>::Output as Or<<N as PeanoEq<M>>::Output>>::Output: Not,
{
    type Output = <<N as PeanoLEq<M>>::Output as Not>::Output;
}

// x >= y is just !(x < y)
impl<N, M> PeanoGEq<M> for N
where
    N: Num + PeanoLt<M>,
    M: Num,
    <N as PeanoLt<M>>::Output: Not,
{
    type Output = <<N as PeanoLt<M>>::Output as Not>::Output;
}

impl<N, M> PeanoLEq<M> for N
where
    N: Num + PeanoLt<M> + PeanoEq<M>,
    M: Num,
    <N as PeanoLt<M>>::Output: Or<<N as PeanoEq<M>>::Output>,
{
    type Output = <<N as PeanoLt<M>>::Output as Or<<N as PeanoEq<M>>::Output>>::Output;
}

// 0 < 0 = false
impl PeanoLt<Zero> for Zero {
    type Output = False;
}

// 0 < S(x) = true
impl<N: Num> PeanoLt<Next<N>> for Zero {
    type Output = True;
}

// S(x) < 0 = false
impl<N: Num> PeanoLt<Zero> for Next<N> {
    type Output = False;
}

impl<M, N> PeanoLt<Next<M>> for Next<N>
where
    N: Num + PeanoLt<M>,
    M: Num,
{
    type Output = <N as PeanoLt<M>>::Output;
}

impl<N: Num, M: Num> PeanoNEq<N> for M
where
    N: PeanoEq<M>,
    <N as PeanoEq<M>>::Output: Not,
{
    type Output = <<N as PeanoEq<M>>::Output as Not>::Output;
}

// 0 == S(x) = false
impl<N: Num> PeanoEq<Zero> for Next<N> {
    type Output = False;
}

// S(x) == 0 = false
impl<N: Num> PeanoEq<Next<N>> for Zero {
    type Output = False;
}

// 0 == 0 = true
impl PeanoEq<Zero> for Zero {
    type Output = True;
}

impl<N, M> PeanoEq<Next<M>> for Next<N>
where
    N: Num + PeanoEq<M>,
    M: Num,
{
    type Output = <N as PeanoEq<M>>::Output;
}
