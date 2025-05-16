use crate::boolean::*;
use crate::cmp;
use crate::conditional::*;

/// Type level nand operation.
pub trait NAnd<B: Boolean> {
    type Output: Boolean;
}

impl<B1, B2> NAnd<B1> for B2
where
    B1: Boolean,
    B2: Boolean,
    <B1 as Boolean>::Value: cmp::PeanoEq<B2>,
    <B1 as Boolean>::Value: cmp::PeanoEq<<B2 as Boolean>::Value>,
    <<B1 as Boolean>::Value as cmp::PeanoEq<<B2 as Boolean>::Value>>::Output:
        Conditional<False, True>,
    <<<B1 as Boolean>::Value as cmp::PeanoEq<<B2 as Boolean>::Value>>::Output as Conditional<
        False,
        True,
    >>::Output: Boolean,
{
    type Output = crate::condition!(
        <B1::Value as cmp::PeanoEq<B2::Value>>::Output => False | True
    );
}

// impl NAnd<False> for False {
//     type Output = True;
// }

// impl NAnd<True> for False {
//     type Output = True;
// }

// impl NAnd<False> for True {
//     type Output = True;
// }

// impl NAnd<True> for True {
//     type Output = False;
// }

/// Type level binay not operation.
///
/// # Example
/// ```rust
/// use tcrts::all::*;
///
/// assert_eq!(True::VALUE, <False as Not>::Output::VALUE);
/// ```
pub trait Not {
    type Output: Boolean;
}

/// Type level binary and operation.
///
/// # Example
/// ```rust
/// use tcrts::all::*;
///
/// assert_eq!(True::VALUE, <True as And<True>>::Output::VALUE);    // 1 & 1 = 1
/// assert_eq!(False::VALUE, <True as And<False>>::Output::VALUE);  // 1 & 0 = 0
/// assert_eq!(False::VALUE, <False as And<True>>::Output::VALUE);  // 0 & 1 = 0
/// assert_eq!(False::VALUE, <False as And<False>>::Output::VALUE); // 0 & 0 = 0
/// ```
pub trait And<B: Boolean> {
    type Output: Boolean;
}

/// Type level binary or operation.
///
/// # Example
/// ```rust
/// use tcrts::all::*;
///
/// assert_eq!(True::VALUE, <True as Or<True>>::Output::VALUE);    // 1 | 1 = 1
/// assert_eq!(True::VALUE, <True as Or<False>>::Output::VALUE);   // 1 | 0 = 1
/// assert_eq!(True::VALUE, <False as Or<True>>::Output::VALUE);   // 0 | 1 = 1
/// assert_eq!(False::VALUE, <False as Or<False>>::Output::VALUE); // 0 | 0 = 0
/// ``````
pub trait Or<B: Boolean> {
    type Output: Boolean;
}

/// Type level binary xor operation.
///
/// # Example
/// ```rust
/// use tcrts::all::*;
///
/// assert_eq!(False::VALUE, <True as Xor<True>>::Output::VALUE);   // 1 ^ 1 = 0
/// assert_eq!(True::VALUE, <True as Xor<False>>::Output::VALUE);   // 1 ^ 0 = 1
/// assert_eq!(True::VALUE, <False as Xor<True>>::Output::VALUE);   // 0 ^ 1 = 1
/// assert_eq!(False::VALUE, <False as Xor<False>>::Output::VALUE); // 0 ^ 0 = 0
/// ```
pub trait Xor<B: Boolean> {
    type Output: Boolean;
}

impl And<False> for False {
    type Output = False;
}

impl And<True> for False {
    type Output = False;
}

impl And<False> for True {
    type Output = False;
}

impl And<True> for True {
    type Output = True;
}

impl Or<False> for False {
    type Output = False;
}

impl Or<True> for False {
    type Output = True;
}

impl Or<False> for True {
    type Output = True;
}

impl Or<True> for True {
    type Output = True;
}

impl Not for True {
    type Output = False;
}

impl Not for False {
    type Output = True;
}

impl Xor<False> for False {
    type Output = False;
}

impl Xor<True> for True {
    type Output = False;
}

impl Xor<False> for True {
    type Output = True;
}

impl Xor<True> for False {
    type Output = True;
}
