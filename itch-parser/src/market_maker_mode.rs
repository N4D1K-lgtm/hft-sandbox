use crate::prelude::{ItchError, Result};

/// **Market Maker Mode**
///
/// Indicates the market maker mode for the participant.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `N` | Normal | Normal market making |
/// | `P` | Passive | Passive market making |
/// | `S` | Syndicate | Syndicate market making |
/// | `R` | Pre-syndicate | Pre-syndicate market making |
/// | `L` | Penalty | Penalty market making |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MarketMakerMode {
    Normal = b'N',
    Passive = b'P',
    Syndicate = b'S',
    PreSyndicate = b'R',
    Penalty = b'L',
}

impl MarketMakerMode {
    pub const ALL: [Self; 5] = [
        Self::Normal,
        Self::Passive,
        Self::Syndicate,
        Self::PreSyndicate,
        Self::Penalty,
    ];

    pub const ALL_CHARS: [char; 5] = ['N', 'P', 'S', 'R', 'L'];

    pub const NAME: &str = "MarketMakerMode";

    const LUT: [Option<MarketMakerMode>; 256] = {
        let mut lut = [None; 256];
        lut[b'N' as usize] = Some(Self::Normal);
        lut[b'P' as usize] = Some(Self::Passive);
        lut[b'S' as usize] = Some(Self::Syndicate);
        lut[b'R' as usize] = Some(Self::PreSyndicate);
        lut[b'L' as usize] = Some(Self::Penalty);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<MarketMakerMode> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<MarketMakerMode> {
        Self::LUT[b as usize]
    }
}