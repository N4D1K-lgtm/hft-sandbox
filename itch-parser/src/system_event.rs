use crate::prelude::{ItchError, Result};

/// **System Event Message Codes**
///
/// Message codes embedded in [`ItchMessage::SystemEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SystemEventCode {
    /// Start of day: start of message traffic.
    StartMessages = b'O',
    /// Start of system hours (pre-market setup, testing).
    StartSystemHours = b'S',
    /// Start of market hours (continuous trading).
    StartMarketHours = b'Q',
    /// End of market hours (continuous trading ends).
    EndMarketHours = b'M',
    /// End of system hours (after-hours/closing procedures).
    EndSystemHours = b'E',
    /// End of day: end of message traffic.
    EndMessages = b'C',
}

impl SystemEventCode {
    pub const ALL: [Self; 6] = [
        SystemEventCode::StartMessages,
        SystemEventCode::StartSystemHours,
        SystemEventCode::StartMarketHours,
        SystemEventCode::EndMarketHours,
        SystemEventCode::EndSystemHours,
        SystemEventCode::EndMessages,
    ];

    pub const ALL_CHARS: [char; 6] = ['O', 'S', 'Q', 'M', 'E', 'C'];

    pub const NAME: &str = "SystemEventCode";

    const LUT: [Option<SystemEventCode>; 256] = {
        let mut lut = [None; 256];
        lut[b'O' as usize] = Some(SystemEventCode::StartMessages);
        lut[b'S' as usize] = Some(SystemEventCode::StartSystemHours);
        lut[b'Q' as usize] = Some(SystemEventCode::StartMarketHours);
        lut[b'M' as usize] = Some(SystemEventCode::EndMarketHours);
        lut[b'E' as usize] = Some(SystemEventCode::EndSystemHours);
        lut[b'C' as usize] = Some(SystemEventCode::EndMessages);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<SystemEventCode> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<SystemEventCode> {
        Self::LUT[b as usize]
    }
}
