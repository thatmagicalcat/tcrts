use super::number::*;

pub trait Add<N> {
    type Output: Num;
}

pub trait Mul<N> {
    type Output;
}

// N * 0 = 0
impl<N: Num> Mul<Zero> for N {
    type Output = Zero;
}

impl<N, M> Mul<Next<M>> for N
where
    N: Num + Mul<M> + Add<<N as Mul<M>>::Output>,
    M: Num,
{
    type Output = <N as Add<<N as Mul<M>>::Output>>::Output;
}

// Base case, N + Zero = N
impl<N: Num> Add<Zero> for N {
    type Output = N;
}

// N + Next<M> = Next<N + Next<M - 1>>
impl<N, M> Add<Next<M>> for N
where
    N: Num + Add<M>,
    M: Num,
{
    type Output = Next<<N as Add<M>>::Output>;
}
