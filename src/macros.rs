#[macro_export]
macro_rules! eq {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::cmp::PeanoEq<$b>>::Output
    };
}

#[macro_export]
macro_rules! neq {
    [ $a:ty, $b:ty ] => {
        <<$a as tcrts::cmp::PeanoEq<$b>>::Output as tcrts::binaryop::Not>::Output
    };
}

#[macro_export]
macro_rules! lt {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::cmp::PeanoLt<$b>>::Output
    };
}

#[macro_export]
macro_rules! leq {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::cmp::PeanoLEq<$b>>::Output
    };
}

#[macro_export]
macro_rules! gt {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::cmp::PeanoGt<$b>>::Output
    };
}

#[macro_export]
macro_rules! geq {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::cmp::PeanoGEq<$b>>::Output
    };
}

#[macro_export]
macro_rules! add {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::arithmetic::PeanoAdd<$b>>::Output
    };
}

#[macro_export]
macro_rules! subtract {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::arithmetic::PeanoSub<$b>>::Output
    };
}

#[macro_export]
macro_rules! mul {
    [ $a:ty, $b:ty ] => {
        <$a as tcrts::arithmetic::PeanoMul<$b>>::Output
    };
}

#[macro_export]
macro_rules! index {
    [ $list:ty, $index:ty ] => {
        <$list as tcrts::list::GetIndex<$index>>::Output
    };
}

#[macro_export]
macro_rules! replace {
    [ $list:ty, $index:ty, $value:ty ] => {
        <$list as tcrts::list::Replace<$index, $value>>::Output
    };
}

#[macro_export]
macro_rules! condition {
    [ $condition:ty => $truety:ty | $falsety:ty ] => {
        <$condition as tcrts::conditional::Conditional<$truety, $falsety>>::Output
    };
}
