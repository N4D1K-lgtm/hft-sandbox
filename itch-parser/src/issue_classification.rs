use crate::prelude::{ItchError, Result};

/// **Issue Classification**
///
/// Identifies the security class for the issue as assigned by Nasdaq.
/// Identifies the primary listing market for the issue.
///
/// | Value | Symbol | Description |
/// |-------|--------|-------------|
/// | `A` | NASDAQ-listed | American Depositary Shares |
/// | `B` | NASDAQ-listed | Bonds |
/// | `C` | NYSE-listed | Common Stock |
/// | `D` | NASDAQ-listed | Depository Receipts |
/// | `E` | NASDAQ-listed | ETFs - Equity Based |
/// | `F` | NASDAQ-listed | Debentures |
/// | `G` | NYSE-listed | Global Shares |
/// | `H` | NYSE-listed | ETFs - Fixed Income |
/// | `I` | NASDAQ-listed | ETNs - Fixed Income |
/// | `J` | NYSE-listed | ETNs - Equity Based |
/// | `K` | NYSE-listed | ETNs - Fixed Income |
/// | `L` | NASDAQ-listed | ETNs - Equity Based |
/// | `M` | NASDAQ-listed | Mutual Funds |
/// | `N` | NYSE-listed | Notes |
/// | `O` | NASDAQ-listed | Ordinary Shares |
/// | `P` | NYSE-listed | Preferred Shares |
/// | `Q` | NASDAQ-listed | Other Securities |
/// | `R` | NASDAQ-listed | Rights |
/// | `S` | NYSE-listed | Shares of Beneficial Interest |
/// | `T` | NYSE-listed | Convertible Debentures |
/// | `U` | NASDAQ-listed | Units |
/// | `V` | NYSE-listed | Units/Investment Trusts |
/// | `W` | NASDAQ-listed | Warrants |
/// | `X` | NASDAQ-listed | Index Warrants |
/// | `Y` | NYSE-listed | ADRs |
/// | `Z` | NYSE-listed | Miscellaneous |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IssueClassification {
    AmericanDepositaryShares = b'A',
    Bonds = b'B',
    CommonStock = b'C',
    DepositoryReceipts = b'D',
    EquityBasedEtf = b'E',
    Debentures = b'F',
    GlobalShares = b'G',
    FixedIncomeEtf = b'H',
    FixedIncomeEtnNasdaq = b'I',
    EquityBasedEtnNyse = b'J',
    FixedIncomeEtnNyse = b'K',
    EquityBasedEtnNasdaq = b'L',
    MutualFunds = b'M',
    Notes = b'N',
    OrdinaryShares = b'O',
    PreferredShares = b'P',
    OtherSecurities = b'Q',
    Rights = b'R',
    SharesOfBeneficialInterest = b'S',
    ConvertibleDebentures = b'T',
    Units = b'U',
    UnitsInvestmentTrusts = b'V',
    Warrants = b'W',
    IndexWarrants = b'X',
    Adrs = b'Y',
    Miscellaneous = b'Z',
}

impl IssueClassification {
    pub const ALL: [Self; 26] = [
        Self::AmericanDepositaryShares,
        Self::Bonds,
        Self::CommonStock,
        Self::DepositoryReceipts,
        Self::EquityBasedEtf,
        Self::Debentures,
        Self::GlobalShares,
        Self::FixedIncomeEtf,
        Self::FixedIncomeEtnNasdaq,
        Self::EquityBasedEtnNyse,
        Self::FixedIncomeEtnNyse,
        Self::EquityBasedEtnNasdaq,
        Self::MutualFunds,
        Self::Notes,
        Self::OrdinaryShares,
        Self::PreferredShares,
        Self::OtherSecurities,
        Self::Rights,
        Self::SharesOfBeneficialInterest,
        Self::ConvertibleDebentures,
        Self::Units,
        Self::UnitsInvestmentTrusts,
        Self::Warrants,
        Self::IndexWarrants,
        Self::Adrs,
        Self::Miscellaneous,
    ];

    pub const ALL_CHARS: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub const NAME: &str = "IssueClassification";

    const LUT: [Option<IssueClassification>; 256] = {
        let mut lut = [None; 256];
        lut[b'A' as usize] = Some(Self::AmericanDepositaryShares);
        lut[b'B' as usize] = Some(Self::Bonds);
        lut[b'C' as usize] = Some(Self::CommonStock);
        lut[b'D' as usize] = Some(Self::DepositoryReceipts);
        lut[b'E' as usize] = Some(Self::EquityBasedEtf);
        lut[b'F' as usize] = Some(Self::Debentures);
        lut[b'G' as usize] = Some(Self::GlobalShares);
        lut[b'H' as usize] = Some(Self::FixedIncomeEtf);
        lut[b'I' as usize] = Some(Self::FixedIncomeEtnNasdaq);
        lut[b'J' as usize] = Some(Self::EquityBasedEtnNyse);
        lut[b'K' as usize] = Some(Self::FixedIncomeEtnNyse);
        lut[b'L' as usize] = Some(Self::EquityBasedEtnNasdaq);
        lut[b'M' as usize] = Some(Self::MutualFunds);
        lut[b'N' as usize] = Some(Self::Notes);
        lut[b'O' as usize] = Some(Self::OrdinaryShares);
        lut[b'P' as usize] = Some(Self::PreferredShares);
        lut[b'Q' as usize] = Some(Self::OtherSecurities);
        lut[b'R' as usize] = Some(Self::Rights);
        lut[b'S' as usize] = Some(Self::SharesOfBeneficialInterest);
        lut[b'T' as usize] = Some(Self::ConvertibleDebentures);
        lut[b'U' as usize] = Some(Self::Units);
        lut[b'V' as usize] = Some(Self::UnitsInvestmentTrusts);
        lut[b'W' as usize] = Some(Self::Warrants);
        lut[b'X' as usize] = Some(Self::IndexWarrants);
        lut[b'Y' as usize] = Some(Self::Adrs);
        lut[b'Z' as usize] = Some(Self::Miscellaneous);
        lut
    };

    #[inline]
    pub fn try_from_byte(raw: u8) -> Result<IssueClassification> {
        Self::from_byte(raw).ok_or(ItchError::InvalidCharField {
            field: Self::NAME,
            expected: &Self::ALL_CHARS,
            actual: raw as char,
        })
    }

    #[inline(always)]
    pub fn from_byte(b: u8) -> Option<IssueClassification> {
        Self::LUT[b as usize]
    }
}

