use crate::prelude::{ItchError, Result};

/// **Imbalance Direction**
///
/// Indicates the direction of the order imbalance.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `B` | Buy | Buy imbalance |
/// | `S` | Sell | Sell imbalance |
/// | `N` | No Imbalance | No imbalance exists |
/// | `O` | Insufficient Orders | Insufficient orders to calculate |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImbalanceDirection {
    Buy = b'B',
    Sell = b'S',
    NoImbalance = b'N',
    InsufficientOrders = b'O',
}

impl ImbalanceDirection {
    pub const ALL: [Self; 4] = [
        Self::Buy,
        Self::Sell,
        Self::NoImbalance,
        Self::InsufficientOrders,
    ];

    pub const ALL_CHARS: [char; 4] = ['B', 'S', 'N', 'O'];

    pub const NAME: &str = "ImbalanceDirection";

    const LUT: [Option<ImbalanceDirection>; 256] = {
        let mut lut = [None; 256];
        lut[b'B' as usize] = Some(Self::Buy);
        lut[b'S' as usize] = Some(Self::Sell);
        lut[b'N' as usize] = Some(Self::NoImbalance);
        lut[b'O' as usize] = Some(Self::InsufficientOrders);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<ImbalanceDirection> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<ImbalanceDirection> {
        Self::LUT[b as usize]
    }
}