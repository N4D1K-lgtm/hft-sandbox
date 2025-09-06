use crate::prelude::{ItchError, Result};

/// **Round Lots Only**
///
/// Indicates if Nasdaq system limits order entry for the issue to round lots only.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `Y` | Yes | Only round lot orders are accepted |
/// | `N` | No | Odd lot and mixed lot orders are accepted |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RoundLotsOnly {
    Yes = b'Y',
    No = b'N',
}

impl RoundLotsOnly {
    pub const ALL: [Self; 2] = [Self::Yes, Self::No];

    pub const ALL_CHARS: [char; 2] = ['Y', 'N'];

    pub const NAME: &str = "RoundLotsOnly";

    const LUT: [Option<RoundLotsOnly>; 256] = {
        let mut lut = [None; 256];
        lut[b'Y' as usize] = Some(Self::Yes);
        lut[b'N' as usize] = Some(Self::No);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<RoundLotsOnly> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<RoundLotsOnly> {
        Self::LUT[b as usize]
    }
}