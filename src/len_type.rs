use core::{
    ops::{Add, AddAssign, Sub, SubAssign},
};
#[cfg(not(feature = "certified_subset"))]
use core::fmt::{Debug, Display};

#[cfg(not(feature = "certified_subset"))]
pub trait Sealed:
    Send
    + Sync
    + Copy
    + Display
    + Debug
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialOrd
    + TryFrom<usize, Error: Debug>
    + TryInto<usize, Error: Debug>
{
    /// The zero value of the integer type.
    const ZERO: Self;
    /// The one value of the integer type.
    const MAX: Self;
    /// The maximum value of this type, as a `usize`.
    const MAX_USIZE: usize;

    /// The one value of the integer type.
    ///
    /// It's a function instead of constant because we want to have implementation which panics for
    /// type `ZeroLenType`
    fn one() -> Self;

    /// An infallible conversion from `usize` to `LenT`.
    #[inline]
    fn from_usize(val: usize) -> Self {
        val.try_into().unwrap()
    }

    /// An infallible conversion from `LenT` to `usize`.
    #[inline]
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }

    /// Converts `LenT` into `Some(usize)`, unless it's `Self::MAX`, where it returns `None`.
    #[inline]
    fn to_non_max(self) -> Option<usize> {
        if self == Self::MAX {
            None
        } else {
            Some(self.into_usize())
        }
    }
}

#[cfg(feature = "certified_subset")]
pub trait Sealed:
    Send
    + Sync
    + Copy
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialOrd
    + TryFrom<usize>
    + TryInto<usize>
{
    /// The zero value of the integer type.
    const ZERO: Self;
    /// The one value of the integer type.
    const MAX: Self;
    /// The maximum value of this type, as a `usize`.
    const MAX_USIZE: usize;

    /// The one value of the integer type.
    ///
    /// It's a function instead of constant because we want to have implementation which panics for
    /// type `ZeroLenType`
    fn one() -> Self;

    /// An infallible conversion from `usize` to `LenT`.
    #[inline]
    // Coverage: the unreachable!() error branch can never execute because LenType
    // is only implemented for u32 and usize, both of which are infallible from usize.
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn from_usize(val: usize) -> Self {
        val.try_into().unwrap_or_else(|_| unreachable!())
    }

    /// An infallible conversion from `LenT` to `usize`.
    #[inline]
    // Coverage: the unreachable!() error branch can never execute because LenType
    // is only implemented for u32 and usize, both of which convert infallibly to usize.
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn into_usize(self) -> usize {
        self.try_into().unwrap_or_else(|_| unreachable!())
    }

    /// Converts `LenT` into `Some(usize)`, unless it's `Self::MAX`, where it returns `None`.
    #[inline]
    fn to_non_max(self) -> Option<usize> {
        if self == Self::MAX {
            None
        } else {
            Some(self.into_usize())
        }
    }
}

macro_rules! impl_lentype {
    ($($(#[$meta:meta])* $LenT:ty),*) => {$(
        $(#[$meta])*
        impl Sealed for $LenT {
            const ZERO: Self = 0;
            const MAX: Self = Self::MAX;
            const MAX_USIZE: usize = Self::MAX as _;

            fn one() -> Self {
                1
            }
        }

        $(#[$meta])*
        impl LenType for $LenT {}
    )*}
}

/// A sealed trait representing a valid type to use as a length for a container.
///
/// This cannot be implemented in user code, and is restricted to `u8`, `u16`, `u32`, and `usize`.
pub trait LenType: Sealed {}

impl_lentype!(
    #[cfg(not(feature = "certified_subset"))]
    u8,
    #[cfg(not(feature = "certified_subset"))]
    u16,
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    u32,
    usize
);

pub const fn check_capacity_fits<LenT: LenType, const N: usize>() {
    assert!(LenT::MAX_USIZE >= N, "The capacity is larger than `LenT` can hold, increase the size of `LenT` or reduce the capacity");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize_one() {
        assert_eq!(<usize as Sealed>::one(), 1usize);
    }

    #[test]
    fn usize_from_usize() {
        assert_eq!(<usize as Sealed>::from_usize(42), 42usize);
    }

    #[test]
    fn usize_into_usize() {
        assert_eq!(Sealed::into_usize(42usize), 42);
    }

    #[test]
    fn usize_to_non_max_returns_none_for_max() {
        assert_eq!(<usize as Sealed>::to_non_max(usize::MAX), None);
    }

    #[test]
    fn usize_to_non_max_returns_some_for_non_max() {
        assert_eq!(<usize as Sealed>::to_non_max(5usize), Some(5));
    }

    #[test]
    fn usize_to_non_max_zero() {
        assert_eq!(<usize as Sealed>::to_non_max(0usize), Some(0));
    }

    #[test]
    fn usize_constants() {
        assert_eq!(<usize as Sealed>::ZERO, 0usize);
        assert_eq!(<usize as Sealed>::MAX, usize::MAX);
        assert_eq!(<usize as Sealed>::MAX_USIZE, usize::MAX);
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn u32_one() {
        assert_eq!(<u32 as Sealed>::one(), 1u32);
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn u32_from_usize() {
        assert_eq!(<u32 as Sealed>::from_usize(42), 42u32);
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn u32_into_usize() {
        assert_eq!(Sealed::into_usize(42u32), 42usize);
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn u32_to_non_max_returns_none_for_max() {
        assert_eq!(<u32 as Sealed>::to_non_max(u32::MAX), None);
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn u32_to_non_max_returns_some_for_non_max() {
        assert_eq!(<u32 as Sealed>::to_non_max(5u32), Some(5));
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn u32_constants() {
        assert_eq!(<u32 as Sealed>::ZERO, 0u32);
        assert_eq!(<u32 as Sealed>::MAX, u32::MAX);
        assert_eq!(<u32 as Sealed>::MAX_USIZE, u32::MAX as usize);
    }

    #[test]
    fn check_capacity_fits_usize() {
        check_capacity_fits::<usize, 0>();
        check_capacity_fits::<usize, 1024>();
    }

    #[test]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn check_capacity_fits_u32() {
        check_capacity_fits::<u32, 0>();
        check_capacity_fits::<u32, 1024>();
    }
}
