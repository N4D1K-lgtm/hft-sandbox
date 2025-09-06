use crate::prelude::{
    Authenticity, BuySellIndicator, CrossType, EtpFlag, FinancialStatusIndicator,
    ImbalanceDirection, InterestFlag, InverseIndicator, IpoFlag, IpoQuotationReleaseQualifier,
    IssueClassification, LuldReferencePriceTier, MarketCategory, MarketMakerMode,
    MarketParticipantState, PriceVariationIndicator, PrimaryMarketMaker, Printable, RegShoAction,
    RoundLotsOnly, ShortSaleThreshold, SystemEventCode, TradingState,
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
    /// all active symbols in the Nasdaq execution system.
    ///
    /// > *Market data redistributors should process this message to populate the
    /// > **Financial Status Indicator** (required display field) and the **Market Category**
    /// > (recommended display field) for Nasdaq listed issues.*
    ///
    /// | Name                        | Offset | Length | Type       | Notes                           |
    /// |-----------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type               | 0      | 1      | `R`        | Stock Directory Message         |
    /// |  Stock Locate               | 1      | 2      | [`u16`]    | Locate code uniquely assigned to the security symbol for the day |
    /// |  Tracking Number            | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp                  | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Stock                      | 11     | 8      | `String`   | Security symbol, right padded with spaces |
    /// |  Market Category            | 19     | 1      | [`char`]   | See [`MarketCategory`]          |
    /// |  Financial Status Indicator | 20     | 1      | [`char`]   | See [`FinancialStatusIndicator`] |
    /// |  Round Lot Size             | 21     | 4      | [`u32`]    | Number of shares in a round lot |
    /// |  Round Lots Only            | 25     | 1      | [`char`]   | See [`RoundLotsOnly`]           |
    /// |  Issue Classification       | 26     | 1      | [`char`]   | See [`IssueClassification`]     |
    /// |  Issue Sub Type             | 27     | 2      | `String`   | Security sub-type identifier    |
    /// |  Authenticity               | 29     | 1      | [`char`]   | See [`Authenticity`]            |
    /// |  Short Sale Threshold       | 30     | 1      | [`char`]   | See [`ShortSaleThreshold`]      |
    /// |  IPO Flag                   | 31     | 1      | [`char`]   | See [`IpoFlag`]                 |
    /// |  LULD Reference Price Tier  | 32     | 1      | [`char`]   | See [`LuldReferencePriceTier`]  |
    /// |  ETP Flag                   | 33     | 1      | [`char`]   | See [`EtpFlag`]                 |
    /// |  ETP Leverage Factor        | 34     | 4      | [`u32`]    | Leverage factor for ETPs        |
    /// |  Inverse Indicator          | 38     | 1      | [`char`]   | See [`InverseIndicator`]        |
    StockDirectory {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        market_category: MarketCategory,
        financial_status: FinancialStatusIndicator,
        round_lot_size: u32,
        round_lots_only: RoundLotsOnly,
        issue_classification: IssueClassification,
        issue_sub_type: String,
        authenticity: Authenticity,
        short_sale_threshold: ShortSaleThreshold,
        ipo_flag: IpoFlag,
        luld_reference_price_tier: LuldReferencePriceTier,
        etp_flag: EtpFlag,
        etp_leverage_factor: u32,
        inverse_indicator: InverseIndicator,
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
    /// **Market Participant Position**
    ///
    /// Sent at the start of each trading day for all active market participants.
    /// Provides information about market maker status and mode.
    ///
    /// | Name                     | Offset | Length | Type       | Notes                           |
    /// |--------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type            | 0      | 1      | `L`        | Market Participant Position     |
    /// |  Stock Locate            | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number         | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp               | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  MPID                    | 11     | 4      | `String`   | Market participant identifier   |
    /// |  Stock                   | 15     | 8      | `String`   | Stock symbol                    |
    /// |  Primary Market Maker    | 23     | 1      | [`char`]   | See [`PrimaryMarketMaker`]      |
    /// |  Market Maker Mode       | 24     | 1      | [`char`]   | See [`MarketMakerMode`]         |
    /// |  Market Participant State| 25     | 1      | [`char`]   | See [`MarketParticipantState`]  |
    MarketParticipantPosition {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        mpid: String,
        stock: String,
        primary_market_maker: PrimaryMarketMaker,
        market_maker_mode: MarketMakerMode,
        market_participant_state: MarketParticipantState,
    },
    /// **MWCB Decline Level Message**
    ///
    /// Sent when MWCB (Market Wide Circuit Breaker) decline levels are updated.
    /// Contains the three MWCB decline levels.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `V`        | MWCB Decline Level Message      |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Always 0                        |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Level 1         | 11     | 8      | [`u64`]    | Level 1 decline threshold       |
    /// |  Level 2         | 19     | 8      | [`u64`]    | Level 2 decline threshold       |
    /// |  Level 3         | 27     | 8      | [`u64`]    | Level 3 decline threshold       |
    MwcbDeclineLevel {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        level_1: u64,
        level_2: u64,
        level_3: u64,
    },
    /// **MWCB Status Message**
    ///
    /// Sent when a MWCB (Market Wide Circuit Breaker) is breached.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `W`        | MWCB Status Message             |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Always 0                        |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Breached Level  | 11     | 1      | [`u8`]     | Level that was breached (1, 2, or 3) |
    MwcbStatus {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        breached_level: u8,
    },
    /// **IPO Quoting Period Update**
    ///
    /// Sent during the IPO quoting period to provide updates on the IPO release.
    ///
    /// | Name                           | Offset | Length | Type       | Notes                           |
    /// |--------------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type                  | 0      | 1      | `K`        | IPO Quoting Period Update       |
    /// |  Stock Locate                  | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number               | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp                     | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Stock                         | 11     | 8      | `String`   | Stock symbol                    |
    /// |  IPO Quotation Release Time    | 19     | 4      | [`u32`]    | Expected release time           |
    /// |  IPO Quotation Release Qualifier| 23     | 1      | [`char`]   | See [`IpoQuotationReleaseQualifier`] |
    /// |  IPO Price                     | 24     | 4      | [`u32`]    | IPO price (4 decimal places)   |
    IpoQuotingPeriodUpdate {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        ipo_quotation_release_time: u32,
        ipo_quotation_release_qualifier: IpoQuotationReleaseQualifier,
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
    /// **Order Executed Message**
    ///
    /// Sent when an order on the book is executed in whole or in part.
    /// The execution price is the same as the original order price.
    ///
    /// | Name                   | Offset | Length | Type       | Notes                           |
    /// |------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type          | 0      | 1      | `E`        | Order Executed Message          |
    /// |  Stock Locate          | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number       | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp             | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Order Reference Number| 11     | 8      | [`u64`]    | Reference to original order     |
    /// |  Executed Shares       | 19     | 4      | [`u32`]    | Number of shares executed       |
    /// |  Match Number          | 23     | 8      | [`u64`]    | Nasdaq generated match number   |
    OrderExecuted {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        executed_shares: u32,
        match_number: u64,
    },
    /// **Order Executed With Price Message**
    ///
    /// Sent when an order is executed at a price different from the original order price.
    /// This can occur during opening/closing crosses or when hidden orders are executed.
    ///
    /// | Name                   | Offset | Length | Type       | Notes                           |
    /// |------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type          | 0      | 1      | `C`        | Order Executed With Price       |
    /// |  Stock Locate          | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number       | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp             | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Order Reference Number| 11     | 8      | [`u64`]    | Reference to original order     |
    /// |  Executed Shares       | 19     | 4      | [`u32`]    | Number of shares executed       |
    /// |  Match Number          | 23     | 8      | [`u64`]    | Nasdaq generated match number   |
    /// |  Printable             | 31     | 1      | [`char`]   | See [`Printable`]               |
    /// |  Execution Price       | 32     | 4      | [`u32`]    | Execution price (4 decimals)    |
    OrderExecutedWithPrice {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        executed_shares: u32,
        match_number: u64,
        printable: Printable,
        execution_price: u32,
    },
    /// **Order Cancel Message**
    ///
    /// Sent when an order on the book is being canceled.
    /// The order may be partially or completely canceled.
    ///
    /// | Name                   | Offset | Length | Type       | Notes                           |
    /// |------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type          | 0      | 1      | `X`        | Order Cancel Message            |
    /// |  Stock Locate          | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number       | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp             | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Order Reference Number| 11     | 8      | [`u64`]    | Reference to original order     |
    /// |  Cancelled Shares      | 19     | 4      | [`u32`]    | Number of shares cancelled      |
    OrderCancel {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
        cancelled_shares: u32,
    },
    /// **Order Delete Message**
    ///
    /// Sent when an order on the book is being deleted in its entirety.
    /// This message is used when the entire remaining quantity is removed.
    ///
    /// | Name                   | Offset | Length | Type       | Notes                           |
    /// |------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type          | 0      | 1      | `D`        | Order Delete Message            |
    /// |  Stock Locate          | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number       | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp             | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Order Reference Number| 11     | 8      | [`u64`]    | Reference to original order     |
    OrderDelete {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        order_reference_number: u64,
    },
    /// **Order Replace Message**
    ///
    /// Sent when an order on the book is being replaced.
    /// The original order is deleted and a new order is added with new attributes.
    ///
    /// | Name                           | Offset | Length | Type       | Notes                           |
    /// |--------------------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type                  | 0      | 1      | `U`        | Order Replace Message           |
    /// |  Stock Locate                  | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number               | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp                     | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Original Order Reference Number| 11     | 8      | [`u64`]    | Reference to original order     |
    /// |  New Order Reference Number    | 19     | 8      | [`u64`]    | Reference to new order          |
    /// |  Shares                        | 27     | 4      | [`u32`]    | New number of shares            |
    /// |  Price                         | 31     | 4      | [`u32`]    | New price (4 decimal places)   |
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
    /// **Broken Trade Message**
    ///
    /// Sent when a trade is broken (canceled) by Nasdaq.
    /// References the original match number of the broken trade.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `B`        | Broken Trade Message            |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Match Number    | 11     | 8      | [`u64`]    | Match number of broken trade    |
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
        price_variation_indicator: PriceVariationIndicator,
    },
    /// **Retail Price Improvement Indicator (RPII)**
    ///
    /// Sent when there is a change in the availability of Retail Price Improvement.
    /// Indicates whether RPI is available for retail orders.
    ///
    /// | Name             | Offset | Length | Type       | Notes                           |
    /// |------------------|--------|--------|------------|---------------------------------|
    /// |  Message Type    | 0      | 1      | `N`        | RPII Message                    |
    /// |  Stock Locate    | 1      | 2      | [`u16`]    | Locate code identifying the security |
    /// |  Tracking Number | 3      | 2      | [`u16`]    | Nasdaq internal tracking number |
    /// |  Timestamp       | 5      | 6      | [`u64`]    | Nanoseconds since midnight      |
    /// |  Stock           | 11     | 8      | `String`   | Stock symbol                    |
    /// |  Interest Flag   | 19     | 1      | [`char`]   | See [`InterestFlag`]            |
    RetailPriceImprovementIndicator {
        stock_locate: u16,
        tracking_number: u16,
        timestamp: u64,
        stock: String,
        interest_flag: InterestFlag,
    },
}
