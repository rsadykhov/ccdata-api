use std::fmt::Display;
use serde::{Serialize, Deserialize};
use crate::schemas::data_api::InstrumentStatus;


#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
/// The exchange to obtain data from.
pub enum FuturesMarket {
    BINANCE,
    BIT,
    BITFINEX,
    BITGET,
    BITMEX,
    BTCEX,
    BULLISH,
    BYBIT,
    COINBASE,
    COINBASEINTERNATIONAL,
    CROSSTOWER,
    CRYPTODOTCOM,
    DERIBIT,
    DYDXV4,
    FTX,
    GATEIO,
    HUOBIPRO,
    HYPERLIQUID,
    #[default]
    KRAKEN,
    KUCOIN,
    MOCK,
    OKEX,
}

impl Display for FuturesMarket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BINANCE => write!(f, "binance"),
            Self::BIT => write!(f, "bit"),
            Self::BITFINEX => write!(f, "bitfinex"),
            Self::BITGET => write!(f, "bitget"),
            Self::BITMEX => write!(f, "bitmex"),
            Self::BTCEX => write!(f, "btcex"),
            Self::BULLISH => write!(f, "bullish"),
            Self::BYBIT => write!(f, "bybit"),
            Self::COINBASE => write!(f, "coinbase"),
            Self::COINBASEINTERNATIONAL => write!(f, "coinbaseinternational"),
            Self::CROSSTOWER => write!(f, "crosstower"),
            Self::CRYPTODOTCOM => write!(f, "cryptodotcom"),
            Self::DERIBIT => write!(f, "deribit"),
            Self::DYDXV4 => write!(f, "dydxv4"),
            Self::FTX => write!(f, "ftx"),
            Self::GATEIO => write!(f, "gateio"),
            Self::HUOBIPRO => write!(f, "huobipro"),
            Self::HYPERLIQUID => write!(f, "hyperliquid"),
            Self::KRAKEN => write!(f, "kraken"),
            Self::KUCOIN => write!(f, "kucoin"),
            Self::MOCK => write!(f, "mock"),
            Self::OKEX => write!(f, "okex"),
        }   
    }
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FuturesInstrumentMapping {
    #[serde(rename = "MAPPED_INSTRUMENT")]
    pub mapped_instrument: String,
    #[serde(rename = "INDEX_UNDERLYING")]
    pub index_underlying: String,
    #[serde(rename = "INDEX_UNDERLYING_ID")]
    pub index_underlying_id: i32,
    #[serde(rename = "QUOTE_CURRENCY")]
    pub quote_currency: String,
    #[serde(rename = "QUOTE_CURRENCY_ID")]
    pub quote_currency_id: i32,
    #[serde(rename = "SETTLEMENT_CURRENCY")]
    pub settlement_currency: String,
    #[serde(rename = "SETTLEMENT_CURRENCY_ID")]
    pub settlement_currency_id: i32,
    #[serde(rename = "CONTRACT_CURRENCY")]
    pub contract_currency: String,
    #[serde(rename = "CONTRACT_CURRENCY_ID")]
    pub contract_currency_id: i32,
    #[serde(rename = "DENOMINATION_TYPE")]
    pub denomination_type: String,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    pub transform_function: String,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
}


// Futures: Historical OHLCV+


