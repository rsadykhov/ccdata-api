use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


pub enum CCFuturesMarket {
    BINANCE,
    BITFINEX,
    BITGET,
    BITMEX,
    BTCEX,
    BYBIT,
    COINBASE,
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

impl Market for CCFuturesMarket {
    fn to_string(&self) -> String {
        match self {
            CCFuturesMarket::BINANCE => String::from("binance"),
            CCFuturesMarket::BITFINEX => String::from("bitfinex"),
            CCFuturesMarket::BITGET => String::from("bitget"),
            CCFuturesMarket::BITMEX => String::from("bitmex"),
            CCFuturesMarket::BTCEX => String::from("btcex"),
            CCFuturesMarket::BYBIT => String::from("bybit"),
            CCFuturesMarket::COINBASE => String::from("coinbase"),
            CCFuturesMarket::COINBASEINTERNATIONAL => String::from("coinbaseinternational"),
            CCFuturesMarket::CROSSTOWER => String::from("crosstower"),
            CCFuturesMarket::CRYPTODOTCOM => String::from("cryptodotcom"),
            CCFuturesMarket::DERIBIT => String::from("deribit"),
            CCFuturesMarket::DYDXV4 => String::from("dydxv4"),
            CCFuturesMarket::FTX => String::from("ftx"),
            CCFuturesMarket::KRAKEN => String::from("kraken"),
            CCFuturesMarket::MOCK => String::from("mock"),
            CCFuturesMarket::OKEX => String::from("okex"),

        }
    }
}


// Futures: Historical OHLCV+


/// Futures: Historical OHLCV+
#[derive(Deserialize, Debug)]
pub struct CCFuturesOHLCV {
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
    #[serde(rename = "MAPPED_INSTRUMENT")]
    pub mapped_instrument: String,
    #[serde(rename = "INDEX_UNDERLYING")]
    pub index_underlying: String,
    #[serde(rename = "QUOTE_CURRENCY")]
    pub quote_currency: String,
    #[serde(rename = "SETTLEMENT_CURRENCY")]
    pub settlement_currency: String,
    #[serde(rename = "CONTRACT_CURRENCY")]
    pub contract_currency: String,
    #[serde(rename = "DENOMINATION_TYPE")]
    pub denomination_type: String,
    #[serde(rename = "INDEX_UNDERLYING_ID")]
    pub index_underlying_id: i32,
    #[serde(rename = "QUOTE_CURRENCY_ID")]
    pub quote_currency_id: i32,
    #[serde(rename = "SETTLEMENT_CURRENCY_ID")]
    pub settlement_currency_id: i32,
    #[serde(rename = "CONTRACT_CURRENCY_ID")]
    pub contract_currency_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    pub transform_function: String,
    #[serde(rename = "OPEN")]
    pub open: f64,
    #[serde(rename = "HIGH")]
    pub high: f64,
    #[serde(rename = "LOW")]
    pub low: f64,
    #[serde(rename = "CLOSE")]
    pub close: f64,
    #[serde(rename = "FIRST_TRADE_TIMESTAMP")]
    pub first_trade_timestamp: i64,
    #[serde(rename = "LAST_TRADE_TIMESTAMP")]
    pub last_trade_timestamp: i64,
    #[serde(rename = "FIRST_TRADE_PRICE")]
    pub first_trade_price: f64,
    #[serde(rename = "HIGH_TRADE_PRICE")]
    pub high_trade_price: f64,
    #[serde(rename = "HIGH_TRADE_TIMESTAMP")]
    pub high_trade_timestamp: i64,
    #[serde(rename = "LOW_TRADE_PRICE")]
    pub low_trade_price: f64,
    #[serde(rename = "LOW_TRADE_TIMESTAMP")]
    pub low_trade_timestamp: i64,
    #[serde(rename = "LAST_TRADE_PRICE")]
    pub last_trade_price: f64,
    #[serde(rename = "TOTAL_TRADES")]
    pub total_trades: i64,
    #[serde(rename = "TOTAL_TRADES_BUY")]
    pub total_trades_buy: i64,
    #[serde(rename = "TOTAL_TRADES_SELL")]
    pub total_trades_sell: i64,
    #[serde(rename = "TOTAL_TRADES_UNKNOWN")]
    pub total_trades_unknown: i64,
    #[serde(rename = "NUMBER_OF_CONTRACTS")]
    pub number_of_contracts: i64,
    #[serde(rename = "VOLUME")]
    pub volume: f64,
    #[serde(rename = "QUOTE_VOLUME")]
    pub quote_volume: f64,
    #[serde(rename = "VOLUME_BUY")]
    pub volume_buy: f64,
    #[serde(rename = "QUOTE_VOLUME_BUY")]
    pub quote_volume_buy: f64,
    #[serde(rename = "VOLUME_SELL")]
    pub volume_sell: f64,
    #[serde(rename = "QUOTE_VOLUME_SELL")]
    pub quote_volume_sell: f64,
    #[serde(rename = "VOLUME_UNKNOWN")]
    pub volume_unknown: f64,
    #[serde(rename = "QUOTE_VOLUME_UNKNOWN")]
    pub quote_volume_unknown: f64,
}


// Futures: Markets


/// Futures: Markets
#[derive(Deserialize, Debug)]
pub struct CCFuturesMarkets {
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
    #[serde(rename = "TOTAL_TRADES_FUTURES")]
    pub total_trades_futures: i64,
    #[serde(rename = "TOTAL_FUNDING_RATE_UPDATES")]
    pub total_funding_rate_updates: i64,
    #[serde(rename = "TOTAL_OPEN_INTEREST_UPDATES")]
    pub total_open_interest_updates: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}