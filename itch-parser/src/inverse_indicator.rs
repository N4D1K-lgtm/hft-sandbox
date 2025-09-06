use crate::prelude::{ItchError, Result};

/// **Inverse Indicator**
///
/// Indicates if the ETP is an inverse ETP.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `Y` | Yes | ETP is an inverse ETP |
/// | `N` | No | ETP is not an inverse ETP |
/// | ` ` | Not Available | Inverse indicator not available |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InverseIndicator {
    Yes = b'Y',
    No = b'N',
    NotAvailable = b' ',
}

impl InverseIndicator {
    pub const ALL: [Self; 3] = [Self::Yes, Self::No, Self::NotAvailable];

    pub const ALL_CHARS: [char; 3] = ['Y', 'N', ' '];

    pub const NAME: &str = "InverseIndicator";

    const LUT: [Option<InverseIndicator>; 256] = {
        let mut lut = [None; 256];
        lut[b'Y' as usize] = Some(Self::Yes);
        lut[b'N' as usize] = Some(Self::No);
        lut[b' ' as usize] = Some(Self::NotAvailable);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<InverseIndicator> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<InverseIndicator> {
        Self::LUT[b as usize]
    }
}