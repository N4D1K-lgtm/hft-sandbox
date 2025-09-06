use crate::prelude::{
    Authenticity, BuySellIndicator, CrossType, FinancialStatusIndicator, ImbalanceDirection,
    IssueClassification, MarketCategory, RegShoAction, SystemEventCode, TradingState,
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
        issue_classification: IssueClassification,
        issue_sub_type: String,
        authenticity: Authenticity,
        short_sale_threshold: u8,
        ipo_flag: u8,
        luld_reference_price_tier: u8,
        etp_flag: u8,
        etp_leverage_factor: u32,
        inverse_indicator: u8,
    },
    /// **Stock Trading Action**
    ///
    /// Sent whenever Nasdaq halts or resumes trading in a security.
    /// Recipients should note that a Trading Action message with "H" (Halted) or "P" (Paused)
    /// is sent when Nasdaq halts or suspends trading.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `H`        | Stock Trading Action Message    |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Stock           | 11     | 8      | `String`   | Stock symbol, right padded with spaces |
    /// |  Trading State   | 19     | 1      | [`char`]   | See [`TradingState`]            |
    /// |  Reserved        | 20     | 1      | [`u8`]     | Reserved, always 0              |
    /// |  Reason          | 21     | 4      | `String`   | Trading action reason           |
    StockTradingAction {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        trading_state: TradingState,
        reserved: u8,
        reason: String,
    },
    /// **Reg SHO Short Sale Price Test Restriction**
    ///
    /// Sent when Nasdaq Regulation imposes or removes a short sale price test restriction.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `Y`        | Reg SHO Restriction Message     |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Stock           | 11     | 8      | `String`   | Stock symbol, right padded with spaces |
    /// |  Reg SHO Action  | 19     | 1      | [`u8`]     | 0 = No price test, 1 = Price test restriction |
    RegShoRestriction {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        reg_sho_action: RegShoAction,
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
    /// **Add Order Message**
    ///
    /// Sent when a new order is accepted into the Nasdaq system.
    ///
    /// | Name                   | Offset | Length | Type       | Notes                           |
    /// |------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type          | 0      | 1      | `A`        | Add Order Message               |
    /// |  Stock Locate          | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number       | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp             | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Order Reference Number| 11     | 8      | [`u64`]    | Unique reference number         |
    /// |  Buy/Sell Indicator    | 19     | 1      | [`char`]   | See [`BuySellIndicator`]        |
    /// |  Shares                | 20     | 4      | [`u32`]    | Number of shares                |
    /// |  Stock                 | 24     | 8      | `String`   | Stock symbol                    |
    /// |  Price                 | 32     | 4      | [`u32`]    | Price (4 decimal places)        |
    AddOrder {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        buy_sell_indicator: BuySellIndicator,
        shares: u32,
        stock: String,
        price: u32,
    },
    /// **Add Order with MPID Attribution**
    ///
    /// Similar to Add Order but includes attribution information.
    ///
    /// | Name                   | Offset | Length | Type       | Notes                           |
    /// |------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type          | 0      | 1      | `F`        | Add Order with MPID Message     |
    /// |  Stock Locate          | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number       | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp             | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Order Reference Number| 11     | 8      | [`u64`]    | Unique reference number         |
    /// |  Buy/Sell Indicator    | 19     | 1      | [`char`]   | See [`BuySellIndicator`]        |
    /// |  Shares                | 20     | 4      | [`u32`]    | Number of shares                |
    /// |  Stock                 | 24     | 8      | `String`   | Stock symbol                    |
    /// |  Price                 | 32     | 4      | [`u32`]    | Price (4 decimal places)        |
    /// |  Attribution           | 36     | 4      | `String`   | Market participant identifier   |
    AddOrderMpid {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        buy_sell_indicator: BuySellIndicator,
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
    /// **Trade Message (Non-Cross)**
    ///
    /// Provides details of executions of non-displayable orders or shares from reserve.
    ///
    /// | Name                   | Offset | Length | Type       | Notes                           |
    /// |------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type          | 0      | 1      | `P`        | Trade Message                   |
    /// |  Stock Locate          | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number       | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp             | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Order Reference Number| 11     | 8      | [`u64`]    | Reference to original order     |
    /// |  Buy/Sell Indicator    | 19     | 1      | [`char`]   | See [`BuySellIndicator`]        |
    /// |  Shares                | 20     | 4      | [`u32`]    | Number of shares executed       |
    /// |  Stock                 | 24     | 8      | `String`   | Stock symbol                    |
    /// |  Price                 | 32     | 4      | [`u32`]    | Execution price (4 decimals)    |
    /// |  Match Number          | 36     | 8      | [`u64`]    | Nasdaq generated match number   |
    Trade {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        buy_sell_indicator: BuySellIndicator,
        shares: u32,
        stock: String,
        price: u32,
        match_number: u64,
    },
    /// **Cross Trade Message**
    ///
    /// Sent when Nasdaq executes a cross trade.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `Q`        | Cross Trade Message             |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Shares          | 11     | 8      | [`u64`]    | Number of shares crossed        |
    /// |  Stock           | 19     | 8      | `String`   | Stock symbol                    |
    /// |  Cross Price     | 27     | 4      | [`u32`]    | Price at which shares crossed   |
    /// |  Match Number    | 31     | 8      | [`u64`]    | Nasdaq match number             |
    /// |  Cross Type      | 39     | 1      | [`char`]   | See [`CrossType`]               |
    CrossTrade {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        shares: u64,
        stock: String,
        cross_price: u32,
        match_number: u64,
        cross_type: CrossType,
    },
    BrokenTrade {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        match_number: u64,
    },
    /// **Net Order Imbalance Indicator (NOII)**
    ///
    /// Disseminated prior to the opening and closing crosses.
    /// Provides order imbalance information.
    ///
    /// | Name                      | Offset | Length | Type       | Notes                           |
    /// |---------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type             | 0      | 1      | `I`        | NOII Message                    |
    /// |  Stock Locate             | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number          | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp                | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Paired Shares            | 11     | 8      | [`u64`]    | Number of shares paired         |
    /// |  Imbalance Shares         | 19     | 8      | [`u64`]    | Number of shares imbalanced     |
    /// |  Imbalance Direction      | 27     | 1      | [`char`]   | See [`ImbalanceDirection`]      |
    /// |  Stock                    | 28     | 8      | `String`   | Stock symbol                    |
    /// |  Far Price                | 36     | 4      | [`u32`]    | Far clearing price              |
    /// |  Near Price               | 40     | 4      | [`u32`]    | Near clearing price             |
    /// |  Current Reference Price  | 44     | 4      | [`u32`]    | Current reference price         |
    /// |  Cross Type               | 48     | 1      | [`char`]   | See [`CrossType`]               |
    /// |  Price Variation Indicator| 49     | 1      | [`u8`]     | Price variation indicator       |
    NetOrderImbalanceIndicator {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        paired_shares: u64,
        imbalance_shares: u64,
        imbalance_direction: ImbalanceDirection,
        stock: String,
        far_price: u32,
        near_price: u32,
        current_reference_price: u32,
        cross_type: CrossType,
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
