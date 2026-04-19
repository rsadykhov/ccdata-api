use std::{fmt::Display, collections::HashMap};
use serde::{Serialize, Deserialize};
use crate::schemas::data_api::InstrumentStatus;


#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
/// The exchange to obtain data from.
pub enum SpotMarket {
    AAX,
    ABCC,
    ACX,
    AIDOSMARKET,
    ALPHAEX,
    ARCHAX,
    ASCENDEX,
    ATAIX,
    BACKPACK,
    BEQUANT,
    BGOGO,
    BIBOX365,
    BIGONE,
    BILAXY,
    BINANCE,
    BINANCEAGGREGATE,
    BINANCETR,
    BINANCEUSA,
    BINGX,
    BISQ,
    BIT,
    BIT2C,
    BITBANK,
    BITBAY,
    BITBNS,
    BITBUY,
    BITCI,
    BITEXBOOK,
    BITFEX,
    BITFINEX,
    BITFLYER,
    BITFLYEREU,
    BITFLYERFX,
    BITFLYERUS,
    BITFOREX,
    BITGET,
    BITHUMBGLOBAL,
    BITHUMBKOREA,
    BITINKA,
    BITKUB,
    BITMART,
    BITMEX,
    BITPANDA,
    BITRUE,
    BITSO,
    BITSTAMP,
    BITTREX,
    BITVAVO,
    BKEX,
    BLACKTURTLE,
    BLEUTRADE,
    BLOCKCHAINCOM,
    BTCALPHA,
    BTCBOX,
    BTCEX,
    BTCMARKETS,
    BTCTURK,
    BTSE,
    BUDA,
    BULLISH,
    BUYUCOIN,
    BWEXCHANGE,
    BYBIT,
    BYDFI,
    CATEX,
    CEXIO,
    COINBASE,
    COINBASEINTERNATIONAL,
    COINCHECK,
    COINCORNER,
    COINDCX,
    COINDEAL,
    COINEX,
    COINFALCON,
    COINFIELD,
    COINJAR,
    COINMATE,
    COINONE,
    COINSBIT,
    COINSPRO,
    COINTIGER,
    COINW,
    COSS,
    CREX24,
    CROSSTOWER,
    CRYPTOCARBON,
    CRYPTODOTCOM,
    CRYPTOPIA,
    CRYPTSY,
    CUBE,
    CURRENCY,
    DCOIN,
    DDEX,
    DECOIN,
    DERIBIT,
    DIGIFINEX,
    ERISX,
    ETORO,
    EXMO,
    FCOIN,
    FOXBIT,
    FTX,
    FTXUS,
    GARANTEX,
    GATEIO,
    GEMINI,
    GLOBITEX,
    GOPAX,
    GRAVIEX,
    HASHKEY,
    HITBTC,
    HUOBIJAPAN,
    HUOBIPRO,
    INDEPENDENTRESERVE,
    INDODAX,
    INDOEX,
    INX,
    ITBIT,
    KORBIT,
    #[default]
    KRAKEN,
    KUCOIN,
    KUNA,
    LATOKEN,
    LBANK,
    LIQNET,
    LIQUID,
    LITEBIT,
    LMAX,
    LUNO,
    LYKKE,
    MERCADOBTC,
    MERCATOX,
    MEXC,
    MOCK,
    MTGOX,
    NDAX,
    NOMINEX,
    OKCOIN,
    OKEX,
    ONETRADING,
    OSL,
    OSLHONGKONG,
    P2PB2B,
    PANCAKESWAP,
    PARAMOUNTDAX,
    PARIBU,
    PHEMEX,
    POLONIEX,
    PROBIT,
    SAFETRADE,
    SIGENPRO,
    SIMEX,
    SWITCHEO,
    THEROCKTRADING,
    TIDEFI,
    TIMEX,
    TOKENOMY,
    TRADEOGRE,
    UNISWAP,
    UNOCOIN,
    UPBIT,
    VALR,
    VITEX,
    WAZIRX,
    WHITEBIT,
    WOO,
    XCOEX,
    XTPUB,
    YELLOW,
    YOBIT,
    ZAIF,
    ZBDOTCOM,
    ZBG,
    ZEBITEX,
    ZONDA,
}

