#![warn(clippy::pedantic)]

use std::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Nibble {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    XA = 0xA,
    XB = 0xB,
    XC = 0xC,
    XD = 0xD,
    XE = 0xE,
    XF = 0xF,
}
impl Nibble {
    #[must_use]
    pub const fn try_from_u8(v: u8) -> Option<Self> {
        match v {
            0x0 => Some(Self::X0),
            0x1 => Some(Self::X1),
            0x2 => Some(Self::X2),
            0x3 => Some(Self::X3),
            0x4 => Some(Self::X4),
            0x5 => Some(Self::X5),
            0x6 => Some(Self::X6),
            0x7 => Some(Self::X7),
            0x8 => Some(Self::X8),
            0x9 => Some(Self::X9),
            0xA => Some(Self::XA),
            0xB => Some(Self::XB),
            0xC => Some(Self::XC),
            0xD => Some(Self::XD),
            0xE => Some(Self::XE),
            0xF => Some(Self::XF),
            _ => None,
        }
    }
    #[must_use]
    pub const fn from_u8(v: u8) -> Self {
        match v & 0x0F {
            0x0 => Self::X0,
            0x1 => Self::X1,
            0x2 => Self::X2,
            0x3 => Self::X3,
            0x4 => Self::X4,
            0x5 => Self::X5,
            0x6 => Self::X6,
            0x7 => Self::X7,
            0x8 => Self::X8,
            0x9 => Self::X9,
            0xA => Self::XA,
            0xB => Self::XB,
            0xC => Self::XC,
            0xD => Self::XD,
            0xE => Self::XE,
            0xF => Self::XF,
            _ => unreachable!(),
        }
    }
    #[must_use]
    pub const fn from_u8_upper(v: u8) -> Self {
        match v & 0xF0 {
            0x00 => Self::X0,
            0x10 => Self::X1,
            0x20 => Self::X2,
            0x30 => Self::X3,
            0x40 => Self::X4,
            0x50 => Self::X5,
            0x60 => Self::X6,
            0x70 => Self::X7,
            0x80 => Self::X8,
            0x90 => Self::X9,
            0xA0 => Self::XA,
            0xB0 => Self::XB,
            0xC0 => Self::XC,
            0xD0 => Self::XD,
            0xE0 => Self::XE,
            0xF0 => Self::XF,
            _ => unreachable!(),
        }
    }
    #[must_use]
    pub const fn to_u8(self) -> u8 { self as u8 }
    #[must_use]
    pub const fn to_u8_upper(self) -> u8 { (self as u8) << 4 }
}
impl Debug for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", *self as u8) }
}
impl Display for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", *self as u8) }
}
impl Binary for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:b}", *self as u8) }
}
impl LowerExp for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:e}", *self as u8) }
}
impl UpperExp for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:E}", *self as u8) }
}
impl LowerHex for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:x}", *self as u8) }
}
impl UpperHex for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:X}", *self as u8) }
}
impl Octal for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:o}", *self as u8) }
}

macro_rules! impl_from_nibble {
    ($type: ty) => {
        impl From<Nibble> for $type {
            fn from(value: Nibble) -> Self { value as u8 as Self }
        }
    };
    ($($type: ty),* $(,)*) => {
        $(impl_from_nibble!{$type})*
    }
}
impl_from_nibble! {u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64}
