use super::number::*;

pub trait PeanoAdd<N> {
    type Output: Num;
}

pub trait PeanoSub<N> {
    type Output: Num;
}

pub trait PeanoMul<N> {
    type Output;
}

// N * 0 = 0
impl<N: Num> PeanoMul<Zero> for N {
    type Output = Zero;
}

impl<N, M> PeanoMul<Next<M>> for N
where
    N: Num + PeanoMul<M> + PeanoAdd<<N as PeanoMul<M>>::Output>,
    M: Num,
{
    type Output = <N as PeanoAdd<<N as PeanoMul<M>>::Output>>::Output;
}

// Base case, N + Zero = N
impl<N: Num> PeanoAdd<Zero> for N {
    type Output = N;
}

// N + Next<M> = Next<N + Next<M - 1>>
impl<N, M> PeanoAdd<Next<M>> for N
where
    N: Num + PeanoAdd<M>,
    M: Num,
{
    type Output = Next<<N as PeanoAdd<M>>::Output>;
}

// N - 0 = N
impl<N: Num> PeanoSub<Zero> for N {
    type Output = N;
}

// Next<N> - Next<M> = N - M
impl<N, M> PeanoSub<Next<M>> for Next<N>
where
    N: Num + PeanoSub<M>,
    M: Num,
{
    type Output = <N as PeanoSub<M>>::Output;
}
