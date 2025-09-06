use crate::prelude::{ItchError, Result};

/// **ETP Flag**
///
/// Indicates if the security is an exchange traded product (ETP).
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `Y` | Yes | Security is an ETP |
/// | `N` | No | Security is not an ETP |
/// | ` ` | Not Available | ETP flag not available |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EtpFlag {
    Yes = b'Y',
    No = b'N',
    NotAvailable = b' ',
}

impl EtpFlag {
    pub const ALL: [Self; 3] = [Self::Yes, Self::No, Self::NotAvailable];

    pub const ALL_CHARS: [char; 3] = ['Y', 'N', ' '];

    pub const NAME: &str = "EtpFlag";

    const LUT: [Option<EtpFlag>; 256] = {
        let mut lut = [None; 256];
        lut[b'Y' as usize] = Some(Self::Yes);
        lut[b'N' as usize] = Some(Self::No);
        lut[b' ' as usize] = Some(Self::NotAvailable);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<EtpFlag> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<EtpFlag> {
        Self::LUT[b as usize]
    }
}