impl Display for SpotMarket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AAX => write!(f, "aax"),
            Self::ABCC => write!(f, "abcc"),
            Self::ACX => write!(f, "acx"),
            Self::AIDOSMARKET => write!(f, "aidosmarket"),
            Self::ALPHAEX => write!(f, "alphaex"),
            Self::ARCHAX => write!(f, "archax"),
            Self::ASCENDEX => write!(f, "ascendex"),
            Self::ATAIX => write!(f, "ataix"),
            Self::BACKPACK => write!(f, "backpack"),
            Self::BEQUANT => write!(f, "bequant"),
            Self::BGOGO => write!(f, "bgogo"),
            Self::BIBOX365 => write!(f, "bibox365"),
            Self::BIGONE => write!(f, "bigone"),
            Self::BILAXY => write!(f, "bilaxy"),
            Self::BINANCE => write!(f, "binance"),
            Self::BINANCEAGGREGATE => write!(f, "binanceaggreagate"),
            Self::BINANCETR => write!(f, "binancetr"),
            Self::BINANCEUSA => write!(f, "binanceusa"),
            Self::BINGX => write!(f, "bingx"),
            Self::BISQ => write!(f, "bisq"),
            Self::BIT => write!(f, "bit"),
            Self::BIT2C => write!(f, "bit2c"),
            Self::BITBANK => write!(f, "bitbank"),
            Self::BITBAY => write!(f, "bitbay"),
            Self::BITBNS => write!(f, "bitbns"),
            Self::BITBUY => write!(f, "bitbuy"),
            Self::BITCI => write!(f, "bitci"),
            Self::BITEXBOOK => write!(f, "bitexbook"),
            Self::BITFEX => write!(f, "bitfex"),
            Self::BITFINEX => write!(f, "bitfinex"),
            Self::BITFLYER => write!(f, "bitflyer"),
            Self::BITFLYEREU => write!(f, "bitflyereu"),
            Self::BITFLYERFX => write!(f, "bitflyerfx"),
            Self::BITFLYERUS => write!(f, "bitflyerus"),
            Self::BITFOREX => write!(f, "bitforex"),
            Self::BITGET => write!(f, "bitget"),
            Self::BITHUMBGLOBAL => write!(f, "bithumbglobal"),
            Self::BITHUMBKOREA => write!(f, "bithumbkorea"),
            Self::BITINKA => write!(f, "bitinka"),
            Self::BITKUB => write!(f, "bitkub"),
            Self::BITMART => write!(f, "bitmart"),
            Self::BITMEX => write!(f, "bitmex"),
            Self::BITPANDA => write!(f, "bitpanda"),
            Self::BITRUE => write!(f, "bitrue"),
            Self::BITSO => write!(f, "bitso"),
            Self::BITSTAMP => write!(f, "bitstamp"),
            Self::BITTREX => write!(f, "bittrex"),
            Self::BITVAVO => write!(f, "bitvavo"),
            Self::BKEX => write!(f, "bkex"),
            Self::BLACKTURTLE => write!(f, "blackturtle"),
            Self::BLEUTRADE => write!(f, "bleutrade"),
            Self::BLOCKCHAINCOM => write!(f, "blockchaindotcom"),
            Self::BTCALPHA => write!(f, "btcalpha"),
            Self::BTCBOX => write!(f, "btcbox"),
            Self::BTCEX => write!(f, "btcex"),
            Self::BTCMARKETS => write!(f, "btcmarkets"),
            Self::BTCTURK => write!(f, "btcturk"),
            Self::BTSE => write!(f, "btse"),
            Self::BUDA => write!(f, "buda"),
            Self::BULLISH => write!(f, "bullish"),
            Self::BUYUCOIN => write!(f, "buyucoin"),
            Self::BWEXCHANGE => write!(f, "bwexchange"),
            Self::BYBIT => write!(f, "bybit"),
            Self::BYDFI => write!(f, "bydfi"),
            Self::CATEX => write!(f, "catex"),
            Self::CEXIO => write!(f, "cexio"),
            Self::COINBASE => write!(f, "coinbase"),
            Self::COINBASEINTERNATIONAL => write!(f, "coinbaseinternational"),
            Self::COINCHECK => write!(f, "coincheck"),
            Self::COINCORNER => write!(f, "coincorner"),
            Self::COINDCX => write!(f, "coindcx"),
            Self::COINDEAL => write!(f, "coindeal"),
            Self::COINEX => write!(f, "coinex"),
            Self::COINFALCON => write!(f, "coinfalcon"),
            Self::COINFIELD => write!(f, "coinfield"),
            Self::COINJAR => write!(f, "coinjar"),
            Self::COINMATE => write!(f, "coinmate"),
            Self::COINONE => write!(f, "coinone"),
            Self::COINSBIT => write!(f, "coinsbit"),
            Self::COINSPRO => write!(f, "coinspro"),
            Self::COINTIGER => write!(f, "cointiger"),
            Self::COINW => write!(f, "coinw"),
            Self::COSS => write!(f, "coss"),
            Self::CREX24 => write!(f, "crex24"),
            Self::CROSSTOWER => write!(f, "crosstower"),
            Self::CRYPTOCARBON => write!(f, "cryptocarbon"),
            Self::CRYPTODOTCOM => write!(f, "cryptodotcom"),
            Self::CRYPTOPIA => write!(f, "cryptopia"),
            Self::CRYPTSY => write!(f, "cryptsy"),
            Self::CUBE => write!(f, "cube"),
            Self::CURRENCY => write!(f, "currency"),
            Self::DCOIN => write!(f, "dcoin"),
            Self::DDEX => write!(f, "ddex"),
            Self::DECOIN => write!(f, "decoin"),
            Self::DERIBIT => write!(f, "deribit"),
            Self::DIGIFINEX => write!(f, "digifinex"),
            Self::ERISX => write!(f, "erisx"),
            Self::ETORO => write!(f, "etoro"),
            Self::EXMO => write!(f, "exmo"),
            Self::FCOIN => write!(f, "fcoin"),
            Self::FOXBIT => write!(f, "foxbit"),
            Self::FTX => write!(f, "ftx"),
            Self::FTXUS => write!(f, "ftxus"),
            Self::GARANTEX => write!(f, "garantex"),
            Self::GATEIO => write!(f, "gateio"),
            Self::GEMINI => write!(f, "gemini"),
            Self::GLOBITEX => write!(f, "globitex"),
            Self::GOPAX => write!(f, "gopax"),
            Self::GRAVIEX => write!(f, "graviex"),
            Self::HASHKEY => write!(f, "hashkey"),
            Self::HITBTC => write!(f, "hitbtc"),
            Self::HUOBIJAPAN => write!(f, "huobijapan"),
            Self::HUOBIPRO => write!(f, "huobipro"),
            Self::INDEPENDENTRESERVE => write!(f, "independentreserve"),
            Self::INDODAX => write!(f, "indodax"),
            Self::INDOEX => write!(f, "indoex"),
            Self::INX => write!(f, "inx"),
            Self::ITBIT => write!(f, "itbit"),
            Self::KORBIT => write!(f, "korbit"),
            Self::KRAKEN => write!(f, "kraken"),
            Self::KUCOIN => write!(f, "kucoin"),
            Self::KUNA => write!(f, "kuna"),
            Self::LATOKEN => write!(f, "latoken"),
            Self::LBANK => write!(f, "lbank"),
            Self::LIQNET => write!(f, "liqnet"),
            Self::LIQUID => write!(f, "liquid"),
            Self::LITEBIT => write!(f, "litebit"),
            Self::LMAX => write!(f, "lmax"),
            Self::LUNO => write!(f, "luno"),
            Self::LYKKE => write!(f, "lykke"),
            Self::MERCADOBTC => write!(f, "mercadobtc"),
            Self::MERCATOX => write!(f, "mercatox"),
            Self::MEXC => write!(f, "mexc"),
            Self::MOCK => write!(f, "mock"),
            Self::MTGOX => write!(f, "mtgox"),
            Self::NDAX => write!(f, "ndax"),
            Self::NOMINEX => write!(f, "nominex"),
            Self::OKCOIN => write!(f, "okcoin"),
            Self::OKEX => write!(f, "okex"),
            Self::ONETRADING => write!(f, "onetrading"),
            Self::OSL => write!(f, "osl"),
            Self::OSLHONGKONG => write!(f, "oslhongkong"),
            Self::P2PB2B => write!(f, "p2pb2b"),
            Self::PANCAKESWAP => write!(f, "pancakeswap"),
            Self::PARAMOUNTDAX => write!(f, "paramountdax"),
            Self::PARIBU => write!(f, "paribu"),
            Self::PHEMEX => write!(f, "phemex"),
            Self::POLONIEX => write!(f, "poloniex"),
            Self::PROBIT => write!(f, "probit"),
            Self::SAFETRADE => write!(f, "safetrade"),
            Self::SIGENPRO => write!(f, "sigenpro"),
            Self::SIMEX => write!(f, "simex"),
            Self::SWITCHEO => write!(f, "switcheo"),
            Self::THEROCKTRADING => write!(f, "therocktrading"),
            Self::TIDEFI => write!(f, "tidefi"),
            Self::TIMEX => write!(f, "timex"),
            Self::TOKENOMY => write!(f, "tokenomy"),
            Self::TRADEOGRE => write!(f, "tradeogre"),
            Self::UNISWAP => write!(f, "uniswap"),
            Self::UNOCOIN => write!(f, "unocoin"),
            Self::UPBIT => write!(f, "upbit"),
            Self::VALR => write!(f, "valr"),
            Self::VITEX => write!(f, "vitex"),
            Self::WAZIRX => write!(f, "waxirx"),
            Self::WHITEBIT => write!(f, "whitebit"),
            Self::WOO => write!(f, "woo"),
            Self::XCOEX => write!(f, "xcoex"),
            Self::XTPUB => write!(f, "xtpub"),
            Self::YELLOW => write!(f, "yellow"),
            Self::YOBIT => write!(f, "yobit"),
            Self::ZAIF => write!(f, "zaif"),
            Self::ZBDOTCOM => write!(f, "zbdotcom"),
            Self::ZBG => write!(f, "zbg"),
            Self::ZEBITEX => write!(f, "zebitex"),
            Self::ZONDA => write!(f, "zonda"),
        }        
    }
}


