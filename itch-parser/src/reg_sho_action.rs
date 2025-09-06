use crate::prelude::{ItchError, Result};

/// **Reg SHO Action**
///
/// Indicates the Reg SHO short sale price test restriction status.
///
/// | Value | Description |
/// |-------|-------------|
/// | `0` | No price test in place |
/// | `1` | Reg SHO Short Sale Price Test Restriction in effect |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegShoAction {
    NoPriceTest = 0,
    PriceTestRestriction = 1,
}

impl RegShoAction {
    pub const ALL: [Self; 2] = [Self::NoPriceTest, Self::PriceTestRestriction];

    pub const NAME: &str = "RegShoAction";

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<RegShoAction> {
        match raw {
            0 => Ok(Self::NoPriceTest),
            1 => Ok(Self::PriceTestRestriction),
            _ => Err(ItchError::InvalidNumericField {
                field: Self::NAME,
                value: raw as u64,
            }),
        }
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<RegShoAction> {
        match b {
            0 => Some(Self::NoPriceTest),
            1 => Some(Self::PriceTestRestriction),
            _ => None,
        }
    }
}