/// Futures: Historical OHLCV+
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FuturesOHLCV {
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
    #[serde(rename = "DENOMINATION_TYPE")]
    /// VANILLA = (SETTLEMENT_CURRENCY = QUOTE_CURRENCY), INVERSE = (SETTLEMENT_CURRENCY = INDEX_UNDERLYING),
    /// QUANTO (SETTLEMENT_CURRENCY != INDEX_UNDERLYING or QUOTE_CURRENCY).
    pub denomination_type: String,
    #[serde(rename = "INDEX_UNDERLYING_ID")]
    /// Represents the internal CoinDesk ID for the index underlying asset (e.g., 1). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub index_underlying_id: i32,
    #[serde(rename = "QUOTE_CURRENCY_ID")]
    /// Represents the internal CoinDesk ID for the asset quote / counter symbol / coin (e.g. 5). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub quote_currency_id: i32,
    #[serde(rename = "SETTLEMENT_CURRENCY_ID")]
    /// Represents the internal CoinDesk ID for the currency that the contract is settled in (e.g. 5). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub settlement_currency_id: i32,
    #[serde(rename = "CONTRACT_CURRENCY_ID")]
    /// Represents the internal CoinDesk ID for the currency that the contract size is denominated in (e.g. 5). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub contract_currency_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    /// The transform function. This is the function we apply when we do mapping to change values into easier human readable ones
    /// and to make sure the mapped direction BASE - QUOTE is constant accross all instruments.
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
    #[serde(rename = "FIRST_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the first trade of the time period. Only available when there is at least one trade in the time period.
    pub first_trade_timestamp: i64,
    #[serde(rename = "LAST_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the last trade of the time period. Only available when there is at least one trade in the time period).
    pub last_trade_timestamp: i64,
    #[serde(rename = "FIRST_TRADE_PRICE")]
    /// The price of the first trade of the time period. Only available when there is at least one trade in the time period.
    pub first_trade_price: f64,
    #[serde(rename = "HIGH_TRADE_PRICE")]
    /// The highest trade price of the time period. Only available when there is at least one trade in the time period.
    pub high_trade_price: f64,
    #[serde(rename = "HIGH_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the highest trade in this time period. Only available when there is at least one trade in the time period.
    pub high_trade_timestamp: i64,
    #[serde(rename = "LOW_TRADE_PRICE")]
    /// The lowest trade price of the time period. Only available when there is at least one trade in the time period.
    pub low_trade_price: f64,
    #[serde(rename = "LOW_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the lowest trade of the time period. Only available when there is at least one trade in the time period.
    pub low_trade_timestamp: i64,
    #[serde(rename = "LAST_TRADE_PRICE")]
    /// The price of the last trade of the period. Only available when there is at least one trade in the time period.
    pub last_trade_price: f64,
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
    #[serde(rename = "NUMBER_OF_CONTRACTS")]
    /// The sum of all the trade number of contracts for the time period. If there were no trades in the time period, 0 will be given.
    pub number_of_contracts: i64,
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
}