#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
/// The status of the instrument, can be one of the following: ACTIVE, IGNORED, RETIRED, EXPIRED.
pub enum SpotInstrumentStatus {
    #[default]
    ACTIVE,
    IGNORED,
    RETIRED,
    EXPIRED,
    READYFORDECOMMISSIONING,
}

impl Display for SpotInstrumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ACTIVE => write!(f, "ACTIVE"),
            Self::IGNORED => write!(f, "IGNORED"),
            Self::RETIRED => write!(f, "RETIRED"),
            Self::EXPIRED => write!(f, "EXPIRED"),
            Self::READYFORDECOMMISSIONING => write!(f, "READY_FOR_DECOMMISSIONING"),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotInstrumentMapping {
    #[serde(rename = "MAPPED_INSTRUMENT")]
    /// The current mapping dsv for this instrument.
    pub mapped_instrument: String,
    #[serde(rename = "BASE")]
    /// The current mapping information for this instrument.
    pub base: String,
    #[serde(rename = "BASE_ID")]
    /// Represents the internal CoinDesk ID for the base asset or coin (e.g., 1 for BTC). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub base_id: i32,
    #[serde(rename = "QUOTE")]
    /// The current mapping vs for this instrument.
    pub quote: String,
    #[serde(rename = "QUOTE_ID")]
    /// Represents the internal CoinDesk ID for the quote asset or counter coin (e.g., 5 for USD). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub quote_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    /// The current mapping vsscds for this instrument.
    pub transform_function: String,
    #[serde(rename = "CREATED_ON")]
    /// Timestamp for when this mapping was created.
    pub created_on: i64,
}


