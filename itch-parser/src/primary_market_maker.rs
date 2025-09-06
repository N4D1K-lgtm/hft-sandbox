use crate::prelude::{ItchError, Result};

/// **Primary Market Maker**
///
/// Indicates if the market participant is registered as a Primary Market Maker.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `Y` | Yes | Participant is a Primary Market Maker |
/// | `N` | No | Participant is not a Primary Market Maker |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PrimaryMarketMaker {
    Yes = b'Y',
    No = b'N',
}

impl PrimaryMarketMaker {
    pub const ALL: [Self; 2] = [Self::Yes, Self::No];

    pub const ALL_CHARS: [char; 2] = ['Y', 'N'];

    pub const NAME: &str = "PrimaryMarketMaker";

    const LUT: [Option<PrimaryMarketMaker>; 256] = {
        let mut lut = [None; 256];
        lut[b'Y' as usize] = Some(Self::Yes);
        lut[b'N' as usize] = Some(Self::No);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<PrimaryMarketMaker> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<PrimaryMarketMaker> {
        Self::LUT[b as usize]
    }
}