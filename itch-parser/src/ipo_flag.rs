use crate::prelude::{ItchError, Result};

/// **IPO Flag**
///
/// Indicates if the security is set up for IPO release.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `Y` | Yes | Security is set up as a new IPO security |
/// | `N` | No | Security is not a new IPO security |
/// | ` ` | Not Available | IPO flag not available |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IpoFlag {
    Yes = b'Y',
    No = b'N',
    NotAvailable = b' ',
}

impl IpoFlag {
    pub const ALL: [Self; 3] = [Self::Yes, Self::No, Self::NotAvailable];

    pub const ALL_CHARS: [char; 3] = ['Y', 'N', ' '];

    pub const NAME: &str = "IpoFlag";

    const LUT: [Option<IpoFlag>; 256] = {
        let mut lut = [None; 256];
        lut[b'Y' as usize] = Some(Self::Yes);
        lut[b'N' as usize] = Some(Self::No);
        lut[b' ' as usize] = Some(Self::NotAvailable);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<IpoFlag> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<IpoFlag> {
        Self::LUT[b as usize]
    }
}