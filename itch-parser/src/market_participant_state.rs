use crate::prelude::{ItchError, Result};

/// **Market Participant State**
///
/// Indicates the current state of the market participant.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `A` | Active | Participant is active |
/// | `E` | Excused/Withdrawn | Participant is excused/withdrawn |
/// | `W` | Withdrawn | Participant is withdrawn |
/// | `S` | Suspended | Participant is suspended |
/// | `D` | Deleted | Participant is deleted |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MarketParticipantState {
    Active = b'A',
    ExcusedWithdrawn = b'E',
    Withdrawn = b'W',
    Suspended = b'S',
    Deleted = b'D',
}

impl MarketParticipantState {
    pub const ALL: [Self; 5] = [
        Self::Active,
        Self::ExcusedWithdrawn,
        Self::Withdrawn,
        Self::Suspended,
        Self::Deleted,
    ];

    pub const ALL_CHARS: [char; 5] = ['A', 'E', 'W', 'S', 'D'];

    pub const NAME: &str = "MarketParticipantState";

    const LUT: [Option<MarketParticipantState>; 256] = {
        let mut lut = [None; 256];
        lut[b'A' as usize] = Some(Self::Active);
        lut[b'E' as usize] = Some(Self::ExcusedWithdrawn);
        lut[b'W' as usize] = Some(Self::Withdrawn);
        lut[b'S' as usize] = Some(Self::Suspended);
        lut[b'D' as usize] = Some(Self::Deleted);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<MarketParticipantState> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<MarketParticipantState> {
        Self::LUT[b as usize]
    }
}