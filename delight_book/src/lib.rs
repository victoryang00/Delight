#![cfg_attr(not(target_arch = "x86_64"), no_std)]
#![cfg_attr(not(target_arch = "x86_64"), no_main)]
#![cfg_attr(not(target_arch = "x86_64"),feature(custom_test_frameworks, lang_items, start))]
#![cfg_attr(not(target_arch = "x86_64"),test_runner(crate::test_runner))]

extern crate libc;
extern crate rand;
extern crate rand_core;
extern crate rand_isaac;

pub mod chapter2;
pub mod chapter3;
pub mod chapter4;
pub mod chapter5;
pub mod chapter6;
pub mod chapter7;
pub mod chapter8;
pub mod chapter9;
pub mod chapter10;
pub mod chapter11;
pub mod chapter12;
pub mod chapter15;
pub mod chapter16;
pub mod chapter17;

#[cfg(target_arch = "x86_64")]
use std::{ops::{Add, BitAnd, BitOr, BitOrAssign, BitXor, Not, Shl, ShlAssign, Shr, ShrAssign, Sub}, hash::{Hash, Hasher}, cmp::{Ord, Ordering, PartialOrd}, fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex}};

#[cfg(target_arch = "riscv64")]
use core::{ops::{Add, BitAnd, BitOr, BitOrAssign, BitXor, Not, Shl, ShlAssign, Shr, ShrAssign, Sub}, hash::{Hash, Hasher}, cmp::{Ord, Ordering, PartialOrd}, fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex}};

#[cfg(target_arch = "x86_64")]
pub use std::fmt::Error;
#[cfg(target_arch = "riscv64")]
pub use core::fmt::Error;

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

macro_rules! not_impl {
    ($($t:ty)*) => ($(
        impl BitXnor for $t {
            type Output = $t;

            #[inline]
            fn xnor(self, rhs: $t) -> $t { self & rhs | !self & !rhs }
        }

    )*)
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
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                let &$name(ref value) = self;
                <$type as Display>::fmt(value, f)
            }
        }
        impl UpperHex for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                let &$name(ref value) = self;
                <$type as UpperHex>::fmt(value, f)
            }
        }
        impl LowerHex for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                let &$name(ref value) = self;
                <$type as LowerHex>::fmt(value, f)
            }
        }
        impl Octal for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                let &$name(ref value) = self;
                <$type as Octal>::fmt(value, f)
            }
        }
        impl Binary for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                let &$name(ref value) = self;
                <$type as Binary>::fmt(value, f)
            }
        }

        impl BitXor for $name {
            type Output = $name;

            fn bitxor(self, other: $name) -> Self::Output {
                let result = $name(self.mask().0.bitxor(&other.mask().0));
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
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

        impl BitXnor for $name {
            type Output = Self;
            fn xnor(self, _rhs: Self) -> $name {
                let result = $name(self.0 & _rhs.0 | !self.0 & !_rhs.0);
                if result.mask().0 < 0 {
                    result + $name::MAX + $name(1)
                } else {
                    result
                }
            }
        }
    }
}
pub trait BitXnor<Rhs = Self> {
    type Output;
    fn xnor(self, rhs: Rhs) -> Self::Output;
}

not_impl! { i8 i16 }
define_c!(#[doc="The 8-bit unsigned integer type."], c15, 17, i32);
define_c!(#[doc="The 8-bit unsigned integer type."], c14, 15, i16);
define_c!(#[doc="The 8-bit unsigned integer type."], c13, 14, i16);
define_c!(#[doc="The 8-bit unsigned integer type."], c12, 13, i16);
define_c!(#[doc="The 8-bit unsigned integer type."], c11, 12, i16);
define_c!(#[doc="The 8-bit unsigned integer type."], c10, 11, i16);
define_c!(#[doc="The 8-bit unsigned integer type."], c9, 10, i16);
define_c!(#[doc="The 8-bit unsigned integer type."], c8, 9, i16);
define_c!(#[doc="The 6-bit unsigned integer type."], c6, 7, i8);
define_c!(#[doc="The 5-bit unsigned integer type."], c5, 6, i8);
define_c!(#[doc="The 4-bit unsigned integer type."], c4, 5, i8);
define_c!(#[doc="The 3-bit unsigned integer type."], c3, 4, i8);
define_c!(#[doc="The 2-bit unsigned integer type."], c2, 3, i8);
define_c!(#[doc="The 1-bit unsigned integer type."], c1, 2, i8);

implement_into!([c1, c2, c3, c4, c5, c6], i8);
implement_into!([c1, c2, c3, c4, c5, c6], i64);
implement_into!([c8,c9,c10,c11,c12,c13,c14], i16);
implement_from!(c6, [c1, c2, c3, c4, c5]);



#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

#[cfg(target_arch = "riscv64")]
#[start]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[cfg(target_arch = "riscv64")]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "riscv64")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
