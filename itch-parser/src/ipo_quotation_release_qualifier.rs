use crate::prelude::{ItchError, Result};

/// **IPO Quotation Release Qualifier**
///
/// Indicates the reason for the IPO quotation release.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `A` | Anticipated | Quotation release is anticipated |
/// | `C` | IPO Release Canceled/Postponed | IPO release has been canceled or postponed |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IpoQuotationReleaseQualifier {
    Anticipated = b'A',
    CanceledPostponed = b'C',
}

impl IpoQuotationReleaseQualifier {
    pub const ALL: [Self; 2] = [Self::Anticipated, Self::CanceledPostponed];

    pub const ALL_CHARS: [char; 2] = ['A', 'C'];

    pub const NAME: &str = "IpoQuotationReleaseQualifier";

    const LUT: [Option<IpoQuotationReleaseQualifier>; 256] = {
        let mut lut = [None; 256];
        lut[b'A' as usize] = Some(Self::Anticipated);
        lut[b'C' as usize] = Some(Self::CanceledPostponed);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<IpoQuotationReleaseQualifier> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<IpoQuotationReleaseQualifier> {
        Self::LUT[b as usize]
    }
}