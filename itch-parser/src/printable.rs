use crate::prelude::{ItchError, Result};

/// **Printable**
///
/// Indicates if the execution should be printed on the consolidated tape.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `Y` | Yes | Execution is printable |
/// | `N` | No | Execution is not printable |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Printable {
    Yes = b'Y',
    No = b'N',
}

impl Printable {
    pub const ALL: [Self; 2] = [Self::Yes, Self::No];

    pub const ALL_CHARS: [char; 2] = ['Y', 'N'];

    pub const NAME: &str = "Printable";

    const LUT: [Option<Printable>; 256] = {
        let mut lut = [None; 256];
        lut[b'Y' as usize] = Some(Self::Yes);
        lut[b'N' as usize] = Some(Self::No);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<Printable> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<Printable> {
        Self::LUT[b as usize]
    }
}