use crate::prelude::{ItchError, Result};

/// **Market Category**
///
/// Indicates listing market or listing market tier for the issue.
/// Embedded in [`ItchMessage::StockDirectory`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MarketCategory {
    NasdaqGlobalSelectMarket = b'Q',
    NasdaqGlobalMarket = b'G',
    NasdaqCapitalMarket = b'S',
    Nyse = b'N',
    NyseAmerican = b'A',
    NyseArca = b'P',
    Batsz = b'Z',
    InvestorsExchange = b'V',
}

impl MarketCategory {
    pub const ALL: [Self; 8] = [
        MarketCategory::NasdaqGlobalSelectMarket,
        MarketCategory::NasdaqGlobalMarket,
        MarketCategory::NasdaqCapitalMarket,
        MarketCategory::Nyse,
        MarketCategory::NyseAmerican,
        MarketCategory::NyseArca,
        MarketCategory::Batsz,
        MarketCategory::InvestorsExchange,
    ];

    pub const ALL_CHARS: [char; 8] = ['Q', 'G', 'S', 'N', 'A', 'P', 'Z', 'V'];

    pub const NAME: &str = "MarketCategory";

    const LUT: [Option<MarketCategory>; 256] = {
        let mut lut = [None; 256];
        lut[b'Q' as usize] = Some(MarketCategory::NasdaqGlobalSelectMarket);
        lut[b'G' as usize] = Some(MarketCategory::NasdaqGlobalMarket);
        lut[b'S' as usize] = Some(MarketCategory::NasdaqCapitalMarket);
        lut[b'N' as usize] = Some(MarketCategory::Nyse);
        lut[b'A' as usize] = Some(MarketCategory::NyseAmerican);
        lut[b'P' as usize] = Some(MarketCategory::NyseArca);
        lut[b'Z' as usize] = Some(MarketCategory::Batsz);
        lut[b'V' as usize] = Some(MarketCategory::InvestorsExchange);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<MarketCategory> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<MarketCategory> {
        Self::LUT[b as usize]
    }
}
