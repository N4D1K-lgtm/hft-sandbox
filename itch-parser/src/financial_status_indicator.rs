use crate::prelude::{ItchError, Result};

/// ** Financial Status Indicator**
///
/// For Nasdaq listed issues, this field indicates when a firm
/// is not in compliance with Nasdaq continued listing requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FinancialStatusIndicator {
    Deficient = b'D',
    Delinquent = b'E',
    Bankrupt = b'Q',
    Suspended = b'S',
    DeficientBankrupt = b'G',
    DeficientDelinquent = b'H',
    DelinquentBankrupt = b'J',
    DeficientDelinquentBankrupt = b'K',
    // TODO: what does this mean (rename?)
    CreationsSuspended = b'V',
    Normal = b'N',
    NotAvailable = b' ',
}

impl FinancialStatusIndicator {
    pub const ALL: [Self; 11] = [
        Self::Deficient,
        Self::Delinquent,
        Self::Bankrupt,
        Self::Suspended,
        Self::DeficientBankrupt,
        Self::DeficientDelinquent,
        Self::DelinquentBankrupt,
        Self::DeficientDelinquentBankrupt,
        Self::CreationsSuspended,
        Self::Normal,
        Self::NotAvailable,
    ];

    pub const ALL_CHARS: [char; 11] = ['D', 'E', 'Q', 'S', 'G', 'H', 'J', 'K', 'V', 'N', ' '];

    pub const NAME: &str = "FinancialStatusIndicator";

    const LUT: [Option<FinancialStatusIndicator>; 256] = {
        let mut lut = [None; 256];
        lut[b'D' as usize] = Some(FinancialStatusIndicator::Deficient);
        lut[b'E' as usize] = Some(FinancialStatusIndicator::Delinquent);
        lut[b'Q' as usize] = Some(FinancialStatusIndicator::Bankrupt);
        lut[b'S' as usize] = Some(FinancialStatusIndicator::Suspended);
        lut[b'G' as usize] = Some(FinancialStatusIndicator::DeficientBankrupt);
        lut[b'H' as usize] = Some(FinancialStatusIndicator::DeficientDelinquent);
        lut[b'J' as usize] = Some(FinancialStatusIndicator::DelinquentBankrupt);
        lut[b'K' as usize] = Some(FinancialStatusIndicator::DeficientDelinquentBankrupt);
        lut[b'C' as usize] = Some(FinancialStatusIndicator::CreationsSuspended);
        lut[b'N' as usize] = Some(FinancialStatusIndicator::Normal);
        lut[b' ' as usize] = Some(FinancialStatusIndicator::NotAvailable);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<FinancialStatusIndicator> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<FinancialStatusIndicator> {
        Self::LUT[b as usize]
    }
}
