use crate::prelude::{ItchError, Result};

/// **Price Variation Indicator**
///
/// Indicates the price variation from the previous close.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `L` | Less | Current reference price is less than $1 |
/// | `1` | $1 to $5 | Current reference price is $1 or more but less than $5 |
/// | `5` | $5 to $10 | Current reference price is $5 or more but less than $10 |
/// | `0` | $10 to $20 | Current reference price is $10 or more but less than $20 |
/// | `2` | $20 to $100 | Current reference price is $20 or more but less than $100 |
/// | `H` | $100 or more | Current reference price is $100 or more |
/// | ` ` | Not Available | Price variation indicator not available |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PriceVariationIndicator {
    LessThanOne = b'L',
    OneToFive = b'1',
    FiveToTen = b'5',
    TenToTwenty = b'0',
    TwentyToHundred = b'2',
    HundredOrMore = b'H',
    NotAvailable = b' ',
}

impl PriceVariationIndicator {
    pub const ALL: [Self; 7] = [
        Self::LessThanOne,
        Self::OneToFive,
        Self::FiveToTen,
        Self::TenToTwenty,
        Self::TwentyToHundred,
        Self::HundredOrMore,
        Self::NotAvailable,
    ];

    pub const ALL_CHARS: [char; 7] = ['L', '1', '5', '0', '2', 'H', ' '];

    pub const NAME: &str = "PriceVariationIndicator";

    const LUT: [Option<PriceVariationIndicator>; 256] = {
        let mut lut = [None; 256];
        lut[b'L' as usize] = Some(Self::LessThanOne);
        lut[b'1' as usize] = Some(Self::OneToFive);
        lut[b'5' as usize] = Some(Self::FiveToTen);
        lut[b'0' as usize] = Some(Self::TenToTwenty);
        lut[b'2' as usize] = Some(Self::TwentyToHundred);
        lut[b'H' as usize] = Some(Self::HundredOrMore);
        lut[b' ' as usize] = Some(Self::NotAvailable);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<PriceVariationIndicator> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<PriceVariationIndicator> {
        Self::LUT[b as usize]
    }
}

