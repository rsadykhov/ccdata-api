use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


/// The exchange to obtain data from.
pub enum CCOptionsMarket {
    DERIBIT,
    OKEX,
}

impl Market for CCOptionsMarket {
    /// Converts enum values to `String`.
    fn to_string(&self) -> String {
        match self {
            CCOptionsMarket::DERIBIT => String::from("deribit"),
            CCOptionsMarket::OKEX => String::from("okex"),
        }
    }
}


// Options: Historical OHLCV+


///Options: Historical OHLCV+
#[derive(Deserialize, Debug)]
pub struct CCOptionsOHLCV {
    #[serde(rename = "UNIT")]
    /// The level of granularity (e.g. MINUTE / HOUR / DAY).
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp, in seconds, of the histo period. This refers to the first timestamp of the unit under consideration,
    /// not the last (e.g. for daily data the timestamp will refer to 00:00 GMT/UTC).
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "MARKET")]
    /// The market / exchange under consideration (e.g. bitmex, deribit, ftx, etc).
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    /// The unmapped instrument ID.
    pub instrument: String,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    /// The mapped instrument ID, derived from our mapping rules.
    pub mapped_instrument: String,
    #[serde(rename = "INDEX_UNDERLYING")]
    /// The mapped index underlying asset.
    pub index_underlying: String,
    #[serde(rename = "QUOTE_CURRENCY")]
    /// The mapped to asset quote / counter symbol / coin (e.g. USD). Only available on instruments that have mapping.
    pub quote_currency: String,
    #[serde(rename = "SETTLEMENT_CURRENCY")]
    /// The currency that the contract is settled in (e.g. USD). Only available on instruments that have mapping.
    pub settlement_currency: String,
    #[serde(rename = "CONTRACT_CURRENCY")]
    /// The currency that the contract size is denominated in (e.g. USD). Only available on instruments that have mapping.
    pub contract_currency: String,
    #[serde(rename = "STRIKE_CURRENCY")]
    /// The strike currency of the contract underlying index.
    pub strike_currency: String,
    #[serde(rename = "INDEX_UNDERLYING_ID")]
    /// Represents the internal CCData ID for the index underlying asset (e.g., 1). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub index_underlying_id: i32,
    #[serde(rename = "QUOTE_CURRENCY_ID")]
    /// Represents the internal CCData ID for the asset quote / counter symbol / coin (e.g. 5). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub quote_currency_id: i32,
    #[serde(rename = "SETTLEMENT_CURRENCY_ID")]
    /// Represents the internal CCData ID for the currency that the contract is settled in (e.g. 5). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub settlement_currency_id: i32,
    #[serde(rename = "CONTRACT_CURRENCY_ID")]
    /// Represents the internal CCData ID for the currency that the contract size is denominated in (e.g. 5). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub contract_currency_id: i32,
    #[serde(rename = "STRIKE_CURRENCY_ID")]
    /// Represents the internal CCData ID for the strike currency of the contract underlying index. This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub strike_currency_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    /// The transform function. This is the function we apply when we do mapping to change values into easier human readable
    /// ones and to make sure the mapped direction BASE - QUOTE is constant accross all instruments.
    pub transform_function: String,
    #[serde(rename = "OPEN")]
    /// The open price for the historical period, based on the closest trade before the period start.
    pub open: f64,
    #[serde(rename = "HIGH")]
    /// The highest trade price of the historical period. If there were no trades in the period, the open price will be taken as the highest.
    pub high: f64,
    #[serde(rename = "LOW")]
    /// The lowest trade price of the historical period. If there were no trades in the period, the open price will be taken as the lowest.
    pub low: f64,
    #[serde(rename = "CLOSE")]
    /// The price of the last trade of the historical period. If there were no trades in the period, the open price will be taken as the close.
    pub close: f64,
    #[serde(rename = "NUMBER_OF_CONTRACTS")]
    /// The sum of all the trade number of contracts for the time period. If there were no trades in the time period, 0 will be given.
    pub number_of_contracts: i64,
    #[serde(rename = "TOTAL_TRADES")]
    /// The total number of trades that occurred in the time period. If there were no trades in the time period, 0 will be given.
    pub total_trades: i64,
    #[serde(rename = "TOTAL_TRADES_BUY")]
    /// The total number of BUY trades that occurred in the in time period.
    pub total_trades_buy: i64,
    #[serde(rename = "TOTAL_TRADES_SELL")]
    /// The total number of SELL trades that occurred in the time period.
    pub total_trades_sell: i64,
    #[serde(rename = "TOTAL_TRADES_UNKNOWN")]
    /// The total number of UNKNOWN trades that occurred in the time period.
    pub total_trades_unknown: i64,
    #[serde(rename = "VOLUME")]
    /// The sum of all the trade volumes in the from asset (base symbol / coin) for the time period. If there were no trades in the time period, 0 will be given.
    pub volume: f64,
    #[serde(rename = "QUOTE_VOLUME")]
    /// The sum of all the trade volumes in the To asset (quote/counter symbol/coin) for the time period. If there were no trades in the time period, 0 will be given.
    pub quote_volume: f64,
    #[serde(rename = "VOLUME_BUY")]
    /// The sum of all the BUY trade volumes in the from asset (base symbol / coin) for the time period.
    pub volume_buy: f64,
    #[serde(rename = "QUOTE_VOLUME_BUY")]
    /// The sum of all the BUY trade volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub quote_volume_buy: f64,
    #[serde(rename = "VOLUME_SELL")]
    /// The sum of all the SELL trade volumes in the from asset (base symbol / coin) for the time period.
    pub volume_sell: f64,
    #[serde(rename = "QUOTE_VOLUME_SELL")]
    /// The sum of all the SELL trade volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub quote_volume_sell: f64,
    #[serde(rename = "VOLUME_UNKNOWN")]
    /// The sum of all the UNKNOWN trade volumes in the from asset (base symbol / coin) for the time period.
    pub volume_unknown: f64,
    #[serde(rename = "QUOTE_VOLUME_UNKNOWN")]
    /// The sum of all the UNKNOWN trade volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub quote_volume_unknown: f64,
    #[serde(rename = "NOTIONAL_VOLUME")]
    /// The sum of all the notional trade volumes in the from asset (base symbol / coin) for the time period.
    /// If there were no trades in the time period, 0 will be given.
    pub notional_volume: f64,
    #[serde(rename = "NOTIONAL_QUOTE_VOLUME")]
    /// The sum of all the notional trade volumes in the To asset (quote/counter symbol/coin) for the time period.
    /// If there were no trades in the time period, 0 will be given.
    pub notional_quote_volume: f64,
    #[serde(rename = "NOTIONAL_VOLUME_BUY")]
    /// The sum of all the BUY notional trade volumes in the from asset (base symbol / coin) for the time period.
    pub notional_volume_buy: f64,
    #[serde(rename = "NOTIONAL_QUOTE_VOLUME_BUY")]
    /// The sum of all the BUY notional trade volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub notional_quote_volume_buy: f64,
    #[serde(rename = "NOTIONAL_VOLUME_SELL")]
    /// The sum of all the SELL notional trade volumes in the from asset (base symbol / coin) for the time period.
    pub notional_volume_sell: f64,
    #[serde(rename = "NOTIONAL_QUOTE_VOLUME_SELL")]
    /// The sum of all the SELL notional trade volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub notional_quote_volume_sell: f64,
    #[serde(rename = "NOTIONAL_VOLUME_UNKNOWN")]
    /// The sum of all the UNKNOWN notional trade volumes in the from asset (base symbol / coin) for the time period.
    pub notional_volume_unknown: f64,
    #[serde(rename = "NOTIONAL_QUOTE_VOLUME_UNKNOWN")]
    /// The sum of all the UNKNOWN notional trade volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub notional_quote_volume_unknown: f64,
}


