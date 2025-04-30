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
