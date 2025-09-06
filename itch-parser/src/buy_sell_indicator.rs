use crate::prelude::{ItchError, Result};

/// **Buy/Sell Indicator**
///
/// Indicates whether the order is a buy or sell order.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `B` | Buy | Buy order |
/// | `S` | Sell | Sell order |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BuySellIndicator {
    Buy = b'B',
    Sell = b'S',
}

impl BuySellIndicator {
    pub const ALL: [Self; 2] = [Self::Buy, Self::Sell];

    pub const ALL_CHARS: [char; 2] = ['B', 'S'];

    pub const NAME: &str = "BuySellIndicator";

    const LUT: [Option<BuySellIndicator>; 256] = {
        let mut lut = [None; 256];
        lut[b'B' as usize] = Some(Self::Buy);
        lut[b'S' as usize] = Some(Self::Sell);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<BuySellIndicator> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<BuySellIndicator> {
        Self::LUT[b as usize]
    }
}