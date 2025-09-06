use crate::prelude::{ItchError, Result};

/// **Trading State**
///
/// Indicates the current trading state for a stock.
/// Sent whenever Nasdaq transitions a security to a new trading state.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `H` | Halted | Trading halted across all U.S. equity markets |
/// | `P` | Paused | Trading paused across all U.S. equity markets |
/// | `Q` | Quotation Only Period | Nasdaq is only accepting quotes, no trades |
/// | `T` | Trading | Trading on Nasdaq |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TradingState {
    Halted = b'H',
    Paused = b'P',
    QuotationOnly = b'Q',
    Trading = b'T',
}

impl TradingState {
    pub const ALL: [Self; 4] = [
        Self::Halted,
        Self::Paused,
        Self::QuotationOnly,
        Self::Trading,
    ];

    pub const ALL_CHARS: [char; 4] = ['H', 'P', 'Q', 'T'];

    pub const NAME: &str = "TradingState";

    const LUT: [Option<TradingState>; 256] = {
        let mut lut = [None; 256];
        lut[b'H' as usize] = Some(Self::Halted);
        lut[b'P' as usize] = Some(Self::Paused);
        lut[b'Q' as usize] = Some(Self::QuotationOnly);
        lut[b'T' as usize] = Some(Self::Trading);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<TradingState> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<TradingState> {
        Self::LUT[b as usize]
    }
}