// Spot: Historical OHLCV+


/// Spot: Historical OHLCV+
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotOHLCV {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: MINUTE for minute, HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the beginning of the histo period. For minute it would be every minute at the beginning of the minute,
    /// for hour it would be the start of the hour and for daily it is 00:00 GMT.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// The type of message this is. It helps identify the nature of the data being returned.
    pub type_: String,
    #[serde(rename = "MARKET")]
    /// The market / exchange under consideration (e.g. gemini, kraken, coinbase, etc).
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    /// The unmapped instrument ID.
    pub instrument: String,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    /// The instrument ID, as derived from our mapping rules. This takes the form "BASE-QUOTE" (e.g. BTC-USD). Only available on instruments that have been mapped.
    pub mapped_instrument: String,
    #[serde(rename = "BASE")]
    /// Represents the base asset or coin symbol, commonly known as the ticker (e.g., BTC). This symbol may change in cases of asset rebranding.
    /// Applicable only to instruments with a mapping.
    pub base: String,
    #[serde(rename = "QUOTE")]
    /// Represents the quote asset or counter coin symbol, commonly known as the ticker (e.g., USD). This symbol may change in cases of asset rebranding.
    /// Applicable only to instruments with a mapping.
    pub quote: String,
    #[serde(rename = "BASE_ID")]
    /// Represents the internal CoinDesk ID for the base asset or coin (e.g., 1 for BTC). This ID is unique and immutable, ensuring consistent identification.
    /// Applicable only to instruments with a mapping.
    pub base_id: i32,
    #[serde(rename = "QUOTE_ID")]
    /// Represents the internal CoinDesk ID for the quote asset or counter coin (e.g., 5 for USD). This ID is unique and immutable, ensuring consistent identification.
    /// Applicable only to instruments with a mapping.
    pub quote_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    /// The transform function. This is the function we apply when we do mapping to change values into easier human readable ones and to make sure the
    /// mapped direction BASE - QUOTE is constant accross all instruments.
    pub transform_function: String,
    #[serde(rename = "OPEN")]
    /// The open price for the historical period, this is based on the closest trade before the period start.
    pub open: f64,
    #[serde(rename = "HIGH")]
    /// The highest trade price in the time period. If there were no trades in the time period, the open price will be given.
    pub high: f64,
    #[serde(rename = "LOW")]
    /// The lowest trade price in the time period. If there were no trades in the time period, the open price will be given.
    pub low: f64,
    #[serde(rename = "CLOSE")]
    /// The price of the last trade in this time period. If there were no trades in the time period, the open price will be given.
    pub close: f64,
    #[serde(rename = "FIRST_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the first trade in this time perio. This is only available when there is at least one trade in the time period.
    pub first_trade_timestamp: Option<i64>,
    #[serde(rename = "LAST_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the last trade in this time period. This is only available when there is at least one trade in the time period.
    pub last_trade_timestamp: Option<i64>,
    #[serde(rename = "FIRST_TRADE_PRICE")]
    /// The price of the first trade in the time period. This is only available when there is at least one trade in the time period.
    pub first_trade_price: Option<f64>,
    #[serde(rename = "HIGH_TRADE_PRICE")]
    /// The highest value of the trades in the time period. This is only available when there is at least one trade in the time period.
    pub high_trade_price: Option<f64>,
    #[serde(rename = "HIGH_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the highest trade in the time period. This is only available when there is at least one trade in the time period.
    pub high_trade_timestamp: Option<i64>,
    #[serde(rename = "LOW_TRADE_PRICE")]
    /// The lowest value of the trades in the time period. This is only available when there is at least one trade in the time period.
    pub low_trade_price: Option<f64>,
    #[serde(rename = "LOW_TRADE_TIMESTAMP")]
    /// The timestamp, in seconds, of the lowest trade in the time period. This is only available when there is at least one trade in the time period.
    pub low_trade_timestamp: Option<i64>,
    #[serde(rename = "LAST_TRADE_PRICE")]
    /// The last trade price in the time period. This is only available when there is at least one trade in the time period.
    pub last_trade_price: Option<f64>,
    #[serde(rename = "TOTAL_TRADES")]
    /// The total number of trades seen in this time period. If there were no trades in the time period, 0 will be returned.
    pub total_trades: i64,
    #[serde(rename = "TOTAL_TRADES_BUY")]
    /// The total number of BUY trades seen in this time period. If there were no trades in the time period, 0 will be returned.
    pub total_trades_buy: i64,
    #[serde(rename = "TOTAL_TRADES_SELL")]
    // The total number of SELL trades seen in this time period. If there were no trades in the time period, 0 will be returned.
    pub total_trades_sell: i64,
    #[serde(rename = "TOTAL_TRADES_UNKNOWN")]
    /// The total number of UNKNOWN trades seen in this time period. If there were no trades in the time period, 0 will be returned.
    pub total_trades_unknown: i64,
    #[serde(rename = "VOLUME")]
    /// The sum of all the trade volumes in the from asset (base symbol / coin) for the time period. If there were no trades in the time period,
    /// 0 will be returned.
    pub volume: f64,
    #[serde(rename = "QUOTE_VOLUME")]
    /// The sum of all the trade volumes in the To asset (quote/counter symbol/coin) for the time period. If there were no trades in the time period,
    /// 0 will be returned.
    pub quote_volume: f64,
    #[serde(rename = "VOLUME_BUY")]
    /// The sum of all the BUY trade volumes in the from asset (base symbol / coin) for the time period.
    pub volume_buy: f64,
    #[serde(rename = "QUOTE_VOLUME_BUY")]
    /// The sum of all the BUY trade volumes in the to asset (quote/counter symbol/coin) for the time period.
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


// Spot: Instrument Metadata


/// Spot: Instrument Metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotInstrumentMetdata {
    #[serde(rename = "METADATA_VERSION")]
    /// The version of metadata, used for version conversions/migrates.
    pub metadata_version: usize,
    #[serde(rename = "INSTRUMENT_STATUS")]
    /// The status of the instrument, we only poll / stream / connect to the ACTIVE ones, for the RETIRED / IGNORED / EXPIRED /
    /// READY_FOR_DECOMMISSIONING means we no longer query/stream data.
    pub instrument_status: String,
    #[serde(rename = "FIRST_SEEN_ON_POLLING_TS")]
    /// This is the first time instrument was seen on instrumentListSourceType POLLING.
    pub first_seen_on_pollings_ts: i64,
    #[serde(rename = "LAST_SEEN_ON_POLLING_TS")]
    /// This is the last time instrument was seen on instrumentListSourceType POLLING.
    pub last_seen_pollings_ts: i64,
    #[serde(rename = "INSTRUMENT")]
    /// The instrument ID as it is on the exchange with small modifications - we do not allow the following characters inside isntrument ids: ,/&?
    pub instrument: String,
    #[serde(rename = "INSTRUMENT_MAPPING")]
    /// The current mapping information for this instrument.
    pub instrument_mapping: SpotInstrumentMapping,
    #[serde(rename = "INSTRUMENT_EXTERNAL_DATA")]
    /// The full data we get from the polling endpoint for this specific instrument. This is a JSON stringified object with different properties per exchange.
    pub instrument_external_data: String,
    #[serde(rename = "FIRST_OB_L2_MINUTE_SNAPSHOT_TS")]
    /// Timestamp of the initial Level 2 minute snapshot.
    pub first_ob_l2_minute_snapshot_ts: Option<i64>,
}


