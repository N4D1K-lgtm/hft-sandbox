use crate::prelude::{ItchError, Result};

/// **Cross Type**
///
/// Indicates the type of cross trade.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `O` | Opening Cross | Opening Cross |
/// | `C` | Closing Cross | Closing Cross |
/// | `H` | Cross for IPO and halted securities | Cross for IPO and halted/paused securities |
/// | `I` | Intraday Cross | Nasdaq Cross Network intraday cross |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CrossType {
    Opening = b'O',
    Closing = b'C',
    IpoHalted = b'H',
    Intraday = b'I',
}

impl CrossType {
    pub const ALL: [Self; 4] = [Self::Opening, Self::Closing, Self::IpoHalted, Self::Intraday];

    pub const ALL_CHARS: [char; 4] = ['O', 'C', 'H', 'I'];

    pub const NAME: &str = "CrossType";

    const LUT: [Option<CrossType>; 256] = {
        let mut lut = [None; 256];
        lut[b'O' as usize] = Some(Self::Opening);
        lut[b'C' as usize] = Some(Self::Closing);
        lut[b'H' as usize] = Some(Self::IpoHalted);
        lut[b'I' as usize] = Some(Self::Intraday);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<CrossType> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<CrossType> {
        Self::LUT[b as usize]
    }
}