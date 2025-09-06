use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::prelude::{
    Authenticity, BuySellIndicator, CrossType, EtpFlag, FinancialStatusIndicator,
    ImbalanceDirection, InterestFlag, InverseIndicator, IpoFlag, IpoQuotationReleaseQualifier,
    IssueClassification, ItchError, ItchMessage, LuldReferencePriceTier, MarketCategory,
    MarketMakerMode, MarketParticipantState, PriceVariationIndicator, PrimaryMarketMaker,
    Printable, RegShoAction, Result, RoundLotsOnly, ShortSaleThreshold, SystemEventCode,
    TradingState,
};

#[derive(Default)]
pub struct ItchParser;

impl ItchParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_message(&self, data: &[u8]) -> Result<ItchMessage> {
        if data.is_empty() {
            return Err(ItchError::InvalidLength {
                expected: 1,
                actual: 0,
            });
        }

        let message_type = data[0];
        let mut cursor = Cursor::new(&data[1..]);

        match message_type {
            b'S' => self.parse_system_event(&mut cursor),
            b'R' => self.parse_stock_directory(&mut cursor),
            b'H' => self.parse_stock_trading_action(&mut cursor),
            b'Y' => self.parse_reg_sho_restriction(&mut cursor),
            b'L' => self.parse_market_participant_position(&mut cursor),
            b'V' => self.parse_mwcb_decline_level(&mut cursor),
            b'W' => self.parse_mwcb_status(&mut cursor),
            b'K' => self.parse_ipo_quoting_period_update(&mut cursor),
            b'A' => self.parse_add_order(&mut cursor),
            b'F' => self.parse_add_order_mpid(&mut cursor),
            b'E' => self.parse_order_executed(&mut cursor),
            b'C' => self.parse_order_executed_with_price(&mut cursor),
            b'X' => self.parse_order_cancel(&mut cursor),
            b'D' => self.parse_order_delete(&mut cursor),
            b'U' => self.parse_order_replace(&mut cursor),
            b'P' => self.parse_trade(&mut cursor),
            b'Q' => self.parse_cross_trade(&mut cursor),
            b'B' => self.parse_broken_trade(&mut cursor),
            b'I' => self.parse_net_order_imbalance_indicator(&mut cursor),
            b'N' => self.parse_retail_price_improvement_indicator(&mut cursor),
            _ => Err(ItchError::InvalidMessageType(message_type)),
        }
    }

    fn read_string(cursor: &mut Cursor<&[u8]>, len: usize) -> Result<String> {
        let mut buf = vec![0u8; len];
        cursor.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).trim().to_string())
    }

    fn parse_system_event(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::SystemEvent {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            event_code: SystemEventCode::try_from_byte(cursor.read_u8()?)?,
        })
    }

    fn parse_stock_directory(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::StockDirectory {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            stock: Self::read_string(cursor, 8)?,
            market_category: MarketCategory::try_from_byte(cursor.read_u8()?)?,
            financial_status: FinancialStatusIndicator::try_from_byte(cursor.read_u8()?)?,
            round_lot_size: cursor.read_u32::<BigEndian>()?,
            round_lots_only: RoundLotsOnly::try_from_byte(cursor.read_u8()?)?,
            issue_classification: IssueClassification::try_from_byte(cursor.read_u8()?)?,
            issue_sub_type: Self::read_string(cursor, 2)?,
            authenticity: Authenticity::try_from_byte(cursor.read_u8()?)?,
            short_sale_threshold: ShortSaleThreshold::try_from_byte(cursor.read_u8()?)?,
            ipo_flag: IpoFlag::try_from_byte(cursor.read_u8()?)?,
            luld_reference_price_tier: LuldReferencePriceTier::try_from_byte(cursor.read_u8()?)?,
            etp_flag: EtpFlag::try_from_byte(cursor.read_u8()?)?,
            etp_leverage_factor: cursor.read_u32::<BigEndian>()?,
            inverse_indicator: InverseIndicator::try_from_byte(cursor.read_u8()?)?,
        })
    }

    fn parse_stock_trading_action(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::StockTradingAction {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            stock: Self::read_string(cursor, 8)?,
            trading_state: TradingState::try_from_byte(cursor.read_u8()?)?,
            reserved: cursor.read_u8()?,
            reason: Self::read_string(cursor, 4)?,
        })
    }

    fn parse_reg_sho_restriction(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::RegShoRestriction {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            stock: Self::read_string(cursor, 8)?,
            reg_sho_action: RegShoAction::try_from_byte(cursor.read_u8()?)?,
        })
    }

    fn parse_market_participant_position(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::MarketParticipantPosition {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            mpid: Self::read_string(cursor, 4)?,
            stock: Self::read_string(cursor, 8)?,
            primary_market_maker: PrimaryMarketMaker::try_from_byte(cursor.read_u8()?)?,
            market_maker_mode: MarketMakerMode::try_from_byte(cursor.read_u8()?)?,
            market_participant_state: MarketParticipantState::try_from_byte(cursor.read_u8()?)?,
        })
    }

    fn parse_mwcb_decline_level(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::MwcbDeclineLevel {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            level_1: cursor.read_u64::<BigEndian>()?,
            level_2: cursor.read_u64::<BigEndian>()?,
            level_3: cursor.read_u64::<BigEndian>()?,
        })
    }

    fn parse_mwcb_status(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::MwcbStatus {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            breached_level: cursor.read_u8()?,
        })
    }

    fn parse_ipo_quoting_period_update(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::IpoQuotingPeriodUpdate {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            stock: Self::read_string(cursor, 8)?,
            ipo_quotation_release_time: cursor.read_u32::<BigEndian>()?,
            ipo_quotation_release_qualifier: IpoQuotationReleaseQualifier::try_from_byte(cursor.read_u8()?)?,
            ipo_price: cursor.read_u32::<BigEndian>()?,
        })
    }

    fn parse_add_order(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::AddOrder {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            order_reference_number: cursor.read_u64::<BigEndian>()?,
            buy_sell_indicator: BuySellIndicator::try_from_byte(cursor.read_u8()?)?,
            shares: cursor.read_u32::<BigEndian>()?,
            stock: Self::read_string(cursor, 8)?,
            price: cursor.read_u32::<BigEndian>()?,
        })
    }

    fn parse_add_order_mpid(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::AddOrderMpid {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            order_reference_number: cursor.read_u64::<BigEndian>()?,
            buy_sell_indicator: BuySellIndicator::try_from_byte(cursor.read_u8()?)?,
            shares: cursor.read_u32::<BigEndian>()?,
            stock: Self::read_string(cursor, 8)?,
            price: cursor.read_u32::<BigEndian>()?,
            attribution: Self::read_string(cursor, 4)?,
        })
    }

    fn parse_order_executed(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::OrderExecuted {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            order_reference_number: cursor.read_u64::<BigEndian>()?,
            executed_shares: cursor.read_u32::<BigEndian>()?,
            match_number: cursor.read_u64::<BigEndian>()?,
        })
    }

    fn parse_order_executed_with_price(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::OrderExecutedWithPrice {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            order_reference_number: cursor.read_u64::<BigEndian>()?,
            executed_shares: cursor.read_u32::<BigEndian>()?,
            match_number: cursor.read_u64::<BigEndian>()?,
            printable: Printable::try_from_byte(cursor.read_u8()?)?,
            execution_price: cursor.read_u32::<BigEndian>()?,
        })
    }

    fn parse_order_cancel(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::OrderCancel {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            order_reference_number: cursor.read_u64::<BigEndian>()?,
            cancelled_shares: cursor.read_u32::<BigEndian>()?,
        })
    }

    fn parse_order_delete(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::OrderDelete {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            order_reference_number: cursor.read_u64::<BigEndian>()?,
        })
    }

    fn parse_order_replace(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::OrderReplace {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            original_order_reference_number: cursor.read_u64::<BigEndian>()?,
            new_order_reference_number: cursor.read_u64::<BigEndian>()?,
            shares: cursor.read_u32::<BigEndian>()?,
            price: cursor.read_u32::<BigEndian>()?,
        })
    }

    fn parse_trade(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::Trade {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            order_reference_number: cursor.read_u64::<BigEndian>()?,
            buy_sell_indicator: BuySellIndicator::try_from_byte(cursor.read_u8()?)?,
            shares: cursor.read_u32::<BigEndian>()?,
            stock: Self::read_string(cursor, 8)?,
            price: cursor.read_u32::<BigEndian>()?,
            match_number: cursor.read_u64::<BigEndian>()?,
        })
    }

    fn parse_cross_trade(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::CrossTrade {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            shares: cursor.read_u64::<BigEndian>()?,
            stock: Self::read_string(cursor, 8)?,
            cross_price: cursor.read_u32::<BigEndian>()?,
            match_number: cursor.read_u64::<BigEndian>()?,
            cross_type: CrossType::try_from_byte(cursor.read_u8()?)?,
        })
    }

    fn parse_broken_trade(&self, cursor: &mut Cursor<&[u8]>) -> Result<ItchMessage> {
        Ok(ItchMessage::BrokenTrade {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            match_number: cursor.read_u64::<BigEndian>()?,
        })
    }

    fn parse_net_order_imbalance_indicator(
        &self,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<ItchMessage> {
        Ok(ItchMessage::NetOrderImbalanceIndicator {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            paired_shares: cursor.read_u64::<BigEndian>()?,
            imbalance_shares: cursor.read_u64::<BigEndian>()?,
            imbalance_direction: ImbalanceDirection::try_from_byte(cursor.read_u8()?)?,
            stock: Self::read_string(cursor, 8)?,
            far_price: cursor.read_u32::<BigEndian>()?,
            near_price: cursor.read_u32::<BigEndian>()?,
            current_reference_price: cursor.read_u32::<BigEndian>()?,
            cross_type: CrossType::try_from_byte(cursor.read_u8()?)?,
            price_variation_indicator: PriceVariationIndicator::try_from_byte(cursor.read_u8()?)?,
        })
    }

    fn parse_retail_price_improvement_indicator(
        &self,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<ItchMessage> {
        Ok(ItchMessage::RetailPriceImprovementIndicator {
            stock_locate: cursor.read_u16::<BigEndian>()?,
            tracking_number: cursor.read_u16::<BigEndian>()?,
            timestamp: ReadBytesExtU48::read_u48::<BigEndian>(cursor)?,
            stock: Self::read_string(cursor, 8)?,
            interest_flag: InterestFlag::try_from_byte(cursor.read_u8()?)?,
        })
    }
}

trait ReadBytesExtU48 {
    fn read_u48<T: byteorder::ByteOrder>(&mut self) -> std::io::Result<u64>;
}

impl<R: ReadBytesExt> ReadBytesExtU48 for R {
    fn read_u48<T: byteorder::ByteOrder>(&mut self) -> std::io::Result<u64> {
        let mut buf = [0u8; 6];
        self.read_exact(&mut buf)?;
        Ok(T::read_u48(&buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_system_event() {
        let parser = ItchParser::new();
        let data = vec![
            b'S', // message type
            0x00, 0x01, // stock locate
            0x00, 0x02, // tracking number
            0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // timestamp (48-bit)
            b'O', // event code
        ];

        let result = parser.parse_message(&data).unwrap();
        match result {
            ItchMessage::SystemEvent {
                stock_locate,
                tracking_number,
                timestamp,
                event_code,
            } => {
                assert_eq!(stock_locate, 1);
                assert_eq!(tracking_number, 2);
                assert_eq!(timestamp, 3);
                assert_eq!(event_code, SystemEventCode::try_from_byte(b'O').unwrap());
            }
            _ => panic!("Expected SystemEvent"),
        }
    }

    #[test]
    fn test_parse_add_order() {
        let parser = ItchParser::new();
        let mut data = vec![
            b'A', // message type
            0x00, 0x01, // stock locate
            0x00, 0x02, // tracking number
            0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // timestamp (48-bit)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, // order reference number
            b'B', // buy/sell indicator
            0x00, 0x00, 0x00, 0x64, // shares (100)
        ];
        data.extend_from_slice(b"AAPL    "); // stock (8 chars)
        data.extend_from_slice(&[0x00, 0x00, 0x27, 0x10]); // price

        let result = parser.parse_message(&data).unwrap();
        match result {
            ItchMessage::AddOrder {
                stock_locate,
                tracking_number,
                timestamp,
                order_reference_number,
                buy_sell_indicator,
                shares,
                stock,
                price,
            } => {
                assert_eq!(stock_locate, 1);
                assert_eq!(tracking_number, 2);
                assert_eq!(timestamp, 3);
                assert_eq!(order_reference_number, 4);
                assert_eq!(buy_sell_indicator, BuySellIndicator::Buy);
                assert_eq!(shares, 100);
                assert_eq!(stock, "AAPL");
                assert_eq!(price, 10000);
            }
            _ => panic!("Expected AddOrder"),
        }
    }
}
