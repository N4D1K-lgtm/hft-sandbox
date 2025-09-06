use crate::prelude::{ItchError, Result};

/// **LULD Reference Price Tier**
///
/// Indicates which Limit Up-Limit Down price band calculation parameter is to be used.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `1` | Tier 1 | Tier 1 NMS Stock |
/// | `2` | Tier 2 | Tier 2 NMS Stock |
/// | ` ` | Not Available | LULD tier not available |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LuldReferencePriceTier {
    Tier1 = b'1',
    Tier2 = b'2',
    NotAvailable = b' ',
}

impl LuldReferencePriceTier {
    pub const ALL: [Self; 3] = [Self::Tier1, Self::Tier2, Self::NotAvailable];

    pub const ALL_CHARS: [char; 3] = ['1', '2', ' '];

    pub const NAME: &str = "LuldReferencePriceTier";

    const LUT: [Option<LuldReferencePriceTier>; 256] = {
        let mut lut = [None; 256];
        lut[b'1' as usize] = Some(Self::Tier1);
        lut[b'2' as usize] = Some(Self::Tier2);
        lut[b' ' as usize] = Some(Self::NotAvailable);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<LuldReferencePriceTier> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<LuldReferencePriceTier> {
        Self::LUT[b as usize]
    }
}