// Futures: Instrument Metadata


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FuturesInstrumentMetadata {
    #[serde(rename = "METADATA_VERSION")]
    /// The version of metadata, used for version conversions/migrates.
    pub metadata_version: i32,
    #[serde(rename = "INSTRUMENT_STATUS")]
    /// The status of the instrument, we only poll / stream / connect to the ACTIVE ones,
    /// for the RETIRED / IGNORED / EXPIRED / READY_FOR_DECOMMISSIONING means we no longer query/stream data.
    pub instrument_status: String,
    #[serde(rename = "FIRST_SEEN_ON_POLLING_TS")]
    /// This is the first time instrument was seen on instrumentListSourceType POLLING.
    pub first_seen_on_polling_ts: i64,
    #[serde(rename = "LAST_SEEN_ON_POLLING_TS")]
    /// This is the last time instrument was seen on instrumentListSourceType POLLING.
    pub last_seen_on_polling_ts: i64,
    #[serde(rename = "INSTRUMENT")]
    /// The instrument ID as it is on the exchange with small modifications - we do not allow the following characters inside isntrument ids: ,/&?
    pub instrument: String,
    #[serde(rename = "INSTRUMENT_MAPPING")]
    /// The current mapping information for this instrument.
    pub instrument_mapping: FuturesInstrumentMapping,
    #[serde(rename = "INSTRUMENT_EXTERNAL_DATA")]
    /// The full data we get from the polling endpoint for this specific instrument. 
    /// his is a JSON stringified object with different properties per exchange.
    pub instrument_external_data: String,
    #[serde(rename = "INSTRUMENT_AVAILABLE_ON_INSTRUMENTS_ENDPOINT")]
    /// This flags the exchange instrument is currently available on instruments endpoint.
    pub instrument_available_on_instruments_endpoint: bool,
    #[serde(rename = "INDEX_ID")]
    /// The id of the index the contract is based on.
    pub index_id: String,
    #[serde(rename = "INDEX_UNDERLYING")]
    /// The underlying instrument of the index.
    pub index_underlying: String,
    #[serde(rename = "QUOTE_CURRENCY")]
    /// The instrument that the contract is priced in.
    pub quote_currency: String,
    #[serde(rename = "SETTLEMENT_CURRENCY")]
    /// The currency used to calculate contract PnL. The settlement currency can be different from the index underlying or quote currencies.
    pub settlement_currency: String,
    #[serde(rename = "DENOMINATION_TYPE")]
    /// VANILLA = (SETTLEMENT_CURRENCY = QUOTE_CURRENCY), INVERSE = (SETTLEMENT_CURRENCY = INDEX_UNDERLYING),
    /// QUANTO (SETTLEMENT_CURRENCY != INDEX_UNDERLYING or QUOTE_CURRENCY)
    pub denomination_type: String,
    #[serde(rename = "CONTRACT_CURRENCY")]
    /// The denomination of the CONTRACT_SIZE.
    pub contract_currency: String,
    #[serde(rename = "CONTRACT_SIZE")]
    /// The contract size - how much of the contract currency does one contract contain.
    pub contract_size: f64,
    #[serde(rename = "TICK_SIZE")]
    /// The minimum amount the price can move, denominated in QUOTE_CURRENCY.
    pub tick_size: f64,
    #[serde(rename = "DELIVERY_METHOD")]
    /// The settlement delivery method on the derivative product.
    pub delivery_method: String,
    #[serde(rename = "CONTRACT_TERM")]
    /// The term / duration the contract represents e.g. 3xMONTH.
    pub contract_term: String,
    #[serde(rename = "CONTRACT_CREATION_TS")]
    /// The contract creation timestamp we get for the specific derivative instrument.
    pub contract_creation_ts: i64,
    #[serde(rename = "CONTRACT_EXPIRATION_TS")]
    /// The contract expiration timestamp we get for the specific derivative instrument. Not needed for PERPETUAL contract types.
    pub contract_expiration_ts: Option<i64>,
    #[serde(rename = "FIRST_OB_L2_MINUTE_SNAPSHOT_TS")]
    /// Timestamp of the initial Level 2 minute snapshot.
    pub first_ob_l2_minute_snapshot_t2s: i64,
    #[serde(rename = "LAST_OB_L2_MINUTE_SNAPSHOT_TS")]
    /// Timestamp of the latest Level 2 minute snapshot.
    pub last_ob_l2_minute_snapshot_ts: i64,
}


// Futures: Markets


/// Futures: Markets
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FuturesMarkets {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    /// The status of the echange. We only poll / stream / connect to the ACTIVE ones, for the RETIRED ones we no longer query for data.
    pub exchange_status: String,
    #[serde(rename = "MAPPED_INSTRUMENTS_TOTAL")]
    /// The total number of instruments that have been verified by our mapping team and have been properly assigned with a base, quote, mapping function,
    /// and other necessary fields. This is done to ensure that pairs like XXBTZUSD are accurately mapped to BTC-USD and that the pair refers
    // to the correct assets rather than using the same asset id to represent different assets.
    pub mapped_instrument_total: i64,
    #[serde(rename = "UNMAPPED_INSTRUMENTS_TOTAL")]
    /// The number of instruments that have not yet been verified by our mapping team.
    pub unmapped_instruments_total: i64,
    #[serde(rename = "INSTRUMENT_STATUS")]
    /// An object with the total number of instrument for each of the available instrument statuses.
    pub instrument_status: InstrumentStatus,
    #[serde(rename = "TOTAL_TRADES_FUTURES")]
    /// The total number of futures trades that this exchange has processed.
    pub total_trades_futures: i64,
    #[serde(rename = "TOTAL_FUNDING_RATE_UPDATES")]
    /// The total number of futures funding rate updates that this exchange has processed.
    pub total_funding_rate_updates: i64,
    #[serde(rename = "TOTAL_OPEN_INTEREST_UPDATES")]
    /// The total number of futures open interest updates that this exchange has processed.
    pub total_open_interest_updates: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    /// Boolean field denoting if we have historical minute orderbook snapshots endabled for this exchange.
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}