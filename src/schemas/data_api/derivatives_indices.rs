use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


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
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "MARKET")]
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    pub instrument: String,
    #[serde(rename = "OPEN")]
    pub open: f64,
    #[serde(rename = "HIGH")]
    pub high: f64,
    #[serde(rename = "LOW")]
    pub low: f64,
    #[serde(rename = "CLOSE")]
    pub close: f64,
    #[serde(rename = "FIRST_MESSAGE_TIMESTAMP")]
    pub first_message_timestamp: i64,
    #[serde(rename = "LAST_MESSAGE_TIMESTAMP")]
    pub last_message_timestamp: i64,
    #[serde(rename = "FIRST_MESSAGE_VALUE")]
    pub first_message_value: f64,
    #[serde(rename = "HIGH_MESSAGE_VALUE")]
    pub high_message_value: f64,
    #[serde(rename = "HIGH_MESSAGE_TIMESTAMP")]
    pub high_message_timestamp: i64,
    #[serde(rename = "LOW_MESSAGE_VALUE")]
    pub low_message_value: f64,
    #[serde(rename = "LOW_MESSAGE_TIMESTAMP")]
    pub low_message_timestamp: i64,
    #[serde(rename = "LAST_MESSAGE_VALUE")]
    pub last_message_value: f64,
    #[serde(rename = "TOTAL_INDEX_UPDATES")]
    pub total_index_updates: i32,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    pub mapped_instrument: String,
    #[serde(rename = "CURRENCY")]
    pub currency: String,
    #[serde(rename = "CURRENCY_ID")]
    pub currency_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    pub transform_function: String,
}


// Derivatives Indices: Markets


/// Derivatives Indices: Markets
#[derive(Deserialize, Debug)]
pub struct CCDerIndicesMarkets {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    pub exchange_status: String,
    #[serde(rename = "MAPPED_INSTRUMENTS_TOTAL")]
    pub mapped_instrument_total: i64,
    #[serde(rename = "UNMAPPED_INSTRUMENTS_TOTAL")]
    pub unmapped_instruments_total: i64,
    #[serde(rename = "INSTRUMENT_STATUS")]
    pub instrument_status: CCInstrumentStatus,
    #[serde(rename = "TOTAL_INDEX_UPDATES")]
    pub total_index_updates: i64,
}