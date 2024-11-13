use std::collections::HashMap;
use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


/// The exchange to obtain data from.
pub enum CCSpotMarket {
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

impl Market for CCSpotMarket {
    /// Converts enum value to `String`.
    fn to_string(&self) -> String {
        match self {
            CCSpotMarket::AAX => String::from("aax"),
            CCSpotMarket::ABCC => String::from("abcc"),
            CCSpotMarket::ACX => String::from("acx"),
            CCSpotMarket::AIDOSMARKET => String::from("aidosmarket"),
            CCSpotMarket::ALPHAEX => String::from("alphaex"),
            CCSpotMarket::ARCHAX => String::from("archax"),
            CCSpotMarket::ASCENDEX => String::from("ascendex"),
            CCSpotMarket::ATAIX => String::from("ataix"),
            CCSpotMarket::BACKPACK => String::from("backpack"),
            CCSpotMarket::BEQUANT => String::from("bequant"),
            CCSpotMarket::BGOGO => String::from("bgogo"),
            CCSpotMarket::BIBOX365 => String::from("bibox365"),
            CCSpotMarket::BIGONE => String::from("bigone"),
            CCSpotMarket::BILAXY => String::from("bilaxy"),
            CCSpotMarket::BINANCE => String::from("binance"),
            CCSpotMarket::BINANCEAGGREGATE => String::from("binanceaggreagate"),
            CCSpotMarket::BINANCETR => String::from("binancetr"),
            CCSpotMarket::BINANCEUSA => String::from("binanceusa"),
            CCSpotMarket::BINGX => String::from("bingx"),
            CCSpotMarket::BISQ => String::from("bisq"),
            CCSpotMarket::BIT => String::from("bit"),
            CCSpotMarket::BIT2C => String::from("bit2c"),
            CCSpotMarket::BITBANK => String::from("bitbank"),
            CCSpotMarket::BITBAY => String::from("bitbay"),
            CCSpotMarket::BITBNS => String::from("bitbns"),
            CCSpotMarket::BITBUY => String::from("bitbuy"),
            CCSpotMarket::BITCI => String::from("bitci"),
            CCSpotMarket::BITEXBOOK => String::from("bitexbook"),
            CCSpotMarket::BITFEX => String::from("bitfex"),
            CCSpotMarket::BITFINEX => String::from("bitfinex"),
            CCSpotMarket::BITFLYER => String::from("bitflyer"),
            CCSpotMarket::BITFLYEREU => String::from("bitflyereu"),
            CCSpotMarket::BITFLYERFX => String::from("bitflyerfx"),
            CCSpotMarket::BITFLYERUS => String::from("bitflyerus"),
            CCSpotMarket::BITFOREX => String::from("bitforex"),
            CCSpotMarket::BITGET => String::from("bitget"),
            CCSpotMarket::BITHUMBGLOBAL => String::from("bithumbglobal"),
            CCSpotMarket::BITHUMBKOREA => String::from("bithumbkorea"),
            CCSpotMarket::BITINKA => String::from("bitinka"),
            CCSpotMarket::BITKUB => String::from("bitkub"),
            CCSpotMarket::BITMART => String::from("bitmart"),
            CCSpotMarket::BITMEX => String::from("bitmex"),
            CCSpotMarket::BITPANDA => String::from("bitpanda"),
            CCSpotMarket::BITRUE => String::from("bitrue"),
            CCSpotMarket::BITSO => String::from("bitso"),
            CCSpotMarket::BITSTAMP => String::from("bitstamp"),
            CCSpotMarket::BITTREX => String::from("bittrex"),
            CCSpotMarket::BITVAVO => String::from("bitvavo"),
            CCSpotMarket::BKEX => String::from("bkex"),
            CCSpotMarket::BLACKTURTLE => String::from("blackturtle"),
            CCSpotMarket::BLEUTRADE => String::from("bleutrade"),
            CCSpotMarket::BLOCKCHAINCOM => String::from("blockchaindotcom"),
            CCSpotMarket::BTCALPHA => String::from("btcalpha"),
            CCSpotMarket::BTCBOX => String::from("btcbox"),
            CCSpotMarket::BTCEX => String::from("btcex"),
            CCSpotMarket::BTCMARKETS => String::from("btcmarkets"),
            CCSpotMarket::BTCTURK => String::from("btcturk"),
            CCSpotMarket::BTSE => String::from("btse"),
            CCSpotMarket::BUDA => String::from("buda"),
            CCSpotMarket::BULLISH => String::from("bullish"),
            CCSpotMarket::BUYUCOIN => String::from("buyucoin"),
            CCSpotMarket::BWEXCHANGE => String::from("bwexchange"),
            CCSpotMarket::BYBIT => String::from("bybit"),
            CCSpotMarket::BYDFI => String::from("bydfi"),
            CCSpotMarket::CATEX => String::from("catex"),
            CCSpotMarket::CEXIO => String::from("cexio"),
            CCSpotMarket::COINBASE => String::from("coinbase"),
            CCSpotMarket::COINBASEINTERNATIONAL => String::from("coinbaseinternational"),
            CCSpotMarket::COINCHECK => String::from("coincheck"),
            CCSpotMarket::COINCORNER => String::from("coincorner"),
            CCSpotMarket::COINDCX => String::from("coindcx"),
            CCSpotMarket::COINDEAL => String::from("coindeal"),
            CCSpotMarket::COINEX => String::from("coinex"),
            CCSpotMarket::COINFALCON => String::from("coinfalcon"),
            CCSpotMarket::COINFIELD => String::from("coinfield"),
            CCSpotMarket::COINJAR => String::from("coinjar"),
            CCSpotMarket::COINMATE => String::from("coinmate"),
            CCSpotMarket::COINONE => String::from("coinone"),
            CCSpotMarket::COINSBIT => String::from("coinsbit"),
            CCSpotMarket::COINSPRO => String::from("coinspro"),
            CCSpotMarket::COINTIGER => String::from("cointiger"),
            CCSpotMarket::COINW => String::from("coinw"),
            CCSpotMarket::COSS => String::from("coss"),
            CCSpotMarket::CREX24 => String::from("crex24"),
            CCSpotMarket::CROSSTOWER => String::from("crosstower"),
            CCSpotMarket::CRYPTOCARBON => String::from("cryptocarbon"),
            CCSpotMarket::CRYPTODOTCOM => String::from("cryptodotcom"),
            CCSpotMarket::CRYPTOPIA => String::from("cryptopia"),
            CCSpotMarket::CRYPTSY => String::from("cryptsy"),
            CCSpotMarket::CUBE => String::from("cube"),
            CCSpotMarket::CURRENCY => String::from("currency"),
            CCSpotMarket::DCOIN => String::from("dcoin"),
            CCSpotMarket::DDEX => String::from("ddex"),
            CCSpotMarket::DECOIN => String::from("decoin"),
            CCSpotMarket::DERIBIT => String::from("deribit"),
            CCSpotMarket::DIGIFINEX => String::from("digifinex"),
            CCSpotMarket::ERISX => String::from("erisx"),
            CCSpotMarket::ETORO => String::from("etoro"),
            CCSpotMarket::EXMO => String::from("exmo"),
            CCSpotMarket::FCOIN => String::from("fcoin"),
            CCSpotMarket::FOXBIT => String::from("foxbit"),
            CCSpotMarket::FTX => String::from("ftx"),
            CCSpotMarket::FTXUS => String::from("ftxus"),
            CCSpotMarket::GARANTEX => String::from("garantex"),
            CCSpotMarket::GATEIO => String::from("gateio"),
            CCSpotMarket::GEMINI => String::from("gemini"),
            CCSpotMarket::GLOBITEX => String::from("globitex"),
            CCSpotMarket::GOPAX => String::from("gopax"),
            CCSpotMarket::GRAVIEX => String::from("graviex"),
            CCSpotMarket::HASHKEY => String::from("hashkey"),
            CCSpotMarket::HITBTC => String::from("hitbtc"),
            CCSpotMarket::HUOBIJAPAN => String::from("huobijapan"),
            CCSpotMarket::HUOBIPRO => String::from("huobipro"),
            CCSpotMarket::INDEPENDENTRESERVE => String::from("independentreserve"),
            CCSpotMarket::INDODAX => String::from("indodax"),
            CCSpotMarket::INDOEX => String::from("indoex"),
            CCSpotMarket::INX => String::from("inx"),
            CCSpotMarket::ITBIT => String::from("itbit"),
            CCSpotMarket::KORBIT => String::from("korbit"),
            CCSpotMarket::KRAKEN => String::from("kraken"),
            CCSpotMarket::KUCOIN => String::from("kucoin"),
            CCSpotMarket::KUNA => String::from("kuna"),
            CCSpotMarket::LATOKEN => String::from("latoken"),
            CCSpotMarket::LBANK => String::from("lbank"),
            CCSpotMarket::LIQNET => String::from("liqnet"),
            CCSpotMarket::LIQUID => String::from("liquid"),
            CCSpotMarket::LITEBIT => String::from("litebit"),
            CCSpotMarket::LMAX => String::from("lmax"),
            CCSpotMarket::LUNO => String::from("luno"),
            CCSpotMarket::LYKKE => String::from("lykke"),
            CCSpotMarket::MERCADOBTC => String::from("mercadobtc"),
            CCSpotMarket::MERCATOX => String::from("mercatox"),
            CCSpotMarket::MEXC => String::from("mexc"),
            CCSpotMarket::MOCK => String::from("mock"),
            CCSpotMarket::MTGOX => String::from("mtgox"),
            CCSpotMarket::NDAX => String::from("ndax"),
            CCSpotMarket::NOMINEX => String::from("nominex"),
            CCSpotMarket::OKCOIN => String::from("okcoin"),
            CCSpotMarket::OKEX => String::from("okex"),
            CCSpotMarket::ONETRADING => String::from("onetrading"),
            CCSpotMarket::OSL => String::from("osl"),
            CCSpotMarket::OSLHONGKONG => String::from("oslhongkong"),
            CCSpotMarket::P2PB2B => String::from("p2pb2b"),
            CCSpotMarket::PANCAKESWAP => String::from("pancakeswap"),
            CCSpotMarket::PARAMOUNTDAX => String::from("paramountdax"),
            CCSpotMarket::PARIBU => String::from("paribu"),
            CCSpotMarket::PHEMEX => String::from("phemex"),
            CCSpotMarket::POLONIEX => String::from("poloniex"),
            CCSpotMarket::PROBIT => String::from("probit"),
            CCSpotMarket::SAFETRADE => String::from("safetrade"),
            CCSpotMarket::SIGENPRO => String::from("sigenpro"),
            CCSpotMarket::SIMEX => String::from("simex"),
            CCSpotMarket::SWITCHEO => String::from("switcheo"),
            CCSpotMarket::THEROCKTRADING => String::from("therocktrading"),
            CCSpotMarket::TIDEFI => String::from("tidefi"),
            CCSpotMarket::TIMEX => String::from("timex"),
            CCSpotMarket::TOKENOMY => String::from("tokenomy"),
            CCSpotMarket::TRADEOGRE => String::from("tradeogre"),
            CCSpotMarket::UNISWAP => String::from("uniswap"),
            CCSpotMarket::UNOCOIN => String::from("unocoin"),
            CCSpotMarket::UPBIT => String::from("upbit"),
            CCSpotMarket::VALR => String::from("valr"),
            CCSpotMarket::VITEX => String::from("vitex"),
            CCSpotMarket::WAZIRX => String::from("waxirx"),
            CCSpotMarket::WHITEBIT => String::from("whitebit"),
            CCSpotMarket::WOO => String::from("woo"),
            CCSpotMarket::XCOEX => String::from("xcoex"),
            CCSpotMarket::XTPUB => String::from("xtpub"),
            CCSpotMarket::YELLOW => String::from("yellow"),
            CCSpotMarket::YOBIT => String::from("yobit"),
            CCSpotMarket::ZAIF => String::from("zaif"),
            CCSpotMarket::ZBDOTCOM => String::from("zbdotcom"),
            CCSpotMarket::ZBG => String::from("zbg"),
            CCSpotMarket::ZEBITEX => String::from("zebitex"),
            CCSpotMarket::ZONDA => String::from("zonda"),
        }
    }
}


/// The status of the instrument, can be one of the following: ACTIVE, IGNORED, RETIRED, EXPIRED.
pub enum CCSpotInstrumentStatus {
    ACTIVE,
    IGNORED,
    RETIRED,
    EXPIRED,
    READYFORDECOMMISSIONING,
}

impl CCSpotInstrumentStatus {
    /// Converts enum value to `String`.
    pub fn to_string(&self) -> String {
        match self {
            CCSpotInstrumentStatus::ACTIVE => String::from("ACTIVE"),
            CCSpotInstrumentStatus::IGNORED => String::from("IGNORED"),
            CCSpotInstrumentStatus::RETIRED => String::from("RETIRED"),
            CCSpotInstrumentStatus::EXPIRED => String::from("EXPIRED"),
            CCSpotInstrumentStatus::READYFORDECOMMISSIONING => String::from("READY_FOR_DECOMMISSIONING"),
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct CCInstrumentMapping {
    #[serde(rename = "MAPPED_INSTRUMENT")]
    /// The current mapping dsv for this instrument.
    pub mapped_instrument: String,
    #[serde(rename = "BASE")]
    /// The current mapping information for this instrument.
    pub base: String,
    #[serde(rename = "BASE_ID")]
    /// Represents the internal CCData ID for the base asset or coin (e.g., 1 for BTC). This ID is unique and immutable,
    /// ensuring consistent identification. Applicable only to instruments with a mapping.
    pub base_id: i32,
    #[serde(rename = "QUOTE")]
    /// The current mapping vs for this instrument.
    pub quote: String,
    #[serde(rename = "QUOTE_ID")]
    /// Represents the internal CCData ID for the quote asset or counter coin (e.g., 5 for USD). This ID is unique and immutable,
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
#[derive(Deserialize, Debug)]
pub struct CCSpotOHLCV {
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
    /// Represents the internal CCData ID for the base asset or coin (e.g., 1 for BTC). This ID is unique and immutable, ensuring consistent identification.
    /// Applicable only to instruments with a mapping.
    pub base_id: i32,
    #[serde(rename = "QUOTE_ID")]
    /// Represents the internal CCData ID for the quote asset or counter coin (e.g., 5 for USD). This ID is unique and immutable, ensuring consistent identification.
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
#[derive(Deserialize, Debug)]
pub struct CCSpotInstrumentMetdata {
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
    pub instrument_mapping: CCInstrumentMapping,
    #[serde(rename = "INSTRUMENT_EXTERNAL_DATA")]
    /// The full data we get from the polling endpoint for this specific instrument. This is a JSON stringified object with different properties per exchange.
    pub instrument_external_data: String,
    #[serde(rename = "FIRST_OB_L2_MINUTE_SNAPSHOT_TS")]
    /// Timestamp of the initial Level 2 minute snapshot.
    pub first_ob_l2_minute_snapshot_ts: Option<i64>,
}


// Spot: Markets


/// Spot: Markets
#[derive(Deserialize, Debug)]
pub struct CCSpotMarkets {
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
    pub instrument_status: CCInstrumentStatus,
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    /// The total number of spot trades that this exchange has processed.
    pub total_trades_spot: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    /// Boolean field denoting if we have historical minute orderbook snapshots endabled for this exchange.
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}


// Spot: Markets + Instruments


#[derive(Deserialize, Debug)]
pub struct CCInstrument {
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
    /// The ccdata mapped instrument ID.
    pub mapped_instrument: String,
    #[serde(rename = "INSTRUMENT_MAPPING")]
    /// The current mapping information for this instrument.
    pub instrument_mapping: CCInstrumentMapping,
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
#[derive(Deserialize, Debug)]
pub struct CCSpotMarketsInstruments {
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
    pub instrument_status: CCInstrumentStatus,
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    /// The total number of spot trades that this exchange has processed.
    pub total_trades_spot: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    /// Boolean field denoting if we have historical minute orderbook snapshots endabled for this exchange.
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
    #[serde(rename = "instruments")]
    /// The list of instruments requested. It could be a selected few or all for each market.
    pub instruments: HashMap<String, CCInstrument>,
}