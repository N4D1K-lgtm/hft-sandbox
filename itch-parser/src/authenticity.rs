use crate::prelude::{ItchError, Result};

/// **Authenticity**
///
/// Denotes if an issue or quoting participant record is set-up in
/// Nasdaq systems in a live/production, test, or demo state.
///
/// TODO: docs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Authenticity {
    Production = b'P',
    Test = b'T',
}

impl Authenticity {
    pub const ALL: [Self; 2] = [Authenticity::Production, Authenticity::Test];

    pub const ALL_CHARS: [char; 2] = ['P', 'T'];

    pub const NAME: &str = "Authenticity";

    const LUT: [Option<Authenticity>; 256] = {
        let mut lut = [None; 256];
        lut[b'P' as usize] = Some(Authenticity::Production);
        lut[b'T' as usize] = Some(Authenticity::Test);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<Authenticity> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<Authenticity> {
        Self::LUT[b as usize]
    }
}
