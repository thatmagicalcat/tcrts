/// A trait to represent a value at the type level.
/// This is implemented by the `Zero` and `Next` types
/// provide a way to convert Peano numbers to their integer values.
pub trait ToVal {
    const VALUE: usize;
}