// Spot: Markets


/// Spot: Markets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotMarkets {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    /// The status of the echange. We only poll / stream / connect to the ACTIVE ones, for the RETIRED ones we no longer query for data".
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
    pub instrument_status: InstrumentStatus,
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    /// The total number of spot trades that this exchange has processed.
    pub total_trades_spot: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    /// Boolean field denoting if we have historical minute orderbook snapshots endabled for this exchange.
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}


// Spot: Markets + Instruments


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "INSTRUMENT_STATUS")]
    /// Status of the current instrument.
    pub instrument_status: String,
    #[serde(rename = "INSTRUMENT")]
    /// The internal exchange defined instrument id.
    pub instrument: String,
    #[serde(rename = "HISTO_SHARD")]
    /// Our internal shard for historical OHLCV+ (minute/hour/day) market data. Minute data is only held in the historical database for
    /// up to 3 weeks and we ship it to blob storage afterwards. The API utilizes multiple replicas of a single shard in a round-robin manner.
    pub histo_shard: String,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    /// The CoinDesk mapped instrument ID.
    pub mapped_instrument: String,
    #[serde(rename = "INSTRUMENT_MAPPING")]
    /// The current mapping information for this instrument.
    pub instrument_mapping: SpotInstrumentMapping,
    #[serde(rename = "HAS_TRADES_SPOT")]
    /// A boolean field indicating whether the instrument has spot trades or not.
    pub has_trades_spot: bool,
    #[serde(rename = "FIRST_TRADE_SPOT_TIMESTAMP")]
    /// The first spot defi trade timestamp.
    pub first_trade_spot_timestamp: i64,
    #[serde(rename = "LAST_TRADE_SPOT_TIMESTAMP")]
    /// The last spot defi trade timestamp.
    pub last_trade_spot_timestamp: i64,
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    /// The total number of spot trades that this exchange has processed.
    pub total_trades_spot: i64,
}


/// Spot: Markets + Instruments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotMarketsInstruments {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    /// The status of the echange. We only poll / stream / connect to the ACTIVE ones, for the RETIRED ones we no longer query for data".
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
    pub instrument_status: InstrumentStatus,
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    /// The total number of spot trades that this exchange has processed.
    pub total_trades_spot: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    /// Boolean field denoting if we have historical minute orderbook snapshots endabled for this exchange.
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
    #[serde(rename = "instruments")]
    /// The list of instruments requested. It could be a selected few or all for each market.
    pub instruments: HashMap<String, Instrument>,
}