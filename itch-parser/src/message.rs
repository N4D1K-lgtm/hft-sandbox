use crate::prelude::{
    FinancialStatusIndicator, ItchError, MarketCategory, Result, SystemEventCode,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ItchMessage {
    /// **System Event Message**
    ///
    /// Used to signal a market or data feed handler event.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `S`        | System Event Message            |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Always 0                        |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Event Code      | 11     | 1      | [`char`]   | See [`SystemEventCode`]         |
    SystemEvent {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        event_code: SystemEventCode,
    },

    /// **Stock Directory**
    ///
    /// At the start of each trading day, Nasdaq disseminates stock directory messages for
    /// for all active symbols in the Nasdaq execution system.
    ///
    /// > *Market data redistributors should process this message to populate the
    /// > **Financial Status Indicator** (required display field) and the **Market Category**
    /// > (recommended display field) for Nasdaq listed issues.*
    ///
    /// | Name             | Offset | Length | Value   | Notes                           |
    /// |------------------|--------|--------|---------|---------------------------------|
    /// |  Message Type    | 0      | 1      | 'R'     | Stock Directory Message         |
    /// |  Stock Locate    | 1      | 2      | [`u16`] | Locate Code uniquely assigned to the security symbol for the day |
    /// |  Tracking Number | 3      | 2      | [`u16`] | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Time at which the directory message was generated    |
    /// |  Stock           | 11     | 8      | `[char; 8]`| Denotes the security symbol for the issue in the Nasdaq execution system.  |
    /// |  Market Category | 19     | 1      | [`char`]   | Indicates Listing market or market tier for the issue |
    /// |  Financial Status Indicator | 20     | 1      | [`char`]   | Indicates Listing market or market tier for the issue |
    /// |  Round Lot Size  | 21     | 4      | [`u32`]  | Denotes the number of shares that represent a round lot for the issue. |
    /// |  Round Lots Only | 25     | 1      | [`char`]   | Indicates if Nasdaq system limits order entry for issue. |
    /// |  Issue Classification | 26 | 1      | [`char`]   | Identifies the security class for issue as assigned by Nasdaq. |
    /// |  Issue Sub Type | 27     | 2      | `[char; 2]`   | Identifies the security sub-type for issue as assigned by Nasdaq. |
    /// |  Authenticity   | 29     | 1      | [`char`]   | Denotes if an issue or quoting participant record is set-up in Nasdaq systems in a live/production, test, or demo state. |
    StockDirectory {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        market_category: MarketCategory,
        financial_status: FinancialStatusIndicator,
        round_lot_size: u32,
        round_lots_only: u8,
        issue_classification: u8,
        issue_sub_type: String,
        authenticity: u8,
        short_sale_threshold: u8,
        ipo_flag: u8,
        luld_reference_price_tier: u8,
        etp_flag: u8,
        etp_leverage_factor: u32,
        inverse_indicator: u8,
    },
    StockTradingAction {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        trading_state: u8,
        reserved: u8,
        reason: String,
    },
    RegShoRestriction {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        reg_sho_action: u8,
    },
    MarketParticipantPosition {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        mpid: String,
        stock: String,
        primary_market_maker: u8,
        market_maker_mode: u8,
        market_participant_state: u8,
    },
    MwcbDeclineLevel {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        level_1: u64,
        level_2: u64,
        level_3: u64,
    },
    MwcbStatus {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        breached_level: u8,
    },
    IpoQuotingPeriodUpdate {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        ipo_quotation_release_time: u32,
        ipo_quotation_release_qualifier: u8,
        ipo_price: u32,
    },
    AddOrder {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        buy_sell_indicator: u8,
        shares: u32,
        stock: String,
        price: u32,
    },
    AddOrderMpid {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        buy_sell_indicator: u8,
        shares: u32,
        stock: String,
        price: u32,
        attribution: String,
    },
    OrderExecuted {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        executed_shares: u32,
        match_number: u64,
    },
    OrderExecutedWithPrice {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        executed_shares: u32,
        match_number: u64,
        printable: u8,
        execution_price: u32,
    },
    OrderCancel {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        cancelled_shares: u32,
    },
    OrderDelete {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
    },
    OrderReplace {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        original_order_reference_number: u64,
        new_order_reference_number: u64,
        shares: u32,
        price: u32,
    },
    Trade {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        buy_sell_indicator: u8,
        shares: u32,
        stock: String,
        price: u32,
        match_number: u64,
    },
    CrossTrade {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        shares: u64,
        stock: String,
        cross_price: u32,
        match_number: u64,
        cross_type: u8,
    },
    BrokenTrade {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        match_number: u64,
    },
    NetOrderImbalanceIndicator {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        paired_shares: u64,
        imbalance_shares: u64,
        imbalance_direction: u8,
        stock: String,
        far_price: u32,
        near_price: u32,
        current_reference_price: u32,
        cross_type: u8,
        price_variation_indicator: u8,
    },
    RetailPriceImprovementIndicator {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        interest_flag: u8,
    },
}
