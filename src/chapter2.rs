use std::ops::{Add, BitAnd, BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Sub};

use std::hash::{Hash, Hasher};

use std::cmp::{Ord, Ordering, PartialOrd};

use std::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};

/// modified from https://docs.rs/ux/0.0.1/src/ux/lib.rs.html#63

macro_rules! implement_from {
    {[$($name:ident),*], [$($from:ident),*] } => {$(implement_from!($name, $from);)*};
    {$name:ident, [$($from:ident),*] } => {$(implement_from!($name, $from);)*};
    {[$($name:ident),*], $from:ident } => {$(implement_from!($name, $from);)*};
    {$name:ident, $from:ty} => {
        impl From<$from> for $name {
            fn from(x: $from) -> $name {
                $name(x.into())
            }
        }
    };
}

// Only implement if $type can be converted from $name lossless
macro_rules! implement_into {
    {[$($name:ident),*], $from:ident } => {$(implement_into!($name, $from);)*};
    {$name:ident, $into:ident} => {
        impl From<$name> for $into {
            fn from(x: $name) -> $into {
                $into::from(x.0)
            }
        }
    };
}
macro_rules! define_c {
    ($name:ident, $bits:expr, $type:ident) => {define_c!(#[doc=""], $name, $bits, $type);};
    (#[$doc:meta], $name:ident, $bits:expr, $type:ident) => {

        #[$doc]
        #[allow(non_camel_case_types)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct $name($type);

        #[$doc]
        impl $name {
            pub const MAX: Self = $name(((1 as $type) << ($bits - 1)) - 1);
            pub const MIN: Self = $name(-((1 as $type) << ($bits - 1)));

            fn mask(self) -> Self {
                if ( self.0 & (1<<($bits-1)) ) == 0 {
                    $name(self.0 & ( ((1 as $type) << $bits).overflowing_sub(1).0))
                } else {
                    $name(self.0 | !( ((1 as $type) << $bits).overflowing_sub(1).0))
                }
            }

        }

        implement_common!($name, $bits, $type);

    }
}

macro_rules! implement_common {
    ($name:ident, $bits:expr, $type:ident) => {
        impl $name {
            /// Returns the smallest value that can be represented by this integer type.
            pub fn min_value() -> $name {
                $name::MIN
            }
            /// Returns the largest value that can be represented by this integer type.
            pub fn max_value() -> $name {
                $name::MAX
            }
            pub fn new(value: $type) -> $name {
                assert!(value <= $name::MAX.0 && value >= $name::MIN.0);
                $name(value)
            }
            pub fn wrapping_sub(self, rhs: Self) -> Self {
                $name(self.0.wrapping_sub(rhs.0)).mask()
            }
            pub fn wrapping_add(self, rhs: Self) -> Self {
                $name(self.0.wrapping_add(rhs.0)).mask()
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.mask().0 == other.mask().0
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<Ordering> {
                self.mask().0.partial_cmp(&other.mask().0)
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &$name) -> Ordering {
                self.mask().0.cmp(&other.mask().0)
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, h: &mut H) {
                self.mask().0.hash(h)
            }
        }

        // Implement formatting functions
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
                let &$name(ref value) = self;
                <$type as Display>::fmt(value, f)
            }
        }
        impl UpperHex for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
                let &$name(ref value) = self;
                <$type as UpperHex>::fmt(value, f)
            }
        }
        impl LowerHex for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
                let &$name(ref value) = self;
                <$type as LowerHex>::fmt(value, f)
            }
        }
        impl Octal for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
                let &$name(ref value) = self;
                <$type as Octal>::fmt(value, f)
            }
        }
        impl Binary for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
                let &$name(ref value) = self;
                <$type as Binary>::fmt(value, f)
            }
        }

        impl<T> Shr<T> for $name
        where
            $type: Shr<T, Output = $type>,
        {
            type Output = $name;

            fn shr(self, rhs: T) -> $name {
                let result = $name(self.mask().0.shr(rhs));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl<T> Shl<T> for $name
        where
            $type: Shl<T, Output = $type>,
        {
            type Output = $name;

            fn shl(self, rhs: T) -> $name {
                let result = $name(self.mask().0.shl(rhs));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl<T> ShrAssign<T> for $name
        where
            $type: ShrAssign<T>,
        {
            fn shr_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shr_assign(rhs);
            }
        }

        impl<T> ShlAssign<T> for $name
        where
            $type: ShlAssign<T>,
        {
            fn shl_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shl_assign(rhs);
            }
        }

        impl BitOr<$name> for $name {
            type Output = $name;

            fn bitor(self, rhs: $name) -> Self::Output {
                let result = $name(self.mask().0.bitor(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl<'a> BitOr<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitor(self, rhs: &'a $name) -> Self::Output {
                let result = $name(self.mask().0.bitor(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl<'a> BitOr<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitor(self, rhs: $name) -> Self::Output {
                let result = $name(self.mask().0.bitor(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl<'a> BitOr<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitor(self, rhs: &'a $name) -> Self::Output {
                let result = $name(self.mask().0.bitor(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl BitOrAssign<$name> for $name {
            fn bitor_assign(&mut self, other: $name) {
                *self = self.mask();
                self.0.bitor_assign(other.mask().0);
            }
        }

        impl BitAnd<$name> for $name {
            type Output = $name;

            fn bitand(self, rhs: $name) -> Self::Output {
                let result = $name(self.mask().0.bitand(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }
        impl<'a> BitAnd<&'a $name> for $name {
            type Output = <$name as BitAnd<$name>>::Output;

            fn bitand(self, rhs: &'a $name) -> Self::Output {
                let result = $name(self.mask().0.bitand(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl<'a> BitAnd<$name> for &'a $name {
            type Output = <$name as BitAnd<$name>>::Output;

            fn bitand(self, rhs: $name) -> Self::Output {
                let result = $name(self.mask().0.bitand(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl<'a> BitAnd<&'a $name> for &'a $name {
            type Output = <$name as BitAnd<$name>>::Output;

            fn bitand(self, rhs: &'a $name) -> Self::Output {
                let result = $name(self.mask().0.bitand(rhs.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, _rhs: Self) -> $name {
                let result = $name(self.0.wrapping_add(_rhs.0)).mask();
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl Not for $name {
            type Output = Self;
            fn not(self) -> $name {
                let result = $name(self.mask().0.not());
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, _rhs: Self) -> $name {
                let result = $name(self.0.wrapping_sub(_rhs.0)).mask();
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }
    };
}

define_c!(#[doc="The 8-bit unsigned integer type."], c8, 9, i16);
define_c!(#[doc="The 6-bit unsigned integer type."], c6, 7, i8);
define_c!(#[doc="The 5-bit unsigned integer type."], c5, 6, i8);
define_c!(#[doc="The 4-bit unsigned integer type."], c4, 5, i8);
define_c!(#[doc="The 3-bit unsigned integer type."], c3, 4, i8);
define_c!(#[doc="The 2-bit unsigned integer type."], c2, 3, i8);
define_c!(#[doc="The 1-bit unsigned integer type."], c1, 2, i8);

implement_into!([c1, c2, c3, c4, c5, c6], i8);
implement_into!([c1, c2, c3, c4, c5, c6], i64);
implement_from!(c6, [c1, c2, c3, c4, c5]);

/// turn off the rightmost 1-bit in a word, producing 0 if none
fn basics_get_and(x: c8) -> c8 {
    x & (x - c8(1))
}
/// turn on the rightmost 0-bit in a word, producing all 1’s if none
fn basics_get_or(x: c8) -> c8 {
    x | (x + c8(1))
}
/// turn off the trailing 1’s in a word, producing x if none
pub fn basics_all_right_1_to_0(x: c8) -> c8 {
    x & (x + c8(1))
}
/// turn on the trailing 0’s in a word, producing x if none
pub fn basics_all_right_0_to_1(x: c8) -> c8 {
    x | (x - c8(1))
}
/// create a word with a single 1-bit at the position of the rightmost 0-bit in x
pub fn basics_single_0_right_0(x: c8) -> c8 {
    !x & (x + c8(1))
}
/// create a word with a single 0-bit at the position of the rightmost 1-bit in x, producing all 1’s if none
pub fn basics_single_0_right_1(x: c8) -> c8 {
    !x | (x - c8(1))
}
/// create a word with 1’s at the positions of the trailing 0’s in x, and 0’s elsewhere, producing 0 if none, have 3 of them
pub fn basics_trailing_0_1(x: c8) -> c8 {
    !(x | (c8(0) - x))
}
pub fn basics_trailing_0_2(x: c8) -> c8 {
    !x & (x - c8(1))
}
pub fn basics_trailing_0_3(x: c8) -> c8 {
    (x & (c8(0) - x)) - c8(1)
}

/// create a word with 0’s at the positions of the trailing 1’s in x, and 0’s elsewhere, producing all 1’s if none
pub fn basics_trailing_1(x: c8) -> c8 {
    !x | (x + c8(1))
}
/// isolate the rightmost 1-bit, producing 0 if none (e.g.,
pub fn basics_isolate_1(x: c8) -> c8 {
    x & (c8(0) - x)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basics() {
        assert_eq!(basics_get_and(c8(0b01011000)), c8(0b001010000));
        assert_eq!(basics_get_or(c8(0b10100111)), c8(0b10101111));
        assert_eq!(basics_all_right_1_to_0(c8(0b10100111)), c8(0b10100000));
        assert_eq!(basics_all_right_0_to_1(c8(0b10101000)), c8(0b10101111));
        assert_eq!(basics_single_0_right_0(c8(0b10100111)), c8(0b00001000));
        assert_eq!(basics_single_0_right_1(c8(0b10101000)), c8(0b11110111));
        assert_eq!(basics_trailing_0_1(c8(0b01011000)), c8(0b00000111));
        assert_eq!(basics_trailing_0_2(c8(0b01011000)), c8(0b00000111));
        assert_eq!(basics_trailing_0_3(c8(0b01011000)), c8(0b00000111));
        assert_eq!(basics_trailing_1(c8(0b10100111)), c8(0b11111000));
        assert_eq!(basics_isolate_1(c8(0b01011000)), c8(0b00001000));
    }

    #[test]
    fn test_vm_result() {}
}
