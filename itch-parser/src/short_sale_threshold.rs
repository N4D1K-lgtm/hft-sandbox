use crate::prelude::{ItchError, Result};

/// **Short Sale Threshold Indicator**
///
/// Indicates if the security is subject to the short sale threshold.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `Y` | Yes | Security is restricted under Rule 201 of Regulation SHO |
/// | `N` | No | Security is not restricted |
/// | ` ` | Not Available | Threshold indicator not available |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShortSaleThreshold {
    Yes = b'Y',
    No = b'N',
    NotAvailable = b' ',
}

impl ShortSaleThreshold {
    pub const ALL: [Self; 3] = [Self::Yes, Self::No, Self::NotAvailable];

    pub const ALL_CHARS: [char; 3] = ['Y', 'N', ' '];

    pub const NAME: &str = "ShortSaleThreshold";

    const LUT: [Option<ShortSaleThreshold>; 256] = {
        let mut lut = [None; 256];
        lut[b'Y' as usize] = Some(Self::Yes);
        lut[b'N' as usize] = Some(Self::No);
        lut[b' ' as usize] = Some(Self::NotAvailable);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<ShortSaleThreshold> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<ShortSaleThreshold> {
        Self::LUT[b as usize]
    }
}