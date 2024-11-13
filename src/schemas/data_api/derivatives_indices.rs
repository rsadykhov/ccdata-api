use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


/// The exchange to obtain data from.
pub enum CCDerIndicesMarket {
    BINANCE,
    BITGET,
    BITMEX,
    BTCEX,
    BYBIT,
    COINBASEINTERNATIONAL,
    CROSSTOWER,
    CRYPTODOTCOM,
    DERIBIT,
    DYDXV4,
    FTX,
    KRAKEN,
    MOCK,
    OKEX,
}

impl Market for CCDerIndicesMarket {
    /// Converts enum value to `String`.
    fn to_string(&self) -> String {
        match self {
            CCDerIndicesMarket::BINANCE => String::from("binance"),
            CCDerIndicesMarket::BITGET => String::from("bitget"),
            CCDerIndicesMarket::BITMEX => String::from("bitmex"),
            CCDerIndicesMarket::BTCEX => String::from("btcex"),
            CCDerIndicesMarket::BYBIT => String::from("bybit"),
            CCDerIndicesMarket::COINBASEINTERNATIONAL => String::from("coinbaseinternational"),
            CCDerIndicesMarket::CROSSTOWER => String::from("crosstower"),
            CCDerIndicesMarket::CRYPTODOTCOM => String::from("cryptodotcom"),
            CCDerIndicesMarket::DERIBIT => String::from("deribit"),
            CCDerIndicesMarket::DYDXV4 => String::from("dydxv4"),
            CCDerIndicesMarket::FTX => String::from("ftx"),
            CCDerIndicesMarket::KRAKEN => String::from("kraken"),
            CCDerIndicesMarket::MOCK => String::from("mock"),
            CCDerIndicesMarket::OKEX => String::from("okex"),

        }
    }
}


// Derivatives Indices: Historical OHLCV+


/// Derivatives Indices: Historical OHLCV+
#[derive(Deserialize, Debug)]
pub struct CCDerIndicesOHLCV {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: MINUTE for minute, HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the histo period, for minute it would be every minute at the beginning of the minute,
    /// for hour it would be start of the hour and for daily it is 00:00 GMT/UTC.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "MARKET")]
    /// The market / exchange you have requested (name of the market / exchange e.g. bitmex, deribit, ftx, etc.).
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    /// The specific financial asset pair that an index is tracking in unmapped format. In most cases this is a combiation of the base and quote assets of the pair.
    pub instrument: String,
    #[serde(rename = "OPEN")]
    /// The unmapped instrument ID.
    pub open: f64,
    #[serde(rename = "HIGH")]
    /// The open value for the historical period, this is based on the closest index update before the period start.
    pub high: f64,
    #[serde(rename = "LOW")]
    /// The max between the open and the highest index update value in this time period (same as open when there no messages in the time period).
    pub low: f64,
    #[serde(rename = "CLOSE")]
    /// The min between the open and the lowest index update value in this time period (same as open when there no messages in the time period).
    pub close: f64,
    #[serde(rename = "FIRST_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the first index update in this time period (only available when we have at least one index update in the time period).
    pub first_message_timestamp: i64,
    #[serde(rename = "LAST_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the last index update in this time period (only available when we have at least one index update in the time period).
    pub last_message_timestamp: i64,
    #[serde(rename = "FIRST_MESSAGE_VALUE")]
    /// The open based on the first index update in the time period (only available when we have at least one index update in the time period).
    pub first_message_value: f64,
    #[serde(rename = "HIGH_MESSAGE_VALUE")]
    /// The highest value of the messages in the time period (only available when we have at least one index update in the time period).
    pub high_message_value: f64,
    #[serde(rename = "HIGH_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the highest index update in this time period (only available when we have at least one index update in the time period).
    pub high_message_timestamp: i64,
    #[serde(rename = "LOW_MESSAGE_VALUE")]
    /// The lowest value of the messages in the time period (only available when we have at least one index update in the time period).
    pub low_message_value: f64,
    #[serde(rename = "LOW_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the lowest index update in this time period (only available when we have at least one index update in the time period).
    pub low_message_timestamp: i64,
    #[serde(rename = "LAST_MESSAGE_VALUE")]
    /// The last index update value in the time period (only available when we have at least one index update in the time period).
    pub last_message_value: f64,
    #[serde(rename = "TOTAL_INDEX_UPDATES")]
    /// The total number of message updates seen in this time period (0 when there no messages in the time period).
    pub total_index_updates: i32,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    /// The instrument ID, as derived from our mapping rules. Only available on instruments that have been mapped.
    pub mapped_instrument: String,
    #[serde(rename = "CURRENCY")]
    /// The mapped index currency. Only available on instruments that have mapping.
    pub currency: String,
    #[serde(rename = "CURRENCY_ID")]
    /// Represents the internal CCData ID for the mapped index currency, e.g. 1. This ID is unique and immutable, ensuring consistent identification.
    // Applicable only to instruments with a mapping.
    pub currency_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    /// The transform function. This is the function we apply when we do mapping to change values into easier human readable ones and to make
    /// sure the mapped direction BASE - QUOTE is constant accross all instruments.
    pub transform_function: String,
}


// Derivatives Indices: Markets


/// Derivatives Indices: Markets
#[derive(Deserialize, Debug)]
pub struct CCDerIndicesMarkets {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    /// The status of the echange. We only poll / stream / connect to the ACTIVE ones, for the RETIRED ones we no longer query for data.
    pub exchange_status: String,
    #[serde(rename = "MAPPED_INSTRUMENTS_TOTAL")]
    /// The total number of instruments that have been verified by our mapping team and have been properly assigned with a base, quote,
    /// mapping function, and other necessary fields. This is done to ensure that pairs like XXBTZUSD are accurately mapped to BTC-USD and that
    /// the pair refers to the correct assets rather than using the same asset id to represent different assets.
    pub mapped_instrument_total: i64,
    #[serde(rename = "UNMAPPED_INSTRUMENTS_TOTAL")]
    /// The number of instruments that have not yet been verified by our mapping team.
    pub unmapped_instruments_total: i64,
    #[serde(rename = "INSTRUMENT_STATUS")]
    /// An object with the total number of instrument for each of the available instrument statuses.
    pub instrument_status: CCInstrumentStatus,
}