// Options: Markets


/// Options: Markets
#[derive(Deserialize, Debug)]
pub struct CCOptionsMarkets {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    /// The status of the echange. We only poll / stream / connect to the ACTIVE ones, for the RETIRED ones we no longer query for data.
    pub exchange_status: String,
    #[serde(rename = "MAPPED_INSTRUMENTS_TOTAL")]
    /// The total number of instruments that have been verified by our mapping team and have been properly assigned with a base, quote,
    /// mapping function, and other necessary fields. This is done to ensure that pairs like XXBTZUSD are accurately mapped to BTC-USD and
    /// that the pair refers to the correct assets rather than using the same asset id to represent different assets.
    pub mapped_instrument_total: i64,
    #[serde(rename = "UNMAPPED_INSTRUMENTS_TOTAL")]
    /// The number of instruments that have not yet been verified by our mapping team.
    pub unmapped_instruments_total: i64,
    #[serde(rename = "INSTRUMENT_STATUS")]
    /// An object with the total number of instrument for each of the available instrument statuses.
    pub instrument_status: CCInstrumentStatus,
    #[serde(rename = "TOTAL_TRADES_OPTIONS")]
    /// The total number of options trades that this exchange has processed.
    pub total_trades_options: i64,
    #[serde(rename = "TOTAL_OPEN_INTEREST_OPTIONS_UPDATES")]
    /// The total number of options open interest updates that this exchange has processed.
    pub total_open_interest_updates: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}