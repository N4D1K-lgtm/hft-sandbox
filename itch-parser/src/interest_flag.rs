use crate::prelude::{ItchError, Result};

/// **Interest Flag**
///
/// Indicates retail price improvement availability.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `B` | Buy Side | RPI available on buy side |
/// | `S` | Sell Side | RPI available on sell side |
/// | `A` | Both Sides | RPI available on both sides |
/// | `N` | No RPI | No RPI available |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InterestFlag {
    BuySide = b'B',
    SellSide = b'S',
    BothSides = b'A',
    NoRpi = b'N',
}

impl InterestFlag {
    pub const ALL: [Self; 4] = [Self::BuySide, Self::SellSide, Self::BothSides, Self::NoRpi];

    pub const ALL_CHARS: [char; 4] = ['B', 'S', 'A', 'N'];

    pub const NAME: &str = "InterestFlag";

    const LUT: [Option<InterestFlag>; 256] = {
        let mut lut = [None; 256];
        lut[b'B' as usize] = Some(Self::BuySide);
        lut[b'S' as usize] = Some(Self::SellSide);
        lut[b'A' as usize] = Some(Self::BothSides);
        lut[b'N' as usize] = Some(Self::NoRpi);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<InterestFlag> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<InterestFlag> {
        Self::LUT[b as usize]
    }
}