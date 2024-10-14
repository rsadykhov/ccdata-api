use std::collections::HashMap;
use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


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


pub enum CCSpotInstrumentStatus {
    ACTIVE,
    IGNORED,
    RETIRED,
    EXPIRED,
    READYFORDECOMMISSIONING,
}

impl CCSpotInstrumentStatus {
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
    pub mapped_instrument: String,
    #[serde(rename = "BASE")]
    pub base: String,
    #[serde(rename = "BASE_ID")]
    pub base_id: i32,
    #[serde(rename = "QUOTE")]
    pub quote: String,
    #[serde(rename = "QUOTE_ID")]
    pub quote_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    pub transform_function: String,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
}


// Spot: Historical OHLCV+


/// Spot: Historical OHLCV+
#[derive(Deserialize, Debug)]
pub struct CCSpotOHLCV {
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
    #[serde(rename = "BASE")]
    pub base: String,
    #[serde(rename = "QUOTE")]
    pub quote: String,
    #[serde(rename = "BASE_ID")]
    pub base_id: i32,
    #[serde(rename = "QUOTE_ID")]
    pub quote_id: i32,
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
    pub first_trade_timestamp: Option<i64>,
    #[serde(rename = "LAST_TRADE_TIMESTAMP")]
    pub last_trade_timestamp: Option<i64>,
    #[serde(rename = "FIRST_TRADE_PRICE")]
    pub first_trade_price: Option<f64>,
    #[serde(rename = "HIGH_TRADE_PRICE")]
    pub high_trade_price: Option<f64>,
    #[serde(rename = "HIGH_TRADE_TIMESTAMP")]
    pub high_trade_timestamp: Option<i64>,
    #[serde(rename = "LOW_TRADE_PRICE")]
    pub low_trade_price: Option<f64>,
    #[serde(rename = "LOW_TRADE_TIMESTAMP")]
    pub low_trade_timestamp: Option<i64>,
    #[serde(rename = "LAST_TRADE_PRICE")]
    pub last_trade_price: Option<f64>,
    #[serde(rename = "TOTAL_TRADES")]
    pub total_trades: i64,
    #[serde(rename = "TOTAL_TRADES_BUY")]
    pub total_trades_buy: i64,
    #[serde(rename = "TOTAL_TRADES_SELL")]
    pub total_trades_sell: i64,
    #[serde(rename = "TOTAL_TRADES_UNKNOWN")]
    pub total_trades_unknown: i64,
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


// Spot: Instrument Metadata


/// Spot: Instrument Metadata
#[derive(Deserialize, Debug)]
pub struct CCSpotInstrumentMetdata {
    #[serde(rename = "METADATA_VERSION")]
    pub metadata_version: usize,
    #[serde(rename = "INSTRUMENT_STATUS")]
    pub instrument_status: String,
    #[serde(rename = "FIRST_SEEN_ON_POLLING_TS")]
    pub first_seen_on_pollings_ts: i64,
    #[serde(rename = "LAST_SEEN_ON_POLLING_TS")]
    pub last_seen_pollings_ts: i64,
    #[serde(rename = "INSTRUMENT")]
    pub instrument: String,
    #[serde(rename = "INSTRUMENT_MAPPING")]
    pub instrument_mapping: CCInstrumentMapping,
    #[serde(rename = "INSTRUMENT_EXTERNAL_DATA")]
    pub instrument_external_data: String,
    #[serde(rename = "FIRST_OB_L2_MINUTE_SNAPSHOT_TS")]
    pub first_ob_l2_minute_snapshot_ts: Option<i64>,
}


// Spot: Markets


/// Spot: Markets
#[derive(Deserialize, Debug)]
pub struct CCSpotMarkets {
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
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    pub total_trades_spot: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}


// Spot: Markets + Instruments


#[derive(Deserialize, Debug)]
pub struct CCInstrument {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "INSTRUMENT_STATUS")]
    pub instrument_status: String,
    #[serde(rename = "INSTRUMENT")]
    pub instrument: String,
    #[serde(rename = "HISTO_SHARD")]
    pub histo_shard: String,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    pub mapped_instrument: String,
    #[serde(rename = "INSTRUMENT_MAPPING")]
    pub instrument_mapping: CCInstrumentMapping,
    #[serde(rename = "HAS_TRADES_SPOT")]
    pub has_trades_spot: bool,
    #[serde(rename = "FIRST_TRADE_SPOT_TIMESTAMP")]
    pub first_trade_spot_timestamp: i64,
    #[serde(rename = "LAST_TRADE_SPOT_TIMESTAMP")]
    pub last_trade_spot_timestamp: i64,
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    pub total_trades_spot: i64,
}


/// Spot: Markets + Instruments
#[derive(Deserialize, Debug)]
pub struct CCSpotMarketsInstruments {
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
    #[serde(rename = "TOTAL_TRADES_SPOT")]
    pub total_trades_spot: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
    #[serde(rename = "instruments")]
    pub instruments: HashMap<String, CCInstrument>,
}