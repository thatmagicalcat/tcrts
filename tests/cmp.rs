#![recursion_limit = "2048"]

use ts_abuse::all::*;

type _0 = Zero;
type _1 = Next<_0>;
type _2 = Next<_1>;
type _3 = Next<_2>;

#[test]
fn to_val() {
    assert_eq!(<_0 as ToVal>::VALUE, 0);
    assert_eq!(<_1 as ToVal>::VALUE, 1);
    assert_eq!(<_2 as ToVal>::VALUE, 2);
    assert_eq!(<_3 as ToVal>::VALUE, 3);
}

#[test]
fn peano_eq() {
    type Eq0_0 = <_0 as PeanoEq<_0>>::Output;
    type Eq1_1 = <_1 as PeanoEq<_1>>::Output;
    type Eq0_1 = <_0 as PeanoEq<_1>>::Output;

    assert_eq!(Eq0_0::VALUE, True::VALUE);
    assert_eq!(Eq1_1::VALUE, True::VALUE);
    assert_eq!(Eq0_1::VALUE, False::VALUE);
}

#[test]
fn peano_lt() {
    type Lt0_1 = <_0 as PeanoLt<_1>>::Output;
    type Lt1_2 = <_1 as PeanoLt<_2>>::Output;
    type Lt2_1 = <_2 as PeanoLt<_1>>::Output;

    assert_eq!(Lt0_1::VALUE, True::VALUE);
    assert_eq!(Lt1_2::VALUE, True::VALUE);
    assert_eq!(Lt2_1::VALUE, False::VALUE);
}

#[test]
fn peano_gt() {
    type Gt0_1 = <_0 as PeanoGt<_1>>::Output;
    type Gt1_0 = <_1 as PeanoGt<_0>>::Output;
    type Gt2_1 = <_2 as PeanoGt<_1>>::Output;

    assert_eq!(Gt0_1::VALUE, False::VALUE);
    assert_eq!(Gt1_0::VALUE, True::VALUE);
    assert_eq!(Gt2_1::VALUE, True::VALUE);
}

#[test]
fn peano_geq() {
    type Geq0_0 = <_0 as PeanoGEq<_0>>::Output;
    type Geq1_0 = <_1 as PeanoGEq<_0>>::Output;
    type Geq1_2 = <_1 as PeanoGEq<_2>>::Output;

    assert_eq!(Geq0_0::VALUE, True::VALUE);
    assert_eq!(Geq1_0::VALUE, True::VALUE);
    assert_eq!(Geq1_2::VALUE, False::VALUE);
}

#[test]
fn peano_leq() {
    type Leq0_0 = <_0 as PeanoLEq<_0>>::Output;
    type Leq0_1 = <_0 as PeanoLEq<_1>>::Output;
    type Leq2_1 = <_2 as PeanoLEq<_1>>::Output;

    assert_eq!(Leq0_0::VALUE, True::VALUE);
    assert_eq!(Leq0_1::VALUE, True::VALUE);
    assert_eq!(Leq2_1::VALUE, False::VALUE);
}
