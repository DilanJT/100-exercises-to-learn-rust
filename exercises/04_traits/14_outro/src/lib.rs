// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::num::Saturating;

macro_rules! impl_from {
    ($from_type:ty; $to_type:ty) => {
        impl From<$from_type> for $to_type {
            fn from(value: $from_type) -> Self {
                <$to_type>::new(value.into())
            }
        }
    };
    ($from_type:ty; $to_type:ty, deref) => {
        impl From<$from_type> for $to_type {
            fn from(value: $from_type) -> Self {
                <$to_type>::new((*value).into())
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16
}

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

// impl From<u16> for SaturatingU16 {
//     fn from(value: u16) -> Self {
//         SaturatingU16::new(value)
//     }
// }

impl_from!(u16; SaturatingU16);

// impl From<u8> for SaturatingU16 {
//     fn from(value: u8) -> Self {
//         SaturatingU16::new(value as u16)
//     }
// }

impl_from!(u8; SaturatingU16);


// impl From<&u16> for SaturatingU16 {
//     fn from(value: &u16) -> Self {
//         SaturatingU16::new(*value)
//     }
// }

impl_from!(&u16; SaturatingU16, deref);

// impl From<&u8> for SaturatingU16 {
//     fn from(value: &u8) -> Self {
//         SaturatingU16::new(*value as u16)
//     }
// }

impl_from!(&u8; SaturatingU16, deref);


impl std::ops::Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        SaturatingU16::new(self.value.saturating_add(other.value))
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &SaturatingU16) -> Self {
        SaturatingU16::new(self.value.saturating_add(other.value))
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        SaturatingU16::new(self.value.saturating_add(other))
    }
}

